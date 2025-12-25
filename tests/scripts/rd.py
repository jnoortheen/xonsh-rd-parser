from xonsh_rd_parser import get_big_py_file, Parser

file_path = get_big_py_file()
_tree = Parser.parse_file(file_path)
