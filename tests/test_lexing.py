import xonsh_rd_parser as parser


def test_parser():
    result =  parser.parse_string("import abc")
    assert result == b"import abc"
