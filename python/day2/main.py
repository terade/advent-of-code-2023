import re

INPUT_FILE = "input.sf"


def part1(lines):
    result = 0
    constraints = {"red": 12, "green": 13, "blue": 14}
    for id, line in enumerate(lines, start=1):
        if all(
            map(
                lambda elem: int(elem[0]) <= constraints[elem[1]],
                re.findall(r"(\d*) (red|green|blue)", line),
            )
        ):
            result += id
    return result


def part2(lines):
    result = 0
    for line in lines:
        record = {"red": 0, "green": 0, "blue": 0}
        for amount, color in re.findall(r"(\d*) (red|green|blue)", line):
            if int(amount) > record[color]:
                record[color] = int(amount)
        result += record["red"] * record["green"] * record["blue"]
    return result


f = open(INPUT_FILE)
file_lines = f.readlines()

print("part1:", part1(file_lines))
print("part2:", part2(file_lines))

f.close()
