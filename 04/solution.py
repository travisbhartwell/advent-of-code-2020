from dataclasses import dataclass
from enum import Enum
import itertools
import re
import sys
from typing import Any


def valid_byr(value):
    int_value = int(value)
    return 1920 <= int_value <= 2002


def valid_iyr(value):
    int_value = int(value)
    return 2010 <= int_value <= 2020


def valid_eyr(value):
    int_value = int(value)
    return 2020 <= int_value <= 2030


def valid_hgt(value):
    valid_re = re.compile(r"^(?P<amount>\d+)(?P<units>\w+)$")
    match = valid_re.match(value)

    if match:
        matched_values = match.groupdict()

        if matched_values["units"] == "cm":
            amount = int(matched_values["amount"])
            return 150 <= amount <= 193
        elif matched_values["units"] == "in":
            amount = int(matched_values["amount"])
            return 59 <= amount <= 76
    else:
        return False


def valid_hcl(value):
    return re.match(r"^#[0-9a-f]{6}$", value) is not None


POSSIBLE_ECL = frozenset(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"])


def valid_ecl(value):
    return value in POSSIBLE_ECL


def valid_pid(value):
    return re.match(r"^\d{9}$", value) is not None


def valid_cid(value):
    return True


class FieldType(Enum):
    BYR = ("byr", "Birth Year")
    IYR = ("iyr", "Issue Year")
    EYR = ("eyr", "Expiration Year")
    HGT = ("hgt", "Height")
    HCL = ("hcl", "Hair Color")
    ECL = ("ecl", "Eye Color")
    PID = ("pid", "Passport ID")
    CID = ("cid", "Country ID")

    def __init__(self, code: str, description: str):
        self.code = code
        self.description = description

    @classmethod
    def _missing_(cls, value: object) -> Any:
        for item in cls:
            if value == item.code:
                return item
        return super()._missing_(value)

    def validate(self, value):
        validator = globals()[f"valid_{self.code}"]
        return validator(value)


VALID_FIELDS = frozenset(FieldType.__members__.values())


@dataclass(frozen=True)
class PassportField:
    field_type: FieldType
    data: str

    def is_valid(self):
        return self.field_type.validate(self.data)

    def __str__(self):
        return f"{self.field_type.description}: {self.data}"

    @classmethod
    def from_field_data(cls, field_data):
        try:
            field_name, value = field_data.split(":")
            field_type = FieldType(field_name)
            return cls(field_type, value)
        except ValueError:
            print(
                f"Unknown field type {field_name} in field data '{datum}' on line '{line}'"
            )


@dataclass(frozen=True)
class Passport:
    fields: PassportField

    def are_required_fields_missing(self):
        fields_present = set(field.field_type for field in self.fields)

        missing = VALID_FIELDS - fields_present
        if len(missing) == 0:
            return False
        elif len(missing) == 1 and FieldType.CID in missing:
            return False
        else:
            return True

    def is_valid(self):
        if self.are_required_fields_missing():
            return False
        else:
            return all([field.is_valid() for field in self.fields])

    def __str__(self):
        return ", ".join(
            str(field) for field in sorted(self.fields, key=lambda a: a.field_type.code)
        )

    @classmethod
    def from_field_data(cls, passport_data):
        the_fields = [
            PassportField.from_field_data(field_data) for field_data in passport_data
        ]

        return cls(the_fields)


def parse_data(lines):
    passports_data = [
        " ".join(list(group)).split()
        for is_separator, group in itertools.groupby(lines, lambda l: l == "")
        if not is_separator
    ]

    return [Passport.from_field_data(passport_data) for passport_data in passports_data]


def main(filename="input.txt"):
    with open(filename) as f:
        lines = [line.strip() for line in f]

    passports = parse_data(lines)

    valid = [passport for passport in passports if passport.is_valid()]

    print(f"Number of valid passports: {len(valid)}")

    for passport in valid:
        print(passport)


if __name__ == "__main__":
    if len(sys.argv) > 1:
        main(sys.argv[1])
    else:
        main()