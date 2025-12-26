import xonsh_rd_parser as rd

assert not rd.is_debug_build(), "Debug build is being used for benchmarking"
file_path = rd.get_big_py_file()
_tree = rd.Parser.parse_file(file_path)
