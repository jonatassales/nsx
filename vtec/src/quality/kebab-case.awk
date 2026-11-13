# Prints, once, each tracked path whose last segment breaks the naming rule
# (.claude/rules/01-filesystem.rule.md): kebab-case outside Rust code; Rust
# modules and the folders that hold them in snake_case, as the toolchain
# requires; conventional names pass.
{
	path = ""
	rust = ($1 == "src" || $1 == "tests")
	for (part = 1; part <= NF; part++) {
		name = $part
		path = path (part > 1 ? "/" : "") name
		if (name ~ /^(README|AGENTS|CLAUDE|SKILL|SECURITY|CONTRIBUTING|CHANGELOG)(\.[a-z-]+)?\.md$/) continue
		if (name ~ /^(LICENSE|NOTICE|Cargo\.(toml|lock))$/) continue
		sub(/^\./, "", name)
		if (rust && name ~ /^[a-z0-9]+(_[a-z0-9]+)*(\.rs)?$/) continue
		if (name ~ /^([a-z0-9]+(-[a-z0-9]+)*)?(\.[a-z0-9]+(-[a-z0-9]+)*)*$/) continue
		if (path in seen) continue
		seen[path] = 1
		print path
	}
}
