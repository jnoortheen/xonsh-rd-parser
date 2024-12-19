# Xonsh Recursive Descent Parser

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

# Credits

This library is based on [ruff](https://github.com/charliermarsh/ruff)'s own [Python parser](https://github.com/astral-sh/ruff/tree/main/crates/ruff_python_parser).
