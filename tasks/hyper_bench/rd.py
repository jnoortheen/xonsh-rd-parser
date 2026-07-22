import xonsh_rd_parser as xrd

assert not xrd.is_debug_build(), "Debug build is being used for benchmarking"
file_path = xrd.get_big_py_file()
_tree = xrd.Parser.parse_file(file_path)
