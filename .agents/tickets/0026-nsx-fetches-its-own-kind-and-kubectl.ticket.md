# 0026: nsx fetches its own kind and kubectl

**Spec:** None. The owner's direction of 2026-11-14, after a QA run: a team installs nothing by hand; nsx downloads its own pinned copies of kind and kubectl into the project's private state. ADR 001 records it among the owner's exceptions.

**What to build:** `nsx agent run` brings its local cluster up on a machine that has neither kind nor kubectl installed. Before it first reaches the cluster it fetches nsx's pinned kind and kubectl for the machine, verifies each against its published SHA-256, keeps them under `.langdrift/agent/state/tools/`, and starts every cluster tool with that folder first on its `PATH`. A copy already there with the right checksum is not fetched again. Nothing is printed.

**Blocked by:** None (can start immediately).

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it. The simulation's run starts the runtime with no line about tools (`console › runtime    5 Deployments ready`), so fetching prints none; the owner's exception of 2026-11-14 allows it.

- [x] The pins are kind v0.33.0 and kubectl v1.37.2, each with its official SHA-256 for macOS and Linux on arm64 and x64.
- [x] A missing tool is downloaded from its official release URL, checked against its SHA-256, and written executable under `.langdrift/agent/state/tools/`, which Git ignores.
- [x] A tool already there whose checksum matches is kept and nothing is downloaded; one whose checksum differs is replaced.
- [x] A download that fails, or bytes that do not match the pin, stop the run under the stop rule with one sentence naming the tool; nothing unverified is ever written executable.
- [x] kind, kubectl and Docker are started with the tools folder first on `PATH`.
- [x] The run prints the simulation's lines and no other.
- [x] `cargo fmt --check`, `cargo clippy --all-targets --locked -- -D warnings`, `cargo test --locked`, `cargo deny check`, `vtec/vtec.sh quality check` and `vtec/vtec.sh git lint-history origin/main HEAD` pass.

## Dev part

**Category:** `hard-part`: fetching a binary so that nothing unverified ever becomes executable, and nothing is fetched twice.

**Paths:** `src/local/tool/fetch.rs`.

**Boundary:** `Tool::fetch(&self, folder: &Path, downloads: &dyn Download) -> Result<(), Error>`, which the cluster calls before it is first reached.

**Goal:** keep a verified copy of one pinned tool in a folder.

- [x] A matching copy is kept without a download.
- [x] Downloaded bytes whose SHA-256 differs from the pin are refused and nothing is written.

**Override:** on 2026-11-14 the owner asked the agent to write the whole change, dev parts included: "Yes, write it all".
