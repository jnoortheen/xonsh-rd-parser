"""Tests the xonsh lexer."""

import pytest

from xonsh_rd_parser import Parser
from inline_snapshot import snapshot

LEXER_ARGS = {"lextab": "lexer_test_table", "debug": 0}


def lex_input(inp: str, tolerant=False):
    return [
        (t.kind, f"{t.start}..{t.end}", inp[t.start : t.end])
        for t in Parser(inp).tokens(tolerant=tolerant)
    ][:-1]


def test_int_literal():
    assert lex_input("42") == snapshot([("Int", "0..2", "42")])
    assert lex_input("4_2") == snapshot([("Int", "0..3", "4_2")])


def test_indent():
    assert lex_input("  \t  42") == snapshot(
        [("Indent", "0..5", "  \t  "), ("Int", "5..7", "42"), ("Newline", "7..7", "")]
    )


def test_whitespace():
    assert lex_input("42  \t  ") == snapshot([("Int", "0..2", "42")])
    assert lex_input("42  +\t65") == snapshot(
        [("Int", "0..2", "42"), ("Plus", "4..5", "+"), ("Int", "6..8", "65")]
    )
    assert lex_input(" 42  +\t65") == snapshot(
        [
            ("Indent", "0..1", " "),
            ("Int", "1..3", "42"),
            ("Plus", "5..6", "+"),
            ("Int", "7..9", "65"),
            ("Newline", "9..9", ""),
        ]
    )


def test_atdollar_expression():
    inp = "@$(which python)"
    assert lex_input(inp) == snapshot(
        [
            ("AtDollarLParen", "0..3", "@$("),
            ("Name", "3..8", "which"),
            ("Name", "9..15", "python"),
            ("Rpar", "15..16", ")"),
        ]
    )


def test_and():
    assert lex_input("and") == snapshot([("And", "0..3", "and")])


def test_ampersand():
    assert lex_input("&") == snapshot([("Amper", "0..1", "&")])


def test_not_really_and_pre():
    inp = "![foo-and]"
    assert lex_input(inp) == snapshot(
        [
            ("BangLSqb", "0..2", "!["),
            ("Name", "2..5", "foo"),
            ("Minus", "5..6", "-"),
            ("And", "6..9", "and"),
            ("Rsqb", "9..10", "]"),
        ]
    )


def test_not_really_and_post():
    inp = "![and-bar]"
    assert lex_input(inp) == snapshot(
        [
            ("BangLSqb", "0..2", "!["),
            ("And", "2..5", "and"),
            ("Minus", "5..6", "-"),
            ("Name", "6..9", "bar"),
            ("Rsqb", "9..10", "]"),
        ]
    )


def test_not_really_and_pre_post():
    inp = "![foo-and-bar]"
    assert lex_input(inp) == snapshot(
        [
            ("BangLSqb", "0..2", "!["),
            ("Name", "2..5", "foo"),
            ("Minus", "5..6", "-"),
            ("And", "6..9", "and"),
            ("Minus", "9..10", "-"),
            ("Name", "10..13", "bar"),
            ("Rsqb", "13..14", "]"),
        ]
    )


def test_not_really_or_pre():
    inp = "![foo-or]"
    assert lex_input(inp) == snapshot(
        [
            ("BangLSqb", "0..2", "!["),
            ("Name", "2..5", "foo"),
            ("Minus", "5..6", "-"),
            ("Or", "6..8", "or"),
            ("Rsqb", "8..9", "]"),
        ]
    )


def test_not_really_or_post():
    inp = "![or-bar]"
    assert lex_input(inp) == snapshot(
        [
            ("BangLSqb", "0..2", "!["),
            ("Or", "2..4", "or"),
            ("Minus", "4..5", "-"),
            ("Name", "5..8", "bar"),
            ("Rsqb", "8..9", "]"),
        ]
    )


def test_pref_suff_and():
    inp = "echo and-y"
    assert lex_input(inp) == snapshot(
        [
            ("Name", "0..4", "echo"),
            ("And", "5..8", "and"),
            ("Minus", "8..9", "-"),
            ("Name", "9..10", "y"),
        ]
    )


def test_not_really_or_pre_post():
    inp = "![foo-or-bar]"
    assert lex_input(inp) == snapshot(
        [
            ("BangLSqb", "0..2", "!["),
            ("Name", "2..5", "foo"),
            ("Minus", "5..6", "-"),
            ("Or", "6..8", "or"),
            ("Minus", "8..9", "-"),
            ("Name", "9..12", "bar"),
            ("Rsqb", "12..13", "]"),
        ]
    )


