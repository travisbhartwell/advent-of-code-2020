import itertools
import math


def find_sum(target_sum, element_count, numbers):
    all_permutations = itertools.combinations(numbers, element_count)

    return set(
        frozenset(permutation)
        for permutation in all_permutations
        if sum(permutation) == target_sum
    )


def find_sum_rec(target_sum, operand_count, numbers):
    # print(f"target sum: {target_sum}, operand_count: {operand_count}, numbers: {numbers}")
    sum_operands = []

    for index, val in enumerate(numbers):
        if operand_count == 1 and val == target_sum:
            sum_operands.append([val])
        elif val > target_sum:
            continue
        elif index < (len(numbers) - 1):
            found_sub = find_sum_rec(target_sum - val, operand_count - 1, numbers[index:])
            if found_sub:
                sum_operands.extend([[val, *operands] for operands in found_sub])

    return sum_operands



with open("input.txt") as f:
    numbers = set([int(s.strip()) for s in f.readlines()])

numbers_list = list(numbers)

matching = find_sum_rec(2020, 2, numbers_list)
for group in matching:
    product = math.prod(group)
    items = list(group)
    print(f"{items[0]},{items[1]},{product}")

matching = find_sum_rec(2020, 3, numbers_list)

for group in matching:
    product = math.prod(group)
    items = list(group)
    print(f"{items[0]},{items[1]},{items[2]},{product}")
