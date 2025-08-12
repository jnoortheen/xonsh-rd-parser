# Xonsh Recursive Descent Parser

[![PyPI version](https://img.shields.io/pypi/v/xonsh-rd-parser.svg)](https://pypi.org/project/xonsh-rd-parser/)
[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/jnoortheen/xonsh-rd-parser)

A Rust based, recursive descent parser for [Xonsh](https://xon.sh).

# Usage

- install it with pip

```
pip install xonsh-rd-parser
```

- Use it to parse Xonsh CFG
```py
from xonsh_rd_parser import Parse
Parse("print($HOME)").parse()
```

- Run `xonsh` with `env XONSH_RD_PARSER=1 xonsh` to use the new parser.

# Credits

This library is based on [ruff](https://github.com/charliermarsh/ruff)'s own [Python parser](https://github.com/astral-sh/ruff/tree/main/crates/ruff_python_parser).
