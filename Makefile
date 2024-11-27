test:
	cargo test --workspace --all-features
	pytest
.PHONY: test

bench:
	pytest --benchmark-autosave tests/bench.py
	pytest tests/test_simple.py --memray
	python tests/bench_mem.py --empty
	python tests/bench_mem.py
.PHONY: bench