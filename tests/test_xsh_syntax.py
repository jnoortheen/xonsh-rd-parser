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
def test_statements(exec_code, inp):
    exec_code(inp, mode="exec")


@pytest.mark.parametrize(
    "inp, result",
    [
        ("$(ls)", ["ls"]),
        ("$(ls )", ["ls"]),
        ("$( ls )", ["ls"]),
        ("$( ls)", ["ls"]),
        ("$(ls .)", ["ls", "."]),
        ('$(ls ".")', ["ls", "."]),
        ("$(ls -l)", ["ls", "-l"]),
        ("$(ls $WAKKA)", ["ls", "wak"]),
        ('$(ls @(None or "."))', ["ls", ["."]]),
        (
            '$(echo hello | @(lambda a, s=None: "hey!") foo bar baz)',
            [["echo", "hello"], (["hey!"], "foo", "bar", "baz")],
        ),
        pytest.param(
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
            marks=pytest.mark.xfail,
        ),
        pytest.param("$(echo @('a', 7))", ["echo", "a", 7], marks=pytest.mark.xfail),
        pytest.param(
            "$(@$(which echo) ls | @(lambda a, s=None: $(@(s.strip()) @(a[1]))) foo -la baz)",
            "",
            marks=pytest.mark.xfail,
        ),
        ("$(ls $(ls))", ["ls", ["ls"]]),
        ("$(ls $(ls) -l)", ["ls", ["ls"], "-l"]),
        ("$[ls]", ["ls"]),
        ("![ls]", ["ls"]),
        pytest.param(
            "![echo $WAKKA/place]", ["echo", "wak/place"], marks=pytest.mark.xfail
        ),
        ("![echo yo==yo]", ["echo", "yo==yo"]),
        ("!(ls | grep wakka)", [["ls"], ("grep", "wakka")]),
        (
            "!(ls | grep wakka | grep jawaka)",
            [[["ls"], ("grep", "wakka")], ("grep", "jawaka")],
        ),
    ],
)
def test_captured_procs(inp, result, exec_code):
    sh = exec_code(inp, xenv={"WAKKA": "wak"})
    assert sh.cmd.result == result

    last_call = sh.cmd.calls[-1]
    match inp[:2]:
        case "$[":
            assert last_call == "run"
        case "$(":
            assert last_call == "out"
        case "![":
            assert last_call == "hide"
        case "!(":
            assert last_call == "obj"


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
def test_bang_procs(expr, exec_code):
    exec_code(expr, xenv={"LS": "ll", "WAKKA": "wak"})


@pytest.mark.parametrize("p", ["", "p"])
@pytest.mark.parametrize("f", ["", "f"])
@pytest.mark.parametrize("glob_type", ["", "r", "g"])
@pytest.mark.xfail
def test_backtick(p, f, glob_type, exec_code):
    exec_code(f"print({p}{f}{glob_type}`.*`)")


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
@pytest.mark.xfail
def test_use_subshell(case, exec_code):
    exec_code(case)
