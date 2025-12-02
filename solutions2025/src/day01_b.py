import argparse
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument("path", help="Relative path to the input file")
args = parser.parse_args()

file_path = Path(args.path)

instructions = []

with file_path.open() as f:
    for line in f:
        line = line.strip()
        if not line:
            continue
        instructions.append((line[0], int(line[1:])))

dial, total_zeros = 50, 0
for instruction in instructions:
    direction, distance = instruction
    if direction == "L":
        for i in range(distance):
            dial -= 1
            if dial % 100 == 0:
                total_zeros += 1

        if dial < 0:
            dial = 100 + dial
    else:
        dial += distance
        if dial == 0:
            total_zeros += 1
        else:
            total_zeros += dial // 100

    dial = dial % 100

print(total_zeros)

