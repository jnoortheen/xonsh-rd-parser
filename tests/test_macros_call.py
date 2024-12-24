import itertools
import pytest


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
def run_fixture(exec_code):
    def run(code, **kwargs):
        xsh = exec_code(code, f="f", x="x", locals=dict, globals=dict, **kwargs)
        return xsh.call_macro

    return run


@pytest.mark.parametrize("s", MACRO_ARGS)
def test_macro_call_one_arg(run, s):
    f = f"f!({s})"

    method = run(f)
    args = method.call_args.args[:2]
    assert args == ("f", (s.strip(),))


@pytest.mark.parametrize("s,t", itertools.product(MACRO_ARGS[::2], MACRO_ARGS[1::2]))
def test_macro_call_two_args(run, s, t):
    f = f"f!({s}, {t})"
    method = run(f)
    args = method.call_args.args[1]
    assert [ar.strip() for ar in args] == [s.strip(), t.strip()]


@pytest.mark.parametrize(
    "s,t,u", itertools.product(MACRO_ARGS[::3], MACRO_ARGS[1::3], MACRO_ARGS[2::3])
)
def test_macro_call_three_args(run, s, t, u):
    f = f"f!({s}, {t}, {u})"
    method = run(f)
    args = method.call_args.args[1]
    assert [ar.strip() for ar in args] == [s.strip(), t.strip(), u.strip()]


@pytest.mark.parametrize("s", MACRO_ARGS)
def test_macro_call_one_trailing(run, s):
    f = f"f!({s},)"
    method = run(f)
    args = method.call_args.args[1]
    assert [ar.strip() for ar in args] == [s.strip()]


@pytest.mark.parametrize("s", MACRO_ARGS)
def test_macro_call_one_trailing_space(run, s):
    f = f"f!( {s}, )"
    method = run(f)
    args = method.call_args.args[1]
    assert [ar.strip() for ar in args] == [s.strip()]
