test:
	pytest
	cargo clippy
	cargo test --workspace --all-features
.PHONY: test

bench:
	pytest --benchmark-autosave tests/bench.py
	# pytest tests/test_simple.py --memray
	python tests/bench_mem.py --empty
	python tests/bench_mem.py
.PHONY: bench