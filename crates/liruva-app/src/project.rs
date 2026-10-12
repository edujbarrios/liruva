// SPDX-License-Identifier: GPL-3.0-or-later
//! Versioned, human-readable, offline project files.
//! File dialogs and disk operations run on the UI thread, never in the audio callback.

use std::fs::{self, File, OpenOptions};
use std::io::{ErrorKind, Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::audio::AudioControls;
use crate::melody::MELODY_PITCH_COUNT;
use crate::transport::{STEPS_PER_BAR, TRACK_COUNT};

pub const FILE_EXTENSION: &str = "liruva";
const FILE_FORMAT: &str = "liruva-project";
const FORMAT_VERSION: u32 = 2;
const MAX_PROJECT_BYTES: u64 = 64 * 1024;
const MAX_NAME_LENGTH: usize = 80;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub format: String,
    pub format_version: u32,
    pub name: String,
    pub bpm: u32,
    pub tracks: [u16; TRACK_COUNT],
    pub melody: [Option<u8>; STEPS_PER_BAR],
}

#[derive(Deserialize)]
struct ProjectHeader {
    format: String,
    format_version: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LegacyProject {
    format: String,
    format_version: u32,
    name: String,
    bpm: u32,
    tracks: [u16; TRACK_COUNT],
}

impl Project {
    pub fn from_controls(name: &str, controls: &AudioControls) -> Self {
        Self {
            format: FILE_FORMAT.to_owned(),
            format_version: FORMAT_VERSION,
            name: name.to_owned(),
            bpm: controls.bpm(),
            tracks: controls.pattern_masks(),
            melody: controls.melody_notes(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.format != FILE_FORMAT {
            return Err("This is not a LIRUVA project file.".to_owned());
        }
        if self.format_version != FORMAT_VERSION {
            return Err(format!(
                "Unsupported LIRUVA project version: {}. This release supports version 1.",
                self.format_version
            ));
        }
        if self.name.trim().is_empty()
            || self.name.chars().count() > MAX_NAME_LENGTH
            || self.name.chars().any(char::is_control)
        {
            return Err("Project name must contain 1–80 printable characters.".to_owned());
        }
        if self.melody.iter().flatten().any(|note| usize::from(*note) >= MELODY_PITCH_COUNT) {
            return Err("A melody note is outside the supported C4–B5 range.".to_owned());
        }
        if !(40..=240).contains(&self.bpm) {
            return Err("Project tempo must be between 40 and 240 BPM.".to_owned());
        }
        Ok(())
    }

    /// Apply only an already-validated project. Stopping transport before
    /// replacing masks avoids a mixture of old and new hits during playback.
    pub fn apply(&self, controls: &AudioControls) {
        controls.set_playing(false);
        controls.set_bpm(self.bpm);
        controls.set_pattern_masks(self.tracks);
        controls.set_melody_notes(self.melody);
    }
}

pub fn normalize_save_path(path: &Path) -> std::path::PathBuf {
    let has_extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case(FILE_EXTENSION));

    if has_extension {
        path.to_path_buf()
    } else {
        let mut filename = path.as_os_str().to_os_string();
        filename.push(".liruva");
        filename.into()
    }
}

/// Read at most 64 KiB, parse and validate everything before returning.
/// A failed load does not modify any current musical state.
pub fn load_from_path(path: &Path) -> Result<Project, String> {
    let file = File::open(path).map_err(|error| format!("Cannot open project: {error}"))?;
    let mut data = Vec::new();
    file.take(MAX_PROJECT_BYTES + 1)
        .read_to_end(&mut data)
        .map_err(|error| format!("Cannot read project: {error}"))?;
    if data.len() as u64 > MAX_PROJECT_BYTES {
        return Err("Project file is larger than the supported 64 KiB limit.".to_owned());
    }

    let header: ProjectHeader =
        serde_json::from_slice(&data).map_err(|error| format!("Invalid project JSON: {error}"))?;
    if header.format != FILE_FORMAT {
        return Err("This is not a LIRUVA project file.".to_owned());
    }
    let project = match header.format_version {
        1 => {
            let legacy: LegacyProject = serde_json::from_slice(&data)
                .map_err(|error| format!("Invalid legacy project JSON: {error}"))?;
            Project {
                format: legacy.format,
                format_version: FORMAT_VERSION,
                name: legacy.name,
                bpm: legacy.bpm,
                tracks: legacy.tracks,
                melody: [None; STEPS_PER_BAR],
            }
        }
        FORMAT_VERSION => serde_json::from_slice(&data)
            .map_err(|error| format!("Invalid project JSON: {error}"))?,
        version => {
            return Err(format!("Unsupported LIRUVA project version: {version}."));
        }
    };
    project.validate()?;
    Ok(project)
}

/// Write to a new temporary file next to the destination before renaming it.
/// A failed write leaves the existing destination intact and cleans up the temp file.
pub fn save_to_path(path: &Path, project: &Project) -> Result<(), String> {
    project.validate()?;
    let mut data = serde_json::to_vec_pretty(project)
        .map_err(|error| format!("Cannot serialize project: {error}"))?;
    data.push(b'\n');

    let filename = path
        .file_name()
        .ok_or_else(|| "Choose a valid project filename.".to_owned())?
        .to_string_lossy();
    let parent = path.parent().unwrap_or_else(|| Path::new("."));

    for attempt in 0..16 {
        let temp_path = parent.join(format!(
            ".{filename}.{}.{}.tmp",
            std::process::id(),
            attempt
        ));
        let temp = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp_path);
        let mut file = match temp {
            Ok(file) => file,
            Err(error) if error.kind() == ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(format!("Cannot create temporary project file: {error}")),
        };

        let result = file
            .write_all(&data)
            .and_then(|()| file.sync_all())
            .and_then(|()| {
                drop(file);
                fs::rename(&temp_path, path)
            });
        if result.is_err() {
            let _ = fs::remove_file(&temp_path);
        }
        return result.map_err(|error| format!("Cannot save project: {error}"));
    }

