"""Test pure Python parser against cpython parser."""

import ast
import difflib
import sys
from pathlib import Path

import pytest
from xonsh_rd_parser import parse_file, parse_string

files = []
for py in (Path(__file__).parent / "data").glob("*.py"):
    if ".3_" in py.name:
        _, syntax_version, _ = py.name.rsplit(".", 2)
        if sys.version_info < tuple(int(v) for v in syntax_version.split("_")):
            continue
    files.append(pytest.param(py, id=py.name))


def unparse_diff(**trees: ast.AST):
    orig_name, pp_name = trees.keys()
    original, pp_ast = trees.values()
    left = ast.unparse(original)
    try:
        right = ast.unparse(pp_ast)
    except Exception as e:
        return f"Unparse failed with {e}"
    return "\n".join(
        difflib.unified_diff(left.split("\n"), right.split("\n"), orig_name, pp_name)
    )


def dump_diff(**trees: ast.AST):
    kwargs = {
        # "include_attributes": True, # todo: uncomment when we fix the coloffset in ruff parser
        "indent": "  "
    }
    orig_name, pp_name = trees.keys()
    original, pp_ast = trees.values()
    o = ast.dump(original, **kwargs)
    p = ast.dump(pp_ast, **kwargs)
    return "\n".join(
        difflib.unified_diff(o.split("\n"), p.split("\n"), orig_name, pp_name)
    )


marks = {"marks": pytest.mark.xfail} if sys.version_info < (3, 12) else {}


@pytest.mark.parametrize("filename", files)
def test_pure_python_parsing(filename):
    source = filename.read_text()
    for part in source.split("\n\n\n"):
        original = ast.parse(part)

        pp_ast = parse_string(part)

        if diff := dump_diff(cpython=original, pegen=pp_ast):
            if src_diff := unparse_diff(original=original, pp_ast=pp_ast):
                print("Source diff")
                print(src_diff)
            else:
                print("Unparsed sources are the same")
            print("AST diff")
            print(diff)

        assert not diff, "mismatch in generated AST"

    diff = dump_diff(cpython=ast.parse(source), pegen=parse_file(str(filename)))
    assert not diff
