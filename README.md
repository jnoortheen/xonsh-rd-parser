# Xonsh Recursive Descent Parser

A Rust based, Python recursive descent parser for [Xonsh](https://xon.sh).

# Usage

- install it with pip

```
pip install xonsh-rd-parser
```

```py
from xonsh_rd_parser import parse_string
parse_string("print($HOME)")
```

# Credits

This library is based on [ruff](https://github.com/charliermarsh/ruff)'s own [Python parser](https://github.com/astral-sh/ruff/tree/main/crates/ruff_python_parser).
