pull:
	git fetch ruff-repo --tags
	git format-patch origin/v1.0.0..origin/v1.1.0 --output-directory=patches -- crates/ruff_python_parser1
	git checkout ruff-repo/main -- crates/ruff_python_parser1
.PHONY: pull

.PHONY: hyperfine-bench
hyperfine-bench:
	hyperfine --warmup 1 --export-markdown bench.md 'uv run python tests/scripts/rd.py' 'uv run python tests/scripts/xply.py'

.PHONY: bench
bench:
	uv run --no-sync pytest tests/bench.py --codspeed -vv
