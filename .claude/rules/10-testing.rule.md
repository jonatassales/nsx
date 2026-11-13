# Testing rules

Test placement, local evidence and test names. Loaded in every session.

1. **Tests stay with the crate.** Every test lives in `tests/`, outside `src/`, as one test target (Rust rules, item 1). What only the tests use, fixtures, goldens and fakes, lives with them.
2. **The goldens are the oracle.** `tests/simulation/golden/` holds the simulation's lines. A golden changes only when the simulation does (ADR 001), never to make a test pass.
3. **Local evidence stays local.** `.audits/` is ignored local experiment and diagnostic output. Its files stay out of commits and pull request links; the pull request summarizes the validation.
4. **Test names say the behavior.** A test function's name states what holds, in words (`a_listed_id_excuses_one_extra_line`), never the name of the function under test.
