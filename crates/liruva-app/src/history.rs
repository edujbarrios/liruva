// SPDX-License-Identifier: GPL-3.0-or-later
//! Bounded, UI-thread-only edit history. No allocations or locks in the audio callback.

use crate::audio::AudioControls;
use crate::transport::{STEPS_PER_BAR, TRACK_COUNT};

const HISTORY_LIMIT: usize = 64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EditorState {
    pub name: String,
    pub bpm: u32,
    pub drums: [u16; TRACK_COUNT],
    pub melody: [Option<u8>; STEPS_PER_BAR],
}

impl EditorState {
    pub fn capture(name: &str, controls: &AudioControls) -> Self {
        Self {
            name: name.to_owned(),
            bpm: controls.bpm(),
            drums: controls.pattern_masks(),
            melody: controls.melody_notes(),
        }
    }

    pub fn apply(&self, controls: &AudioControls) {
        // Atomic updates are safe while playback is running. The audio callback
        // reads each value directly and never waits for the editor.
        controls.set_bpm(self.bpm);
        controls.set_pattern_masks(self.drums);
        controls.set_melody_notes(self.melody);
    }
}

#[derive(Default)]
pub struct EditHistory {
    undo: Vec<EditorState>,
    redo: Vec<EditorState>,
}

impl EditHistory {
    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    pub fn reset(&mut self) {
        self.undo.clear();
        self.redo.clear();
    }

    /// Only real changes enter history. Editing after an undo discards redo.
    pub fn record(&mut self, before: EditorState, after: &EditorState) {
        if before == *after {
            return;
        }
        self.redo.clear();
        if self.undo.len() == HISTORY_LIMIT {
            self.undo.remove(0);
        }
        self.undo.push(before);
    }

    pub fn undo(&mut self, current: EditorState) -> Option<EditorState> {
        let previous = self.undo.pop()?;
        self.redo.push(current);
        Some(previous)
    }

    pub fn redo(&mut self, current: EditorState) -> Option<EditorState> {
        let next = self.redo.pop()?;
        if self.undo.len() == HISTORY_LIMIT {
            self.undo.remove(0);
        }
        self.undo.push(current);
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::{EditHistory, EditorState, HISTORY_LIMIT};
    use crate::audio::AudioControls;

    #[test]
    fn editing_then_undo_and_redo_restores_all_musical_values() {
        let controls = AudioControls::default();
        let mut history = EditHistory::default();
        let original = EditorState::capture("First Beat", &controls);

        controls.toggle_step(0, 3);
        controls.set_melody_note(7, Some(12));
        controls.set_bpm(155);
        let updated = EditorState::capture("Changed Beat", &controls);
        history.record(original.clone(), &updated);

        let previous = history.undo(updated.clone()).expect("undo must exist");
        previous.apply(&controls);
        assert_eq!(EditorState::capture(&previous.name, &controls), original);
        assert!(history.can_redo());

        let next = history.redo(previous).expect("redo must exist");
        next.apply(&controls);
        assert_eq!(EditorState::capture(&next.name, &controls), updated);
    }

    #[test]
    fn unchanged_edits_do_not_create_history_entries() {
        let controls = AudioControls::default();
        let state = EditorState::capture("Unchanged", &controls);
        let mut history = EditHistory::default();
        history.record(state.clone(), &state);
        assert!(!history.can_undo());
    }

    #[test]
    fn a_new_edit_after_undo_invalidates_redo() {
        let controls = AudioControls::default();
        let mut history = EditHistory::default();
        let first = EditorState::capture("First", &controls);
        controls.set_bpm(130);
        let second = EditorState::capture("Second", &controls);
        history.record(first.clone(), &second);

        let previous = history.undo(second).expect("undo");
        history.record(previous.clone(), &EditorState {
            name: "Another edit".to_owned(),
            ..previous
        });
        assert!(!history.can_redo());
    }

    #[test]
    fn history_never_exceeds_64_entries() {
        let controls = AudioControls::default();
        let mut history = EditHistory::default();
        for index in 0..(HISTORY_LIMIT + 12) {
            let before = EditorState::capture(&format!("Before {index}"), &controls);
            let after = EditorState::capture(&format!("After {index}"), &controls);
            history.record(before, &after);
        }
        assert_eq!(history.undo.len(), HISTORY_LIMIT);
    }

    #[test]
    fn reset_clears_both_stacks() {
        let controls = AudioControls::default();
        let mut history = EditHistory::default();
        let original = EditorState::capture("A", &controls);
        let changed = EditorState::capture("B", &controls);
        history.record(original, &changed);
        let _ = history.undo(changed);
        history.reset();
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn returning_to_saved_snapshot_is_detectable() {
        let controls = AudioControls::default();
        let saved = EditorState::capture("Untitled Beat", &controls);
        controls.set_melody_note(0, Some(7));
        assert_ne!(EditorState::capture("Untitled Beat", &controls), saved);
        controls.set_melody_note(0, None);
        assert_eq!(EditorState::capture("Untitled Beat", &controls), saved);
    }
}
