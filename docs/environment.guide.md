# nsx environment

The variables nsx reads, by key.

| Key | Used by |
| --- | --- |
| `LANGDRIFT_SECRET`, `MCP_CONSOLE_URL` | `nsx init`, to record the project's connection to the LangDrift console in a private file Git does not track |
| `PATH`, `HOME`, `TMPDIR`, `LANG`, `LC_ALL`, `LC_CTYPE`, `LC_MESSAGES` | every program nsx starts |
| `SSH_AUTH_SOCK`, `GH_TOKEN`, `GITHUB_TOKEN`, `GH_HOST`, `XDG_CONFIG_HOME` | `git` and `gh`, for the delivery |
| `DOCKER_HOST`, `XDG_CONFIG_HOME`, `XDG_RUNTIME_DIR` | `kind`, `kubectl` and Docker, for the local runtime |
| the names in `passEnv` of `driver.json` | your coding agent |

nsx gives the coding agent its own: `LANGDRIFT_PROJECT_ROOT`, `LANGDRIFT_RUN`, `LANGDRIFT_TICKET`, `LANGDRIFT_STAGE`, `LANGDRIFT_SKILL`, `LANGDRIFT_MODEL`, `LANGDRIFT_EFFORT` and `LANGDRIFT_REPORT`. No other variable reaches a program nsx starts.
