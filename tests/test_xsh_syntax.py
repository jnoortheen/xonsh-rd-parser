"""Tests the xonsh parser."""

import pytest

from yaml_snaps import yaml_line_items


@pytest.mark.parametrize(
    "inp, snapped", yaml_line_items("exprs", "stmts"), indirect=["snapped"]
)
def test_line_items(inp, unparse, snapped):
    snapped.matches(unparse(inp))


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
def test_captured_procs(inp, args, check_xonsh_ast, xsh):
    check_xonsh_ast(inp, mode="exec", xenv={"WAKKA": "wak"})
    xsh.cmd.assert_called_with(*args)


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
