.PHONY: hyperfine-bench
hyperfine-bench:
# 	maturin develop
	mkdir -p .benchmarks
	hyperfine --warmup 1 'uv run python tests/scripts/rd.py' 'uv run python tests/scripts/xply.py' --export-markdown .benchmarks/bench-$(shell date +%Y%m%d%H%M%S).md
