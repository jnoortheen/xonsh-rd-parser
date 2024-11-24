"""Tests the xonsh parser."""

import sys
from pathlib import Path
import pytest


def get_cases(path: Path, splitter="# "):
    inp = []
    parts = []

    def value():
        res = ("\n".join(inp), "\n".join(parts))
        inp.clear()
        parts.clear()
        return res

    for line in path.read_text().splitlines():
        if line.startswith(splitter):
            inp.append(line.lstrip(splitter))
        elif not line.strip():
            if parts:
                yield value()
        else:
            parts.append(line)
    if parts:
        yield value()


def glob_data_param(pattern: str):
    for path in Path(__file__).parent.joinpath("data").glob(pattern):
        for idx, (inp, exp) in enumerate(get_cases(path)):
            yield pytest.param(inp, exp, id=f"{path.name}-{idx}")


from dataclasses import dataclass


@dataclass
class LineItem:
    path: Path
    exp: str
    key: str
    idx: int
    write: bool = False

    def __eq__(self, other: str) -> bool:
        if self.write:
            self.write_yaml(other)
            return False
        return self.exp == other

    def __repr__(self):
        return repr(self.exp)

    def write_yaml(self, exp: str):
        from ruamel.yaml import YAML

        yaml = YAML()

        with self.path.open("r") as file:
            data = yaml.load(file)
        data[self.key][self.idx]["exp"] = exp
        yaml.dump(data, self.path.open("w"))


@pytest.fixture
def snapped(request):
    if request.config.getoption("--update-snaps"):
        request.param.write = True
    return request.param


def yaml_line_items(*names: str):
    for name in names:
        path = Path(__file__).parent.joinpath("data").joinpath(f"{name}.yml")
        from ruamel.yaml import YAML

        yaml = YAML()
        with path.open("r") as file:
            data = yaml.load(file)
        for case, lines in data.items():
            for idx, item in enumerate(lines):
                exp = LineItem(path, item.get("exp", ""), case, idx)
                yield pytest.param(item["inp"], exp, id=f"{path.stem}-{case}-{idx}")


@pytest.mark.parametrize(
    "inp, snapped", yaml_line_items("exprs", "stmts"), indirect=["snapped"]
)
def test_line_items(inp, unparse, snapped):
    assert snapped == unparse(inp)


@pytest.mark.parametrize("inp, exp", glob_data_param("fstring_py312.py"))
@pytest.mark.skipif(sys.version_info < (3, 12), reason="requires python3.12")
def test_py312_fstring(inp, exp, unparse_diff):
    unparse_diff(inp, exp)


@pytest.mark.parametrize(
    "inp",
    [
        'x = "WAKKA"; ${x} = 65',
        'x = "."; $(ls @(None or x))',
        'x = "."; !(ls @(None or x))',
        '$[git commit -am "wakka jawaka" ]',
        '$[git commit -am "flock jawaka milwaka" ]',
        '$[git commit -am "wakka jawaka"]',
        '$[git commit -am "flock jawaka"]',
        '![git commit -am "wakka jawaka" ]',
        '![git commit -am "flock jawaka milwaka" ]',
        '![git commit -am "wakka jawaka"]',
        '![git commit -am "flock jawaka"]',
    ],
)
def test_statements(check_xonsh_ast, inp):
    if not inp.endswith("\n"):
        inp += "\n"
    check_xonsh_ast(inp, mode="exec")


@pytest.mark.parametrize(
    "inp, args",
    [
        ("$(ls)", ["ls"]),
        ("$(ls )", ["ls"]),
        ("$( ls )", ["ls"]),
        ("$( ls)", ["ls"]),
        ("$(ls .)", ["ls", "."]),
        ('$(ls ".")', ["ls", '"."']),
        ("$(ls -l)", ["ls", "-l"]),
        ("$(ls $WAKKA)", ["ls", "wak"]),
        ('$(ls @(None or "."))', ["ls", "."]),
        (
            '$(echo hello | @(lambda a, s=None: "hey!") foo bar baz)',
            ["echo", "hello", "|", "hey!", "foo", "bar", "baz"],
        ),
        (
            "$(echo @(i**2 for i in range(20) ) )",
            [
                "echo",
                0,
                1,
                4,
                9,
                16,
                25,
                36,
                49,
                64,
                81,
                100,
                121,
                144,
                169,
                196,
                225,
                256,
                289,
                324,
                361,
            ],
        ),
        ("$(echo @('a', 7))", ["echo", "a", 7]),
        pytest.param(
            "$(@$(which echo) ls | @(lambda a, s=None: $(@(s.strip()) @(a[1]))) foo -la baz)",
            "",
            marks=pytest.mark.xfail,
        ),
        ("$(ls $(ls))", ["ls", "ls"]),
        ("$(ls $(ls) -l)", ["ls", "ls", "-l"]),
        ("$[ls]", ["ls"]),
        ("![ls]", ["ls"]),
        ("![echo $WAKKA/place]", ["echo", "wak/place"]),
        ("![echo yo==yo]", ["echo", "yo==yo"]),
        ("!(ls | grep wakka)", ["ls", "|", "grep", "wakka"]),
        (
            "!(ls | grep wakka | grep jawaka)",
            ["ls", "|", "grep", "wakka", "|", "grep", "jawaka"],
        ),
        ("!(ls > x.py)", ["ls", ">", "x.py"]),
    ],
)
def test_captured_procs(inp, args, check_xonsh_ast, xsh_proc_method):
    check_xonsh_ast(inp, mode="exec", xenv={"WAKKA": "wak"})
    method = xsh_proc_method(inp[:2])
    method.assert_called_with(*args)


@pytest.mark.parametrize(
    "expr",
    [
        "!(ls)",
        "!(ls )",
        "!( ls)",
        "!( ls )",
        "!(ls .)",
        '!(ls @(None or "."))',
        '!(ls ".")',
        "!(ls $(ls))",
        "!(ls $(ls) -l)",
        "!(ls $WAKKA)",
        "!($LS .)",
    ],
)
def test_bang_procs(expr, check_xonsh_ast):
    check_xonsh_ast(expr, xenv={"LS": "ll", "WAKKA": "wak"})


@pytest.mark.parametrize("p", ["", "p"])
@pytest.mark.parametrize("f", ["", "f"])
@pytest.mark.parametrize("glob_type", ["", "r", "g"])
def test_backtick(p, f, glob_type, check_xonsh_ast):
    check_xonsh_ast(f"print({p}{f}{glob_type}`.*`)", False)


def test_comment_only(check_xonsh_ast):
    check_xonsh_ast("# hello", mode="exec")


@pytest.mark.parametrize(
    "case",
    [
        "![(cat)]",
        "![(cat;)]",
        "![(cd path; ls; cd)]",
        '![(echo "abc"; sleep 1; echo "def")]',
        '![(echo "abc"; sleep 1; echo "def") | grep abc]',
        "![(if True:\n   ls\nelse:\n   echo not true)]",
    ],
)
def test_use_subshell(case, check_xonsh_ast):
    check_xonsh_ast(case)
