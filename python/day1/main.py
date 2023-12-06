INPUT_FILE = "input.sf"
DIGITS = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]


def digit_in_line(line):
    first = 0
    last = 0
    for i in range(0, len(line), 1):
        if line[i].isdigit():
            first = ord(line[i]) - ord("0")
            break
    for i in range(len(line) - 1, -1, -1):
        if line[i].isdigit():
            last = ord(line[i]) - ord("0")
            break
    return 10 * first + last


def fmt_line(line):
    list_line = list(line)
    for i in range(0, len(line), 1):
        for j in range(0, len(DIGITS), 1):
            if line[i : i + len(DIGITS[j])] == DIGITS[j]:
                list_line[i] = chr(j + 1 + ord("0"))
    return "".join(list_line)


f = open(INPUT_FILE)
lines = f.readlines()

print("part1:", sum(map(digit_in_line, lines)))
print("part2:", sum(map(digit_in_line, map(fmt_line, lines))))
