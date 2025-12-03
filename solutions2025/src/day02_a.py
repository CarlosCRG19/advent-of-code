import argparse
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument("path", help="Relative path to the input file")
args = parser.parse_args()

file_path = Path(args.path)

ranges = []

with file_path.open() as f:
    line = f.read()
    ranges_str = line.strip().split(',')
    for range_str in ranges_str:
        a, b = range_str.split('-')
        ranges.append([int(a), int(b)])

answer = 0
for r in ranges:
    for id in range(r[0], r[1] + 1):
        id_str = str(id)
        if id_str[:len(id_str)//2] == id_str[len(id_str)//2:]:
            answer += id

print(answer)
