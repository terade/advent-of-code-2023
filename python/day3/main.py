INPUT_FILE = "input.sf"


class Engine:
    def __init__(self):
        self.part_numbers = {}
        self.parts = []
        self.part_id = 0
        self.part1 = None
        self.part2 = None

    def input_sequence(self, point, sequence):
        line, begin = point
        pid, word = sequence
        for i in range(0, len(word)):
            self.part_numbers[(line, begin + i)] = (pid, int(word))

    def parse_line(self, line_number, line):
        digit = ""
        begin = None
        for i, c in enumerate(line):
            if c.isdigit():
                if not digit:
                    begin = i
                digit += c
                continue
            if c != ".":
                self.parts.append((line_number, i))
            if digit:
                self.input_sequence((line_number, begin), (self.part_id, digit))
                self.part_id += 1
                digit = ""
        if digit:
            self.input_sequence((line_number, begin), (self.part_id, digit))

    def parse(self, string):
        for i, line in enumerate(string.splitlines()):
            self.parse_line(i, line)

    def __str__(self):
        return f"part_numbers: {self.part_numbers}\nparts: {self.parts}"

    def get_adjacent(self, point):
        adj = []
        line, offset = point
        for i in range(3):
            for j in range(3):
                adj.append((line + i - 1, offset + j - 1))
        return adj

    def get_part_numbers(self, adj):
        return list(
            set(
                filter(
                    lambda e: e is not None,
                    map(lambda x: self.part_numbers.get(x, None), adj),
                )
            )
        )

    def solve(self):
        self.part1 = 0
        self.part2 = 0
        for elem in self.parts:
            adj = self.get_adjacent(elem)
            numbers = self.get_part_numbers(adj)
            self.part1 += sum(map(lambda x: x[1], numbers))
            if len(numbers) == 2:
                self.part2 += numbers[0][1] * numbers[1][1]
        return (self.part1, self.part2)


with open(INPUT_FILE) as file:
    engine = Engine()
    engine.parse(file.read())
    part1, part2 = engine.solve()
    print("part1:", part1)
    print("part2:", part2)