def test_subproc_line_cont_space():
    inp = (
        "![echo --option1 value1 \\\n"
        "     --option2 value2 \\\n"
        "     --optionZ valueZ]"
    )
    assert lex_input(inp) == snapshot(
        [
            ("BangLSqb", "0..2", "!["),
            ("Name", "2..6", "echo"),
            ("Minus", "7..8", "-"),
            ("Minus", "8..9", "-"),
            ("Name", "9..16", "option1"),
            ("Name", "17..23", "value1"),
            ("Minus", "31..32", "-"),
            ("Minus", "32..33", "-"),
            ("Name", "33..40", "option2"),
            ("Name", "41..47", "value2"),
            ("Minus", "55..56", "-"),
            ("Minus", "56..57", "-"),
            ("Name", "57..64", "optionZ"),
            ("Name", "65..71", "valueZ"),
            ("Rsqb", "71..72", "]"),
        ]
    )


def test_subproc_line_cont_nospace():
    inp = (
        "![echo --option1 value1\\\n"
        "     --option2 value2\\\n"
        "     --optionZ valueZ]"
    )
    assert lex_input(inp) == snapshot(
        [
            ("BangLSqb", "0..2", "!["),
            ("Name", "2..6", "echo"),
            ("Minus", "7..8", "-"),
            ("Minus", "8..9", "-"),
            ("Name", "9..16", "option1"),
            ("Name", "17..23", "value1"),
            ("Minus", "30..31", "-"),
            ("Minus", "31..32", "-"),
            ("Name", "32..39", "option2"),
            ("Name", "40..46", "value2"),
            ("Minus", "53..54", "-"),
            ("Minus", "54..55", "-"),
            ("Name", "55..62", "optionZ"),
            ("Name", "63..69", "valueZ"),
            ("Rsqb", "69..70", "]"),
        ]
    )


def test_atdollar():
    assert lex_input("@$()") == snapshot(
        [("AtDollarLParen", "0..3", "@$("), ("Rpar", "3..4", ")")]
    )


def test_doubleamp():
    assert lex_input("&&") == snapshot([("DoubleAmp", "0..2", "&&")])


def test_pipe():
    assert lex_input("|") == snapshot([("Vbar", "0..1", "|")])


def test_doublepipe():
    assert lex_input("||") == snapshot([("DoublePipe", "0..2", "||")])


def test_single_quote_literal():
    assert lex_input("'yo'") == snapshot([("String", "0..4", "'yo'")])


def test_double_quote_literal():
    assert lex_input('"yo"') == snapshot([("String", "0..4", '"yo"')])


def test_path_string_literal():
    assert lex_input("p'/foo'") == snapshot([("String", "0..7", "p'/foo'")])
    assert lex_input('p"/foo"') == snapshot([("String", "0..7", 'p"/foo"')])
    assert lex_input("pr'/foo'") == snapshot([("String", "0..8", "pr'/foo'")])
    assert lex_input('pr"/foo"') == snapshot([("String", "0..8", 'pr"/foo"')])
    assert lex_input("rp'/foo'") == snapshot([("String", "0..8", "rp'/foo'")])
    assert lex_input('rp"/foo"') == snapshot([("String", "0..8", 'rp"/foo"')])


def test_path_fstring_literal():
    assert lex_input("pf'/foo'") == snapshot(
        [
            ("FStringStart", "0..3", "pf'"),
            ("FStringMiddle", "3..7", "/foo"),
            ("FStringEnd", "7..8", "'"),
        ]
    )
    assert lex_input('pf"/foo"') == snapshot(
        [
            ("FStringStart", "0..3", 'pf"'),
            ("FStringMiddle", "3..7", "/foo"),
            ("FStringEnd", "7..8", '"'),
        ]
    )
    assert lex_input("fp'/foo'") == snapshot(
        [
            ("FStringStart", "0..3", "fp'"),
            ("FStringMiddle", "3..7", "/foo"),
            ("FStringEnd", "7..8", "'"),
        ]
    )
    assert lex_input('fp"/foo"') == snapshot(
        [
            ("FStringStart", "0..3", 'fp"'),
            ("FStringMiddle", "3..7", "/foo"),
            ("FStringEnd", "7..8", '"'),
        ]
    )
    assert lex_input("pF'/foo'") == snapshot(
        [
            ("FStringStart", "0..3", "pF'"),
            ("FStringMiddle", "3..7", "/foo"),
            ("FStringEnd", "7..8", "'"),
        ]
    )
    assert lex_input('pF"/foo"') == snapshot(
        [
            ("FStringStart", "0..3", 'pF"'),
            ("FStringMiddle", "3..7", "/foo"),
            ("FStringEnd", "7..8", '"'),
        ]
    )
    assert lex_input("Fp'/foo'") == snapshot(
        [
            ("FStringStart", "0..3", "Fp'"),
            ("FStringMiddle", "3..7", "/foo"),
            ("FStringEnd", "7..8", "'"),
        ]
    )
    assert lex_input('Fp"/foo"') == snapshot(
        [
            ("FStringStart", "0..3", 'Fp"'),
            ("FStringMiddle", "3..7", "/foo"),
            ("FStringEnd", "7..8", '"'),
        ]
    )


