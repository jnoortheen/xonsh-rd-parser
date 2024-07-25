def test_parse_string(benchmark):
    @benchmark
    def main():
        import xonsh_rd_parser as parser

        src_txt = "print(1)"
        return parser.parse_string(src_txt)
