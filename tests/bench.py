import pytest


@pytest.fixture(name="big_python_file", scope="module")
def _big_python_file(tmp_path_factory):
    src_file = tmp_path_factory.mktemp("parser") / "big_file.py"
    with src_file.open("w") as fw:
        for idx in range(10000):
            fw.write(f"x_{idx} = {idx} + 1\n")
            fw.write(f"print(x_{idx})\n")
            fw.write(f"assert x_{idx} == {idx} + 1\n")
            if idx % 200 == 0:
                fw.flush()
    return src_file


def test_parse_file(big_python_file, parse_file, benchmark):
    def target():
        parse_file(str(big_python_file))

    benchmark.pedantic(target, iterations=5, warmup_rounds=1)


def test_xonsh_ply(big_python_file, benchmark):
    def target():
        from xonsh.parser import Parser

        p = Parser()
        p.parse(big_python_file.read_text())

    benchmark.pedantic(target, iterations=5, warmup_rounds=1)
