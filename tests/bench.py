from xonsh_rd_parser import get_big_py_file


def test_parse_file(parse_file, benchmark):
    file_path = get_big_py_file()
    benchmark(parse_file, file_path)


def test_xonsh_ply(benchmark):
    from xonsh.parsers.v310 import Parser
    from pathlib import Path

    file_path = get_big_py_file()
    p = Parser()

    benchmark(p.parse, Path(file_path).read_text())
