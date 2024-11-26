from __future__ import annotations

import maturin_import_hook

maturin_import_hook.install()

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

    def list_of_strs_or_callables(x):
        """
        A simplified version of the xonsh function.
        """
        if isinstance(x, str | bytes):
            return [x]
        if callable(x):
            return [x([])]
        return x

    def subproc_captured(*cmds):
        return "-".join([str(item) for item in cmds])

    def subproc_captured_inject(*cmds):
        return cmds

    obj.list_of_strs_or_callables = MagicMock(wraps=list_of_strs_or_callables)
    obj.subproc_captured = MagicMock(wraps=subproc_captured)
    obj.subproc_captured_inject = MagicMock(wraps=subproc_captured_inject)
    return obj


@pytest.fixture
def xsh_proc_method(xsh):
    def factory(start_symbol: str):
        method_name = {
            "$[": "subproc_uncaptured",
            "$(": "subproc_captured",
            "![": "subproc_captured_hiddenobject",
            "!(": "subproc_captured_object",
        }[start_symbol]
        return getattr(xsh, method_name)

    return factory


#
@pytest.fixture
def check_xonsh_ast(xsh):
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
        return obs

    return factory


# configure plugins
pytest_plugins = ["yaml_snaps"]
