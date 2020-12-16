from dataclasses import dataclass
import re
from typing import Union

# 16-18 h: hhhhhhhhhhhhhhhhhh
LINE_REGEX = re.compile(
    r"(?P<count_min>\d+)-(?P<count_max>\d+) (?P<letter>[a-z]): (?P<password>[a-z]+)"
)


@dataclass(frozen=True)
class InputData:
    count_min: int
    count_max: int
    letter: str
    password: str


def parse_line(line):
    match_data = LINE_REGEX.match(line).groupdict()
    return InputData(
        int(match_data["count_min"]),
        int(match_data["count_max"]),
        match_data["letter"],
        match_data["password"],
    )


def is_valid_password(input: InputData):
    count = input.password.count(input.letter)
    return input.count_min <= count <= input.count_max


input_data = []

with open("input.txt") as f:
    for line in f:
        input_data.append(parse_line(line.strip()))

valid_inputs = [input for input in input_data if is_valid_password(input)]

print(f"There are {len(valid_inputs)} valid passwords.")