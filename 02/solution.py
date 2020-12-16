from dataclasses import dataclass
import re
from typing import Union

# 16-18 h: hhhhhhhhhhhhhhhhhh
LINE_REGEX = re.compile(
    r"(?P<pos1>\d+)-(?P<pos2>\d+) (?P<letter>[a-z]): (?P<password>[a-z]+)"
)


@dataclass(frozen=True)
class InputData:
    pos1: int
    pos2: int
    letter: str
    password: str


def parse_line(line):
    match_data = LINE_REGEX.match(line).groupdict()
    return InputData(
        int(match_data["pos1"]) - 1,
        int(match_data["pos2"]) - 1,
        match_data["letter"],
        match_data["password"],
    )


def is_valid_password(input: InputData):
    return [input.password[input.pos1], input.password[input.pos2]].count(
        input.letter
    ) == 1


input_data = []

with open("input.txt") as f:
    for line in f:
        input_data.append(parse_line(line.strip()))

valid_inputs = [input for input in input_data if is_valid_password(input)]

print(f"There are {len(valid_inputs)} valid passwords.")