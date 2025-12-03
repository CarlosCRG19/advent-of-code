import argparse
from pathlib import Path

parser = argparse.ArgumentParser()
parser.add_argument("path", help="Relative path to the input file")
args = parser.parse_args()

file_path = Path(args.path)

ranges = []

def is_invalid_id(id: int) -> bool:
    id_str = str(id)
    length = len(id_str)
    
    # Check all possible pattern lengths from 1 to half the string length
    for pattern_len in range(1, length // 2 + 1):
        if length % pattern_len != 0:
            continue
        
        pattern = id_str[:pattern_len]
        if pattern * (length // pattern_len) == id_str:
            return True
    
    return False





with file_path.open() as f:
    line = f.read()
    ranges_str = line.strip().split(',')
    for range_str in ranges_str:
        a, b = range_str.split('-')
        ranges.append([int(a), int(b)])

answer = 0
for r in ranges:
    for id in range(r[0], r[1] + 1):
        if is_invalid_id(id):
            answer += id

print(answer)
