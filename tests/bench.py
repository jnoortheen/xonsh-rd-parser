import os

from xonsh_rd_parser import get_big_py_file

# On GitHub CI the "CI" env var is always set to "true".
# Use a smaller file there to keep benchmark wall-time reasonable.
_LINES = 1000 if os.environ.get("CI") else None


def test_parse_file(parse_file, benchmark):
    file_path = get_big_py_file(_LINES)
    benchmark(parse_file, file_path)


def test_xonsh_ply(benchmark):
    from pathlib import Path

    from xonsh.parsers.v310 import Parser

    file_path = get_big_py_file(_LINES)
    p = Parser()

    benchmark(p.parse, Path(file_path).read_text())
