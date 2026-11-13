# Testing rules

Test placement, local evidence and test names. Loaded in every session.

1. **Tests stay with the crate.** Every test lives in `tests/`, outside `src/`, as one test target (Rust rules, item 1). What only the tests use, fixtures, goldens and fakes, lives with them.
2. **Local evidence stays local.** `.audits/` is ignored local experiment and diagnostic output. Its files stay out of commits and pull request links; the pull request summarizes the validation.
3. **Test names say the behavior.** A test function's name states what holds, in words (`a_listed_id_excuses_one_extra_line`), never the name of the function under test.
