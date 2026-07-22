from pathlib import Path

from xonsh.parsers.v310 import Parser

file_path = Path(__file__).parent / ".fixture.py"
p = Parser()
p.parse(Path(file_path).read_text())
