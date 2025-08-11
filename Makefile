pull:
	git fetch ruff-repo --tags
	git format-patch origin/v1.0.0..origin/v1.1.0 --output-directory=patches -- crates/ruff_python_parser1
	git checkout ruff-repo/main -- crates/ruff_python_parser1
.PHONY: pull
