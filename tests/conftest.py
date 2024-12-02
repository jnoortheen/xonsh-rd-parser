import os

if not os.environ.get("GITHUB_ACTIONS"):
    import maturin_import_hook
    from maturin_import_hook.settings import MaturinSettings

    maturin_import_hook.install(
        settings=MaturinSettings(
            release=False,
            # uv=True,
        )
    )
import ast
import logging
from unittest.mock import MagicMock

import pytest
from xonsh_rd_parser import parse_string

log = logging.getLogger(__name__)


def nodes_equal(x, y):
    assert type(x) is type(
        y
    ), f"Ast nodes do not have the same type: '{type(x)}' != '{type(y)}' "
    if isinstance(x, ast.Constant):
        assert x.value == y.value, (
            f"Constant ast nodes do not have the same value: "
            f"{x.value!r} != {y.value!r}"
        )
    if isinstance(x, ast.Expr | ast.FunctionDef | ast.ClassDef):
        assert (
            x.lineno == y.lineno
        ), f"Ast nodes do not have the same line number : {x.lineno} != {y.lineno}"
        assert (
            x.col_offset == y.col_offset
        ), f"Ast nodes do not have the same column offset number : {x.col_offset} != {y.col_offset}"
    for (xname, xval), (yname, yval) in zip(
        ast.iter_fields(x), ast.iter_fields(y), strict=False
    ):
        assert (
            xname == yname
        ), f"Ast nodes fields differ : {xname} (of type {type(xval)}) != {yname} (of type {type(yval)})"
        assert (
            type(xval) is type(yval)
        ), f"Ast nodes fields differ : {xname} (of type {type(xval)}) != {yname} (of type {type(yval)})"
    for xchild, ychild in zip(
        ast.iter_child_nodes(x), ast.iter_child_nodes(y), strict=False
    ):
        assert nodes_equal(xchild, ychild), "Ast node children differs"
    return True


@pytest.fixture
def unparse(unparse_diff):
    def factory(text: str):
        left_tree = parse_string(text)
        return ast.unparse(left_tree)
        # from test_py_syntax import dump_diff
        # assert not dump_diff(parsed=left_tree, expected=right_tree), f"Generated AST didn't match. Source: {text}"

    return factory


@pytest.fixture
def unparse_diff():
    def factory(text: str, right: str | None = None):
        left_tree = parse_string(text)
        left = ast.unparse(left_tree)
        right = right or left
        right_tree = ast.parse(right)
        assert left == ast.unparse(right_tree), f"unparse didn't match. Source: {text}"
        # from test_py_syntax import dump_diff
        # assert not dump_diff(parsed=left_tree, expected=right_tree), f"Generated AST didn't match. Source: {text}"

    return factory


@pytest.fixture
def xsh():
    obj = MagicMock()

    class Cmd:
        def __call__(self, *args, **kwargs):
            self.args = list(args)
            self.kwargs = kwargs
            self.result = None
            self.calls = []
            return self

        def out(self):
            self.result = self.args
            self.calls.append("out")
            return self.result

        def run(self):
            self.result = self.args
            self.calls.append("run")
            return self.result

        def hide(self):
            self.result = self.args
            self.calls.append("hide")
            return self.result

        def obj(self):
            self.result = self.args
            self.calls.append("obj")
            return self.result

        def pipe(self, *args):
            self.args = [self.args, args]
            return self

    def list_of_strs_or_callables(x):
        """
        A simplified version of the xonsh function.
        """
        if isinstance(x, str | bytes):
            return [x]
        if callable(x):
            return [x([])]
        return x

    # using instance to store the result
    obj.cmd = Cmd()
    obj.list_of_strs_or_callables = MagicMock(wraps=list_of_strs_or_callables)
    return obj


#
@pytest.fixture
def exec_code(xsh):
    """compatibility fixture"""

    def factory(
        inp: str,
        xenv: dict | None = None,
        mode="exec",
        **locs,
    ):
        obs = parse_string(inp)
        bytecode = compile(obs, "<test-xonsh-ast>", mode)
        xsh.env = xenv or {}
        locs["__xonsh__"] = xsh
        exec(bytecode, {}, locs)
        return xsh

    return factory


# configure plugins
pytest_plugins = ["yaml_snaps"]
