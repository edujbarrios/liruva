# Architecture decisions

These are starting decisions, not promises that cannot evolve.

## ADR-0001 — Entirely open-source core

**Decision:** LIRUVA original source code is GPL-3.0-or-later. Avoid proprietary components and mandatory paid services. All bundled assets and dependencies require redistribution-compatible licenses.

**Why:** Freedom to use, study, modify, and share is a founding requirement.

## ADR-0002 — Rust desktop shell

**Decision:** Start with a small Rust workspace and native eframe/egui app. eframe 0.36.2 is distributed under MIT OR Apache-2.0.

**Why:** A simple native welcome screen can establish an actual running application without building a custom UI toolkit.

**Tradeoff:** Cross-platform native build dependencies must be documented and verified in CI over time.

## ADR-0003 — No audio or agent placeholders

**Decision:** Do not display fake play, save, or export controls before those operations exist. Do not include an optional AI service in the MVP.

**Why:** A trustworthy, beginner-friendly tool is more useful than an elaborate mockup.

## ADR-0004 — Add modules only when needed

**Decision:** Begin with a single binary package; separate audio, timing, serialization, and commands when they become real.

**Why:** Avoid premature abstraction while planning safe real-time separation.
