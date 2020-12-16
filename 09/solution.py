import sys


def exists_in_range(value, start, end, values_to_pos):
    pos = values_to_pos.get(value, -1)

    return (start <= pos < end)

def validate_input(input_contents, preamable_length):
    invalid_numbers = []
    val_to_pos = dict(enumerate(input_contents))

    for validate_index in range(preamable_length, len(input_contents) + 1):
        valid = False
        start = validate_index - preamable_length
        preamble = input_contents[start:validate_index]
        target = input_contents[validate_index]

        for el in preamble:
            if el > target:
                continue
            pair = target - el
            if exists_in_range(pair, start, validate_index, val_to_pos):
                valid = True
                break

        if not valid:
            invalid_numbers.append(target)

        return invalid_numbers

def load_contents(input_filename):
    with open(input_filename) as f:
        return [int(line.strip()) for line in f]

def main(input_filename, preamable_length):
    input_contents = load_contents(input_filename)

    invalid_numbers = validate_input(input_contents, preamable_length)

    for invalid in invalid_numbers:
        print(f"Found invalid value {invalid}.")

if __name__ == "__main__":
    input_filename = "input.txt"
    preamable_length = 25

    if len(sys.argv) > 1:
        input_filename = sys.argv[1]

    if len(sys.argv) > 2:
        preamable_length = int(sys.argv[2])

    main(input_filename, preamable_length)