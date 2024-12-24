import ast
import textwrap
from unittest.mock import ANY

import pytest


@pytest.fixture(name="run")
def run_fixture(exec_code):
    def run(code, **kwargs):
        xsh = exec_code(code, mode="exec", x="x", locals=dict, globals=dict, **kwargs)
        return xsh.enter_macro, xsh.obs

    return run


WITH_BANG_RAWSUITES = [
    "pass",
    """\
x = 42
y = 12
""",
    """\
export PATH="yo:momma"
echo $PATH
""",
    """\
with q as t:
    v = 10
""",
    """\
with q as t:
    v = 10
    ls -l

for x in range(6):
    if True:
        pass
    else:
        ls -l
a = 42
""",
]


@pytest.mark.parametrize("body", WITH_BANG_RAWSUITES)
def test_withbang_single_suite(body, run):
    code = "with! x:\n{}".format(textwrap.indent(body, "    "))
    method, _ = run(code)
    method.assert_called_once_with("x", body.rstrip(), ANY, ANY)


@pytest.mark.parametrize("body", WITH_BANG_RAWSUITES)
def test_withbang_as_single_suite(body, run):
    code = "with! x as y:\n{}".format(textwrap.indent(body, "    "))
    method, tree = run(code)
    method.assert_called_once_with("x", body.rstrip(), ANY, ANY)
    assert " as y:" in ast.unparse(tree)


@pytest.mark.parametrize("body", WITH_BANG_RAWSUITES)
def test_withbang_single_suite_trailing(body, run):
    code = "with! x:\n{}\nprint(x)\n".format(textwrap.indent(body, "    "))
    method, _ = run(code)
    method.assert_called_once_with("x", body.rstrip(), ANY, ANY)


WITH_BANG_RAWSIMPLE = [
    "pass",
    "x = 42; y = 12",
    'export PATH="yo:momma"; echo $PATH',
    "[1,\n    2,\n    3]",
]


@pytest.mark.parametrize("body", WITH_BANG_RAWSIMPLE)
def test_withbang_single_simple(body, run):
    code = f"with! x: {body}\n"
    method, _ = run(code)
    method.assert_called_once_with("x", body, ANY, ANY)


@pytest.mark.parametrize("body", WITH_BANG_RAWSIMPLE)
def test_withbang_single_simple_opt(body, run):
    code = f"with! x as y: {body}\n"
    method, tree = run(code)
    method.assert_called_once_with("x", body, ANY, ANY)
    assert " as y:" in ast.unparse(tree)


@pytest.mark.parametrize("body", WITH_BANG_RAWSUITES)
def test_withbang_as_many_suite(body, run):
    code = "with! x as a, y as b, z as c:\n{}"
    code = code.format(textwrap.indent(body, "    "))
    method, tree = run(code, y="y", z="z")
    assert isinstance(tree, ast.Module)
    wither = tree.body[0]
    assert isinstance(wither, ast.With)
    assert len(wither.body) == 1
    assert isinstance(wither.body[0], ast.Pass)
    assert len(wither.items) == 3
    for targ, item in zip("abc", wither.items):
        assert getattr(item.optional_vars, "id", None) == targ
        assert isinstance(item.context_expr, ast.Call)
        assert isinstance(item.context_expr.args[1], ast.Constant)
        s = item.context_expr.args[1].value
        assert s == body.strip()
