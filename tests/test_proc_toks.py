import pytest
from xonsh_rd_parser import Parser

INDENT = "    "


def subproc_toks(inp, returnline=False, greedy=False, mincol=0, maxcol=None):
    lexer = Parser(inp)
    return lexer.subproc_toks(
        returnline=returnline, greedy=greedy, mincol=mincol, maxcol=maxcol
    )


@pytest.mark.parametrize(
    "inp",
    [
        "x",
        "ls -l",
        "git commit -am 'hello doc'",
    ],
)
@pytest.mark.parametrize("end", ["\n", ";", ""])
def test_subproc_toks(inp, end):
    exp = f"![{inp}]{end}"
    assert subproc_toks(inp + end, returnline=True) == exp


def test_bash_macro():
    s = "bash -c ! export var=42; echo $var"
    exp = f"![{s}]\n"
    obs = subproc_toks(s + "\n", returnline=True)
    assert exp == obs


def test_python_macro():
    s = 'python -c ! import os; print(os.path.abspath("/"))'
    exp = f"![{s}]\n"
    obs = subproc_toks(s + "\n", returnline=True)
    assert exp == obs


def test_subproc_toks_indent_ls():
    s = "ls -l"
    exp = INDENT + f"![{s}]"
    obs = subproc_toks(INDENT + s, mincol=len(INDENT), returnline=True)
    assert exp == obs


def test_subproc_toks_indent_ls_nl():
    s = "ls -l"
    exp = INDENT + f"![{s}]\n"
    obs = subproc_toks(INDENT + s + "\n", mincol=len(INDENT), returnline=True)
    assert exp == obs


def test_subproc_toks_indent_ls_no_min():
    s = "ls -l"
    exp = INDENT + f"![{s}]"
    obs = subproc_toks(INDENT + s, returnline=True)
    assert exp == obs


def test_subproc_toks_indent_ls_no_min_nl():
    s = "ls -l"
    exp = INDENT + f"![{s}]\n"
    obs = subproc_toks(INDENT + s + "\n", returnline=True)
    assert exp == obs


def test_subproc_toks_indent_ls_no_min_semi():
    s = "ls"
    exp = INDENT + f"![{s}];"
    obs = subproc_toks(INDENT + s + ";", returnline=True)
    assert exp == obs


def test_subproc_toks_indent_ls_no_min_semi_nl():
    s = "ls"
    exp = INDENT + f"![{s}];\n"
    obs = subproc_toks(INDENT + s + ";\n", returnline=True)
    assert exp == obs


def test_subproc_toks_ls_comment():
    s = "ls -l"
    com = "  # lets list"
    exp = f"![{s}]{com}"
    obs = subproc_toks(s + com, returnline=True)
    assert exp == obs


def test_subproc_toks_ls_42_comment():
    s = "ls 42"
    com = "  # lets list"
    exp = f"![{s}]{com}"
    obs = subproc_toks(s + com, returnline=True)
    assert exp == obs


def test_subproc_toks_ls_str_comment():
    s = 'ls "wakka"'
    com = "  # lets list"
    exp = f"![{s}]{com}"
    obs = subproc_toks(s + com, returnline=True)
    assert exp == obs


def test_subproc_toks_indent_ls_comment():
    ind = "    "
    s = "ls -l"
    com = "  # lets list"
    exp = f"{ind}![{s}]{com}"
    obs = subproc_toks(ind + s + com, returnline=True)
    assert exp == obs


def test_subproc_toks_indent_ls_str():
    ind = "    "
    s = 'ls "wakka"'
    com = "  # lets list"
    exp = f"{ind}![{s}]{com}"
    obs = subproc_toks(ind + s + com, returnline=True)
    assert exp == obs


def test_subproc_toks_ls_l_semi_ls_first():
    lsdl = "ls -l"
    ls = "ls"
    s = f"{lsdl}; {ls}"
    exp = f"![{lsdl}]; {ls}"
    obs = subproc_toks(s, maxcol=6, returnline=True)
    assert exp == obs


def test_subproc_toks_ls_l_semi_ls_second():
    lsdl = "ls -l"
    ls = "ls"
    s = f"{lsdl}; {ls}"
    exp = f"{lsdl}; ![{ls}]"
    obs = subproc_toks(s, mincol=7, returnline=True)
    assert exp == obs


def test_subproc_toks_hello_mom_first():
    fst = "echo 'hello'"
    sec = "echo 'mom'"
    s = f"{fst}; {sec}"
    exp = f"![{fst}]; {sec}"
    obs = subproc_toks(s, maxcol=len(fst) + 1, returnline=True)
    assert exp == obs


def test_subproc_toks_hello_mom_second():
    fst = "echo 'hello'"
    sec = "echo 'mom'"
    s = f"{fst}; {sec}"
    exp = f"{fst}; ![{sec}]"
    obs = subproc_toks(s, mincol=len(fst), returnline=True)
    assert exp == obs


def test_subproc_toks_hello_bad_leading_single_quotes():
    obs = subproc_toks('echo "hello', returnline=True)
    assert obs is None


def test_subproc_toks_hello_bad_trailing_single_quotes():
    obs = subproc_toks('echo hello"', returnline=True)
    assert obs is None


