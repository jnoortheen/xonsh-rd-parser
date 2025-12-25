from xonsh_rd_parser import get_big_py_file


def setup():
    return (get_big_py_file(),), {}


def test_parse_file(parse_file, benchmark):
    benchmark.pedantic(parse_file, setup=setup, rounds=4)


def test_xonsh_ply(benchmark):
    def target(file_path):
        from xonsh.parser import Parser
        from pathlib import Path

        p = Parser()
        p.parse(Path(file_path).read_text())

    # less rounds, to reduce CI running time
    benchmark.pedantic(target, setup=setup, rounds=2)
