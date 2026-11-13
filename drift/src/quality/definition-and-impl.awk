# Prints each Rust file that declares a struct, an enum or a trait and also
# holds an impl block (.claude/rules/14-rust.rule.md, item 2).
FNR == 1 { defines = 0; implements = 0 }
/^(pub(\([a-z]+\))? )?(struct|enum|trait) / { defines = 1 }
/^impl[ <]/ { implements = 1 }
defines && implements && !(FILENAME in printed) { printed[FILENAME] = 1; print FILENAME }
