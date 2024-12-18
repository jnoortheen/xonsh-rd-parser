import pytest


@pytest.mark.benchmark(group="parse_string")
def test_parse_string(benchmark, parse_string):
    @benchmark
    def main():
        src_txt = "print(1)"
        return parse_string(src_txt)


@pytest.mark.benchmark(group="parse_file")
def test_parse_file(benchmark, parse_file):
    @benchmark
    def main():
        from pathlib import Path

        path = Path(__file__).parent / "bench.py"
        return parse_file(str(path))
