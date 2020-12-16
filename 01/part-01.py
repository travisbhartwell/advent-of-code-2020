with open("input.txt") as f:
    numbers = set([int(s.strip()) for s in f.readlines()])

for number in sorted(numbers):
    potential_pair = 2020 - number
    if potential_pair in numbers:
        print(f"{number},{potential_pair},{number * potential_pair}")
        break
