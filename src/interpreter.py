from sys import argv, stdin
from pathlib import Path
from enum import StrEnum, auto


class OwnPy:
    def __init__(self):
        self.had_error = False

    def main(self) -> None:
        if len(argv) > 2:
            print("Usage: ownpy [script]")
            exit(64)
        elif len(argv) == 2:
            self.run_file(argv[1])
        else:
            self.run_prompt()

    def run_file(self, path: str) -> None:
        path: Path = Path(path)
        bytes = ""
        if not path.exists():
            raise FileNotFoundError(f"Cannot find file at {path}")
        with open(path, "r") as f:
            source = f.read()
        self.run(source)
        if self.had_error:
            exit(65)

    def run_prompt(self) -> None:
        for line in stdin:
            if line is None:
                break
            self.run(line)
            self.had_error = False

    def run(self, source: str) -> None:
        parser = Parser(source)
        tokens = parser.parse_tokens()
        for token in tokens:
            print(token)

    def error(self, line: int, message: str):
        self.report(line, "", message)

    def report(self, line: int, where: str, message: str):
        print(f"f[line {line}] Error {where}: {message}")


class TokenType(StrEnum):
    # Single-character tokens.
    LEFT_PAREN = auto()
    RIGHT_PAREN = auto()
    LEFT_BRACE = auto()
    RIGHT_BRACE = auto()
    COMMA = auto()
    DOT = auto()
    MINUS = auto()
    PLUS = auto()
    SEMICOLON = auto()
    SLASH = auto()
    STAR = auto()

    # One or two character tokens.
    BANG = auto()
    BANG_EQUAL = auto()
    EQUAL = auto()
    EQUAL_EQUAL = auto()
    GREATER = auto()
    GREATER_EQUAL = auto()
    LESS = auto()
    LESS_EQUAL = auto()

    # Literals.
    IDENTIFIER = auto()
    STRING = auto()
    NUMBER = auto()

    # Keywords.
    AND = auto()
    CLASS = auto()
    ELSE = auto()
    FALSE = auto()
    FUN = auto()
    FOR = auto()
    IF = auto()
    NIL = auto()
    OR = auto()
    PRINT = auto()
    RETURN = auto()
    SUPER = auto()
    THIS = auto()
    TRUE = auto()
    VAR = auto()
    WHILE = auto()

    EOF = auto()


class Token:
    def __init__(self, type: TokenType, lexeme: str, literal: object, line: int):
        self.type = type
        self.lexeme = lexeme
        self.literal = literal
        self.line = line

    def __str__(self):
        return f"{self.type} {self.lexeme} {self.literal}"


class Parser:
    def __init__(self, source_code: str):
        self.source_code = source_code
        self.tokens = []
        self.start = 0
        self.curr = 0
        self.line = 1

    def is_at_end(self) -> bool:
        return self.curr >= len(self.source_code)

    def parse_tokens(self):
        while not self.is_at_end():
            self.start = self.curr
            self.parse_single_token()
        self.tokens.append(Token(TokenType.EOF, "", None, self.line))
        return self.tokens()

    def next(self):
        self.curr += 1
        return self.source_code[self.curr]

    def parse_single_token(self):
        char = self.next()
        match (char):
            case "(":
                self.add_token(TokenType.LEFT_PAREN)
                return
            case ")":
                self.add_token(TokenType.RIGHT_PAREN)
            case "{":
                self.add_token(TokenType.LEFT_BRACE)
            case "}":
                self.add_token(TokenType.RIGHT_BRACE)
            case ",":
                self.add_token(TokenType.COMMA)
            case ".":
                self.add_token(TokenType.DOT)
            case "-":
                self.add_token(TokenType.MINUS)
            case "+":
                self.add_token(TokenType.PLUS)
            case ";":
                self.add_token(TokenType.SEMICOLON)
            case "*":
                self.add_token(TokenType.STAR)
            case _:
                raise ValueError("Unexpected character.")

    def add_token(self, type: TokenType, literal: object | None) -> None:
        text = self.source_code[self.start : self.curr]
        self.tokens.append(Token(type, text, literal, self.line))
