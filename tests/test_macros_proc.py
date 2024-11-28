import itertools
from ast import AST
from unittest.mock import ANY

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
@pytest.mark.xfail
def test_empty_subprocbang(
    opener, closer, body, args, check_xonsh_ast, xsh_proc_method
):
    tree = check_xonsh_ast(opener + body + closer)
    assert isinstance(tree, AST)
    method = xsh_proc_method(opener)
    method.assert_called_once_with("echo", *args)


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
@pytest.mark.xfail
def test_many_subprocbang(opener, closer, body, check_xonsh_ast, xsh_proc_method):
    tree = check_xonsh_ast(opener + body + closer)
    assert isinstance(tree, AST)
    method = xsh_proc_method(opener)
    cmd, arg = body.split("!", 1)
    method.assert_called_once_with(cmd.strip(), arg.strip())


@pytest.mark.xfail
def test_macro_call_empty(check_xonsh_ast, xsh):
    tree = check_xonsh_ast("f!()", f="f")
    assert isinstance(tree, AST)


MACRO_ARGS = [
    "x",
    "True",
    "None",
    "import os",
    "x=10",
    '"oh my kadavule!"',
    "...",
    " ... ",
    "if True:\n  pass",
    "{x: y}",
    "{x: y, 42: 5}",
    "{1, 2, 3,}",
    "(x,y)",
    "(x, y)",
    "((x, y), z)",
    "g()",
    "range(10)",
    "range(1, 10, 2)",
    "()",
    "{}",
    "[]",
    "[1, 2]",
    "@(x)",
    "!(ls -l)",
    "![ls -l]",
    "$(ls -l)",
    "${x + y}",
    "$[ls -l]",
    "@$(which xonsh)",
]


@pytest.fixture(name="run")
def run_fixture(check_xonsh_ast, xsh):
    def run(code, **kwargs):
        tree = check_xonsh_ast(
            code, mode="exec", f="f", x="x", locals=dict, globals=dict, **kwargs
        )
        assert isinstance(tree, AST)
        return xsh.call_macro

    return run


@pytest.mark.parametrize("s", MACRO_ARGS)
@pytest.mark.xfail
def test_macro_call_one_arg(run, s, xsh):
    f = f"f!({s})"

    method = run(f)
    method.assert_called_once_with("f", (s,), ANY, ANY)


@pytest.mark.parametrize("s,t", itertools.product(MACRO_ARGS[::2], MACRO_ARGS[1::2]))
@pytest.mark.xfail
def test_macro_call_two_args(run, s, t, xsh):
    f = f"f!({s}, {t})"
    method = run(f)
    args = method.call_args.args[1]
    assert [ar.strip() for ar in args] == [s.strip(), t.strip()]


@pytest.mark.parametrize(
    "s,t,u", itertools.product(MACRO_ARGS[::3], MACRO_ARGS[1::3], MACRO_ARGS[2::3])
)
@pytest.mark.xfail
def test_macro_call_three_args(run, s, t, u, xsh):
    f = f"f!({s}, {t}, {u})"
    method = run(f)
    args = method.call_args.args[1]
    assert [ar.strip() for ar in args] == [s.strip(), t.strip(), u.strip()]


@pytest.mark.parametrize("s", MACRO_ARGS)
@pytest.mark.xfail
def test_macro_call_one_trailing(run, s, xsh):
    f = f"f!({s},)"
    method = run(f)
    args = method.call_args.args[1]
    assert [ar.strip() for ar in args] == [s.strip()]


@pytest.mark.parametrize("s", MACRO_ARGS)
@pytest.mark.xfail
def test_macro_call_one_trailing_space(run, s, xsh):
    f = f"f!( {s}, )"
    method = run(f)
    args = method.call_args.args[1]
    assert [ar.strip() for ar in args] == [s.strip()]
