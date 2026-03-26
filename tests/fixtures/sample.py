import os
from typing import Optional

def greet(name: str) -> str:
    return f"Hello, {name}!"

def add(a: int, b: int) -> int:
    return a + b

MAX_SIZE = 1024

class Config:
    default_name = "untitled"

    def __init__(self, name: str, value: int = 0):
        self.name = name
        self.value = value

    def get_name(self) -> str:
        return self.name

    @staticmethod
    def create(name: str) -> "Config":
        return Config(name)

@decorator
def decorated(x: int) -> int:
    return x * 2
