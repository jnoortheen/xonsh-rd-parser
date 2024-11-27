import pytest


@pytest.mark.benchmark(group="parse_string")
def test_parse_string(benchmark):
    @benchmark
    def main():
        import xonsh_rd_parser as parser

        src_txt = "print(1)"
        return parser.parse_string(src_txt)


@pytest.mark.benchmark(group="parse_file")
def test_parse_file(benchmark):
    @benchmark
    def main():
        from pathlib import Path

        import xonsh_rd_parser as parser

        path = Path(__file__).parent / "bench.py"
        return parser.parse_file(str(path))
