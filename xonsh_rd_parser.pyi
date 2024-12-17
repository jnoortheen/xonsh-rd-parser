from ast import AST

class Token:
    start: int
    end: int

    @property
    def type(self) -> str: ...
    @property
    def kind(self) -> str: ...

class Parser:
    def __init__(self, src: str, file_name: str | None = None) -> None: ...
    def tokens(self) -> list[Token]: ...
    def subproc_toks(
        self,
        returnline: bool = False,
        greedy: bool = False,
        mincol: int = -1,
        maxcol: int | None = None,
    ) -> str | None: ...
    @staticmethod
    def parse_file(path: str) -> AST: ...
    def parse(self) -> AST: ...