def test_regex_globs():
    snaps = snapshot(
        {
            "`.*`": [("String", "0..4", "`.*`")],
            "r`.*`": [("String", "0..5", "r`.*`")],
            "g`.*`": [("String", "0..5", "g`.*`")],
            "@somethingelse`.*`": [
                ("At", "0..1", "@"),
                ("Name", "1..14", "somethingelse"),
                ("String", "14..18", "`.*`"),
            ],
            "p`.*`": [("String", "0..5", "p`.*`")],
            "pg`.*`": [("String", "0..6", "pg`.*`")],
            "`\\d*`": [("String", "0..5", "`\\d*`")],
            "r`\\d*`": [("String", "0..6", "r`\\d*`")],
            "g`\\d*`": [("String", "0..6", "g`\\d*`")],
            "@somethingelse`\\d*`": [
                ("At", "0..1", "@"),
                ("Name", "1..14", "somethingelse"),
                ("String", "14..19", "`\\d*`"),
            ],
            "p`\\d*`": [("String", "0..6", "p`\\d*`")],
            "pg`\\d*`": [("String", "0..7", "pg`\\d*`")],
            "`.*#{1,2}`": [("String", "0..10", "`.*#{1,2}`")],
            "r`.*#{1,2}`": [("String", "0..11", "r`.*#{1,2}`")],
            "g`.*#{1,2}`": [("String", "0..11", "g`.*#{1,2}`")],
            "@somethingelse`.*#{1,2}`": [
                ("At", "0..1", "@"),
                ("Name", "1..14", "somethingelse"),
                ("String", "14..24", "`.*#{1,2}`"),
            ],
            "p`.*#{1,2}`": [("String", "0..11", "p`.*#{1,2}`")],
            "pg`.*#{1,2}`": [("String", "0..12", "pg`.*#{1,2}`")],
        }
    )
    for i in (".*", r"\d*", ".*#{1,2}"):
        for p in ("", "r", "g", "@somethingelse", "p", "pg"):
            c = f"{p}`{i}`"
            assert lex_input(c) == snaps[c]


@pytest.mark.parametrize(
    ("s", "exp"),
    [
        ("", []),
        ("   \t   \n \t  ", []),
        ("echo hello", ["echo", "hello"]),
        ('echo "hello"', ["echo", '"hello"']),
        ('![echo "hello"]', ["![echo", '"hello"]']),
        ("/usr/bin/echo hello", ["/usr/bin/echo", "hello"]),
        ("$(/usr/bin/echo hello)", ["$(/usr/bin/echo", "hello)"]),
        ("C:\\Python\\python.exe -m xonsh", ["C:\\Python\\python.exe", "-m", "xonsh"]),
        ('print("""I am a triple string""")', ['print("""I am a triple string""")']),
        (
            'print("""I am a \ntriple string""")',
            ['print("""I am a \ntriple string""")'],
        ),
        ("echo $HOME", ["echo", "$HOME"]),
        ("echo -n $HOME", ["echo", "-n", "$HOME"]),
        ("echo --go=away", ["echo", "--go=away"]),
        ("echo --go=$HOME", ["echo", "--go=$HOME"]),
    ],
)
def test_lexer_split(s, exp):
    obs = Parser(s).split()
    assert exp == obs


@pytest.mark.parametrize(
    "s",
    [
        "()",  # sanity
        "(",
        ")",
        "))",
        "'string\nliteral",
        "'''string\nliteral",
        "string\nliteral'",
        '"',
        "'",
        '"""',
    ],
)
def test_tolerant_lexer(s):
    tokens = lex_input(s, tolerant=True)

    error_tokens = list(tok for tok in tokens if tok[0] == "ERRORTOKEN")
    assert all(tok[-1] in s for tok in error_tokens)  # no error messages
