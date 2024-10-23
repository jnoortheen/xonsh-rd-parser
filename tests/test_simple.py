import pytest


@pytest.mark.parametrize(
    "inp",
    [
        'r"""some long lines\nmore lines\n"""',
        'r"some \\nlong lines"',
    ],
)
def test_ast_strings(inp, unparse_diff):
    unparse_diff(inp)