    Err("Cannot create a temporary file. Try saving to another location.".to_owned())
}

#[cfg(test)]
mod tests {
    use super::{Project, load_from_path, normalize_save_path, save_to_path};
    use crate::audio::AudioControls;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_path(label: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System clock must be after the Unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "liruva-{label}-{}-{nonce}.liruva",
            std::process::id()
        ))
    }

    #[test]
    fn saving_and_reopening_restores_tempo_and_every_track() {
        let controls = AudioControls::default();
        controls.set_bpm(147);
        controls.toggle_step(1, 7);
        controls.toggle_step(3, 15);
        controls.set_melody_note(0, Some(9));
        controls.set_melody_note(12, Some(23));
        let expected = Project::from_controls("My First Beat", &controls);
        let path = test_path("roundtrip");

        save_to_path(&path, &expected).expect("save must succeed");
        let decoded = load_from_path(&path).expect("load must succeed");
        assert_eq!(decoded, expected);

        let fresh = AudioControls::default();
        decoded.apply(&fresh);
        assert_eq!(fresh.bpm(), 147);
        assert_eq!(fresh.pattern_masks(), controls.pattern_masks());
        assert_eq!(fresh.melody_notes(), controls.melody_notes());
        fs::remove_file(path).expect("temporary project must be removable");
    }

    #[test]
    fn invalid_file_does_not_replace_current_pattern() {
        let controls = AudioControls::default();
        let before = controls.pattern_masks();
        let path = test_path("broken");
        fs::write(&path, b"not a JSON project").expect("temporary file");
        assert!(load_from_path(&path).is_err());
        assert_eq!(controls.pattern_masks(), before);
        fs::remove_file(path).expect("temporary file cleanup");
    }

    #[test]
    fn unknown_versions_and_invalid_tempos_are_rejected() {
        let controls = AudioControls::default();
        let mut project = Project::from_controls("Test Pattern", &controls);
        project.format_version = 2;
        assert!(project.validate().is_err());
        project.format_version = 2;
        project.bpm = 999;
        assert!(project.validate().is_err());
        project.bpm = 120;
        project.name = "  ".to_owned();
        assert!(project.validate().is_err());
    }

    #[test]
    fn old_v1_drum_projects_migrate_to_v2_with_no_melody() {
        let path = test_path("v1-migration");
        let original = br#"{
            "format": "liruva-project",
            "format_version": 1,
            "name": "My Old Beat",
            "bpm": 95,
            "tracks": [4369, 4112, 21845, 4096]
        }"#;
        fs::write(&path, original).expect("write legacy fixture");
        let migrated = load_from_path(&path).expect("version 1 must open");
        assert_eq!(migrated.format_version, 2);
        assert_eq!(migrated.bpm, 95);
        assert_eq!(migrated.tracks[0], 4369);
        assert_eq!(migrated.melody, [None; STEPS_PER_BAR]);
        fs::remove_file(path).expect("cleanup legacy fixture");
    }

    #[test]
    fn out_of_range_melody_is_rejected_before_loading() {
        let path = test_path("invalid-melody");
        let controls = AudioControls::default();
        let mut invalid = Project::from_controls("A Beat", &controls);
        invalid.melody[1] = Some(24);
        fs::write(&path, serde_json::to_vec(&invalid).expect("encode fixture"))
            .expect("write fixture");
        assert!(load_from_path(&path).is_err());
        fs::remove_file(path).expect("cleanup invalid fixture");
    }

    #[test]
    fn oversized_file_is_refused() {
        let path = test_path("oversize");
        fs::write(&path, vec![b'a'; 65_537]).expect("temporary file");
        assert!(load_from_path(&path).is_err());
        fs::remove_file(path).expect("temporary file cleanup");
    }

    #[test]
    fn saving_again_replaces_an_existing_project_file() {
        let path = test_path("overwrite");
        let controls = AudioControls::default();
        let original = Project::from_controls("First Beat", &controls);
        save_to_path(&path, &original).expect("first save succeeds");

        controls.clear_pattern();
        controls.set_bpm(175);
        let updated = Project::from_controls("Second Beat", &controls);
        save_to_path(&path, &updated).expect("second save replaces the file");
        assert_eq!(load_from_path(&path).expect("open updated file"), updated);
        fs::remove_file(path).expect("temporary project cleanup");
    }

    #[test]
    fn unsupported_projects_are_not_applied_to_the_audio_controls() {
        let path = test_path("unsupported");
        let controls = AudioControls::default();
        controls.toggle_step(2, 7);
        controls.set_bpm(133);
        let before = Project::from_controls("Untitled Beat", &controls);
        let mut unsupported = before.clone();
        unsupported.format_version = 99;
        let data = serde_json::to_vec(&unsupported).expect("serialize invalid fixture");
        fs::write(&path, data).expect("write invalid fixture");
        assert!(load_from_path(&path).is_err());
        assert_eq!(Project::from_controls("Untitled Beat", &controls), before);
        fs::remove_file(path).expect("temporary project cleanup");
    }

    #[test]
    fn extension_is_added_only_when_missing() {
        assert_eq!(
            normalize_save_path(std::path::Path::new("my-beat")),
            std::path::PathBuf::from("my-beat.liruva")
        );
        assert_eq!(
            normalize_save_path(std::path::Path::new("my-beat.liruva")),
            std::path::PathBuf::from("my-beat.liruva")
        );
    }
}
