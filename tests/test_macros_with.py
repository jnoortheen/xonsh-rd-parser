import ast
import textwrap
from ast import AST, Pass, With
from unittest.mock import ANY

import pytest


@pytest.fixture(name="run")
def run_fixture(exec_code, xsh):
    def run(code, **kwargs):
        tree = exec_code(code, mode="exec", x="x", locals=dict, globals=dict, **kwargs)
        assert isinstance(tree, AST)
        return xsh.enter_macro, tree

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
@pytest.mark.xfail
def test_withbang_single_suite(body, run):
    code = "with! x:\n{}".format(textwrap.indent(body, "    "))
    method, _ = run(code)
    method.assert_called_once_with("x", body, ANY, ANY)


@pytest.mark.parametrize("body", WITH_BANG_RAWSUITES)
@pytest.mark.xfail
def test_withbang_as_single_suite(body, run):
    code = "with! x as y:\n{}".format(textwrap.indent(body, "    "))
    method, tree = run(code)
    method.assert_called_once_with("x", body, ANY, ANY)
    assert " as y:" in ast.unparse(tree)


@pytest.mark.parametrize("body", WITH_BANG_RAWSUITES)
@pytest.mark.xfail
def test_withbang_single_suite_trailing(body, run):
    code = "with! x:\n{}\nprint(x)\n".format(textwrap.indent(body, "    "))
    method, tree = run(code)
    method.assert_called_once_with("x", body + "\n", ANY, ANY)


WITH_BANG_RAWSIMPLE = [
    "pass",
    "x = 42; y = 12",
    'export PATH="yo:momma"; echo $PATH',
    "[1,\n    2,\n    3]",
]


@pytest.mark.parametrize("body", WITH_BANG_RAWSIMPLE)
@pytest.mark.xfail
def test_withbang_single_simple(body, run):
    code = f"with! x: {body}\n"
    method, tree = run(code)
    method.assert_called_once_with("x", " " + body + "\n", ANY, ANY)


@pytest.mark.parametrize("body", WITH_BANG_RAWSIMPLE)
@pytest.mark.xfail
def test_withbang_single_simple_opt(body, run):
    code = f"with! x as y: {body}\n"
    method, tree = run(code)
    method.assert_called_once_with("x", " " + body + "\n", ANY, ANY)
    assert " as y:" in ast.unparse(tree)


@pytest.mark.parametrize("body", WITH_BANG_RAWSUITES)
@pytest.mark.xfail
def test_withbang_as_many_suite(body, run):
    code = "with! x as a, y as b, z as c:\n{}"
    code = code.format(textwrap.indent(body, "    "))
    method, tree = run(code)
    assert isinstance(tree, AST)
    wither = tree.body[0]
    assert isinstance(wither, With)
    assert len(wither.body) == 1
    assert isinstance(wither.body[0], Pass)
    assert len(wither.items) == 3
    for i, targ in enumerate("abc"):
        item = wither.items[i]
        assert item.optional_vars.id == targ
        s = item.context_expr.args[1].s
        assert s == body
