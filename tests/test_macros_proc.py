import pytest

SUBPROC_MACRO_OC = [("!(", ")"), ("$(", ")"), ("![", "]"), ("$[", "]")]


@pytest.mark.parametrize("opener, closer", SUBPROC_MACRO_OC)
@pytest.mark.parametrize(
    "body, args",
    [
        ("echo!", [""]),
        ("echo !", [""]),
        ("echo ! ", [""]),
        ("echo!x", ["x"]),
        ("echo !x", ["x"]),
        ("echo ! x", ["x"]),
        ("echo ! x ", ["x"]),
        ("echo -n!x", ["-n", "x"]),
        ("echo -n !x", ["-n", "x"]),
        ("echo -n ! x", ["-n", "x"]),
        ("echo -n ! x ", ["-n", "x"]),
    ],
)
def test_empty_subprocbang(opener, closer, body, args, exec_code):
    sh = exec_code(opener + body + closer)
    assert sh.cmd.result == ["echo"] + args


@pytest.mark.parametrize("opener, closer", SUBPROC_MACRO_OC)
@pytest.mark.parametrize(
    "body",
    [
        "echo!x + y",
        "echo !x + y",
        "echo !x + y",
        "echo ! x + y",
        "timeit! bang! and more",
        "timeit! recurse() and more",
        "timeit! recurse[] and more",
        "timeit! recurse!() and more",
        "timeit! recurse![] and more",
        "timeit! recurse$() and more",
        "timeit! recurse$[] and more",
        "timeit! recurse!() and more",
        "timeit!!!!",
        "timeit! (!)",
        "timeit! [!]",
        "timeit!!(ls)",
        'timeit!"!)"',
    ],
)
def test_many_subprocbang(opener, closer, body, exec_code):
    sh = exec_code(opener + body + closer)
    cmd, arg = body.split("!", 1)
    assert sh.cmd.result == [cmd.strip(), arg.strip()]
