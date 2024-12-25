test:
	pytest
	cargo clippy
	cargo test --workspace --all-features
.PHONY: test

bench:
	pytest --codspeed tests/bench.py
	# pytest tests/test_simple.py --memray
	python tests/bench_mem.py --empty
	python tests/bench_mem.py
.PHONY: bench

pull:
	git fetch ruff-repo main
	git rm -r --cached crates/ruff_python_ast
	git checkout ruff-repo/main -- crates/ruff_python_ast
.PHONY: pull