def test_subproc_toks_hello_bad_leading_triple_quotes():
    obs = subproc_toks('echo """hello', returnline=True)
    assert obs is None


def test_subproc_toks_hello_bad_trailing_triple_quotes():
    obs = subproc_toks('echo hello"""', returnline=True)
    assert obs is None


def test_subproc_toks_hello_mom_triple_quotes_nl():
    s = 'echo """hello\nmom"""'
    exp = f"![{s}]"
    obs = subproc_toks(s, returnline=True)
    assert exp == obs


def test_subproc_toks_comment():
    exp = None
    obs = subproc_toks("# I am a comment", returnline=True)
    assert exp == obs


def test_subproc_toks_not():
    exp = "not ![echo mom]"
    obs = subproc_toks("not echo mom", returnline=True)
    assert exp == obs


def test_subproc_toks_paren():
    exp = "(![echo mom])"
    obs = subproc_toks("(echo mom)", returnline=True)
    assert exp == obs


def test_subproc_toks_paren_ws():
    exp = "(![echo mom])  "
    obs = subproc_toks("(echo mom)  ", returnline=True)
    assert exp == obs


def test_subproc_toks_not_paren():
    exp = "not (![echo mom])"
    obs = subproc_toks("not (echo mom)", returnline=True)
    assert exp == obs


def test_subproc_toks_and_paren():
    exp = "True and (![echo mom])"
    obs = subproc_toks("True and (echo mom)", returnline=True)
    assert exp == obs


def test_subproc_toks_paren_and_paren():
    exp = "(![echo a]) and (echo b)"
    obs = subproc_toks("(echo a) and (echo b)", maxcol=9, returnline=True)
    assert obs == exp


def test_subproc_toks_semicolon_only():
    exp = None
    obs = subproc_toks(";", returnline=True)
    assert exp == obs


def test_subproc_toks_pyeval():
    s = "echo @(1+1)"
    exp = f"![{s}]"
    obs = subproc_toks(s, returnline=True)
    assert exp == obs


def test_subproc_toks_pyeval_multiline_string():
    s = 'echo @("""hello\nmom""")'
    exp = f"![{s}]"
    obs = subproc_toks(s, returnline=True)
    assert exp == obs


def test_subproc_toks_twopyeval():
    s = "echo @(1+1) @(40 + 2)"
    exp = f"![{s}]"
    obs = subproc_toks(s, returnline=True)
    assert exp == obs


def test_subproc_toks_pyeval_parens():
    s = "echo @(1+1)"
    inp = f"({s})"
    exp = f"(![{s}])"
    obs = subproc_toks(inp, returnline=True)
    assert exp == obs


def test_subproc_toks_twopyeval_parens():
    s = "echo @(1+1) @(40+2)"
    inp = f"({s})"
    exp = f"(![{s}])"
    obs = subproc_toks(inp, returnline=True)
    assert exp == obs


def test_subproc_toks_pyeval_nested():
    s = "echo @(min(1, 42))"
    exp = f"![{s}]"
    obs = subproc_toks(s, returnline=True)
    assert exp == obs


@pytest.mark.parametrize(
    "phrase",
    [
        "xandy",
        "xory",
        "xand",
        "andy",
        "xor",
        "ory",
        "x-and",
        "x-or",
        "and-y",
        "or-y",
        "x-and-y",
        "x-or-y",
        "in/and/path",
        "in/or/path",
    ],
)
def test_subproc_toks_and_or(phrase):
    s = "echo " + phrase
    exp = f"![{s}]"
    obs = subproc_toks(s, returnline=True)
    assert exp == obs


def test_subproc_toks_pyeval_nested_parens():
    s = "echo @(min(1, 42))"
    inp = f"({s})"
    exp = f"(![{s}])"
    obs = subproc_toks(inp, returnline=True)
    assert exp == obs


def test_subproc_toks_capstdout():
    s = "echo $(echo bat)"
    exp = f"![{s}]"
    obs = subproc_toks(s, returnline=True)
    assert exp == obs


def test_subproc_toks_capproc():
    s = "echo !(echo bat)"
    exp = f"![{s}]"
    obs = subproc_toks(s, returnline=True)
    assert exp == obs


def test_subproc_toks_pyeval_redirect():
    s = 'echo @("foo") > bar'
    inp = f"{s}"
    exp = f"![{s}]"
    obs = subproc_toks(inp, returnline=True)
    assert exp == obs


def test_subproc_toks_greedy_parens():
    s = "(sort)"
    exp = f"![{s}]"
    obs = subproc_toks(s, returnline=True, greedy=True)
    assert exp == obs


def test_subproc_toks_greedy_parens_inp():
    s = "(sort) < input.txt"
    exp = f"![{s}]"
    obs = subproc_toks(s, returnline=True, greedy=True)
    assert exp == obs


def test_subproc_toks_greedy_parens_statements():
    s = '(echo "abc"; sleep 1; echo "def")'
    exp = f"![{s}]"
    obs = subproc_toks(s, returnline=True, greedy=True)
    assert exp == obs


def test_subproc_toks_greedy_parens_statements_with_grep():
    s = '(echo "abc"; sleep 1; echo "def") | grep'
    exp = f"![{s}]"
    obs = subproc_toks(s, returnline=True, greedy=True)
    assert obs == exp
