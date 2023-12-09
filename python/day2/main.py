import re

INPUT_FILE = "input.sf"


def part1(lines):
    sum = 0
    constraints = {"red": 12, "green": 13, "blue": 14}
    for id, line in enumerate(lines, start=1):
        if all(
            map(
                lambda elem: int(elem[0]) <= constraints[elem[1]],
                re.findall(r"(\d*) (red|green|blue)", line),
            )
        ):
            sum += id
    return sum


def part2(lines):
    sum = 0
    for id, line in enumerate(lines, start=1):
        max = {"red": 0, "green": 0, "blue": 0}
        for amount, color in re.findall(r"(\d*) (red|green|blue)", line):
            if int(amount) > max[color]:
                max[color] = int(amount)
        sum += max["red"] * max["green"] * max["blue"]
    return sum


f = open(INPUT_FILE)
lines = f.readlines()

print("part1:", part1(lines))
print("part2:", part2(lines))

f.close()
