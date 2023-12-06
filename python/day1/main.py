INPUT_FILE = "input.sf"
DIGITS = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]


def digit_in_line(line):
    for c in line:
        if c.isdigit():
            first = int(c)
            break
    for c in reversed(line):
        if c.isdigit():
            last = int(c)
            break
    return 10 * first + last


def fmt_line(line):
    for i, c in enumerate(line):
        for j in range(0, len(DIGITS), 1):
            if line[i : i + len(DIGITS[j])] == DIGITS[j]:
                line = line[:i] + str(j + 1) + line[i + 1 :]
    return line


f = open(INPUT_FILE)
lines = f.readlines()

print("part1:", sum(map(digit_in_line, lines)))
print("part2:", sum(map(digit_in_line, map(fmt_line, lines))))

f.close()
