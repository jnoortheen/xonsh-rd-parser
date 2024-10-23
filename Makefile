test:
	cargo test
	pytest
.PHONY: test

bench:
	pytest tests/test_bench.py
	pytest tests/test_simple.py --memray
	python tests/bench_mem.py --empty
	python tests/bench_mem.py
.PHONY: bench