from ast import AST

class Token:
    kind: str
    start: int
    end: int

def parse_file(path: str) -> AST: ...
def parse_string(src: str) -> AST: ...
def lex_string(src: str) -> list[Token]: ...
