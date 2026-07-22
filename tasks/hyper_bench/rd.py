import os

import xonsh_rd_parser as xrd

assert not xrd.is_debug_build(), "Debug build is being used for benchmarking"
file_path = os.path.join(os.path.dirname(__file__), ".fixture.py")
_tree = xrd.Parser.parse_file(file_path)
