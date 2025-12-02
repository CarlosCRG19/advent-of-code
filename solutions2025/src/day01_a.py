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
        dial -= distance
        if dial < 0:
            dial = 100 + dial
            dial = dial % 100
    else:
        dial += distance
        dial = dial % 100

    if dial == 0:
        total_zeros += 1

print(total_zeros)
