from xonsh_rd_parser import get_big_py_file


from xonsh.parsers.v310 import Parser
from pathlib import Path

file_path = get_big_py_file()
p = Parser()
p.parse(Path(file_path).read_text())
