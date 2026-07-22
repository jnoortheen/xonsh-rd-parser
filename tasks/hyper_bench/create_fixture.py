from pathlib import Path

import xonsh_rd_parser as xrd


def main():
    fixture = Path(__file__).parent / ".fixture.py"
    xrd.get_big_py_file(lines=None, file_name=fixture.as_posix())
