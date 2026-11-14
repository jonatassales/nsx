# 0027: nsx starts Docker when it is stopped

**Spec:** None. The owner's direction of 2026-11-14: when Docker is installed but not running, `nsx agent run` starts it itself and waits, without a line. ADR 001 records it among the owner's exceptions.

**What to build:** `nsx agent run` on a machine whose Docker is installed and stopped starts it, waits until it answers, and goes on. On macOS it starts Docker Desktop, or the Colima nsx installed. On Linux, where starting the daemon needs root, it stops with one sentence that gives the command. Nothing is printed on the way.

**Blocked by:** 0026.

**Status:** completed

**Alignment:** build. Canonical rule (ADR 001): what is done beyond the simulation is removed; what the simulation shows and the code lacks becomes a new ticket that builds it. The simulation shows no line about Docker; the owner's exception of 2026-11-14 allows the silent start.

- [x] Before the cluster is first reached, nsx asks Docker whether it answers (`docker info`).
- [x] On macOS, a Docker that does not answer is started, Docker Desktop with `open -g -a Docker`; nsx then asks again every two seconds, for at most three minutes. Ticket 0028 adds nsx's own Colima.
- [x] A Docker that still does not answer stops the run under the stop rule with one sentence, and is not started or waited for again by the run's cleanup.
- [x] On Linux, a Docker that does not answer stops the run with a sentence that gives `sudo systemctl start docker`.
- [x] No Docker at all stops the run with a sentence that names Docker Desktop or Colima on macOS, until ticket 0028 lets `nsx init` install one, and the official install command on Linux.
- [x] The run prints the simulation's lines and no other.
- [x] The repository's checks pass.

## Dev part

**Override:** on 2026-11-14 no part qualifies: the ticket starts a program and waits for another to answer, with no problem of the policy's categories (`core-domain`, `hard-part`, `new-stack`).
