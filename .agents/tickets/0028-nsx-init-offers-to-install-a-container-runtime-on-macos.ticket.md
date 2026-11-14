# 0028: nsx init offers to install a container runtime on macOS

**Spec:** None. The owner's direction of 2026-11-14: a missing Docker is not a line in the README. `nsx init` asks once and installs an open-source runtime on macOS; on Linux, where installing needs root, nsx gives the official command. ADR 001 records it among the owner's exceptions.

**What to build:** A person runs `nsx init` on a Mac with no container runtime. nsx asks once whether to install one; on yes, or with `NSX_INSTALL_RUNTIME=yes` in the environment, it downloads Colima, Lima and the Docker CLI, pinned and checksum-verified, into `.langdrift/agent/state/tools/`, with no Homebrew and no password. `nsx agent run` then starts that Colima without changing the person's Docker context. The README's getting-started steps no longer list Docker, kind or kubectl.

**Blocked by:** 0027.

**Status:** ready-for-agent

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it. The simulation does not show `nsx init`; the owner's exception of 2026-11-14 allows its one question.

- [ ] `nsx init` on macOS, with no `docker` on `PATH` and none in the tools folder, asks one question on the terminal and reads one line; anything but yes installs nothing.
- [ ] `NSX_INSTALL_RUNTIME=yes` answers the question without the terminal; with no terminal and no variable, nothing is asked and nothing is installed.
- [ ] On yes, Colima, Lima and the Docker CLI are downloaded at their pinned versions, each verified against its SHA-256, and unpacked under the tools folder.
- [ ] `nsx agent run` starts nsx's Colima with the person's Docker context left as it was, and gives the cluster tools Colima's socket.
- [ ] On Linux, and on a Mac that already has Docker, `nsx init` asks nothing and installs nothing.
- [ ] The README's getting started lists neither Docker, kind nor kubectl.
- [ ] The repository's checks pass.

## Dev part

**Category:** `new-stack`: first contact with Colima and Lima, their layout on disk and how a Docker client reaches their socket.

**Paths:** `src/local/colima/start.rs`.

**Boundary:** `Colima::start(&self, programs: &dyn Runner) -> Result<(), Error>`, which the cluster calls when Docker does not answer.

**Goal:** start nsx's Colima so that Docker answers on its socket, without changing the person's Docker context.

- [ ] Colima is started with its own home under the tools folder's state and never activates a Docker context.

**Override:** on 2026-11-14 the owner asked the agent to write the whole change, dev parts included: "Yes, write it all".
