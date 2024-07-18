import xonsh_rd_parser as parser


def test_parser():
    result =  parser.parse_module("import abc")
    assert result == b"import abc"
