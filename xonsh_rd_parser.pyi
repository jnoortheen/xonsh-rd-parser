from ast import AST

class Token:
    start: int
    end: int

    @property
    def type(self) -> str: ...
    @property
    def kind(self) -> str: ...

class Lexer:
    def __init__(self, src: str, file_name: str | None = None) -> None: ...
    def tokens(self) -> list[Token]: ...
    def subproc_toks(
        self,
        returnline: bool = False,
        greedy: bool = False,
        mincol: int = -1,
        maxcol: int | None = None,
    ) -> str | None: ...

def parse_file(path: str) -> AST: ...
def parse_string(src: str, file_name: str | None = None) -> AST: ...
