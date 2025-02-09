import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2022 Day 8 Part 1

    >>> part1(['30373', '25512', '65332', '33549', '35390'])
    21
    """

    lines = [[int(x) for x in line] for line in data]

    visible = set()
    Y = len(lines)
    X = len(lines[0])

    for r in range(Y):
        val = lines[r][0]
        for c in range(X):
            if c == 0 or lines[r][c] > val:
                visible.add((r, c))
                val = lines[r][c]

    for r in range(Y):
        val = lines[r][-1]
        for c in reversed(range(X)):
            if c == X - 1 or lines[r][c] > val:
                visible.add((r, c))
                val = lines[r][c]

    for c in range(X):
        val = lines[0][c]
        for r in range(Y):
            if r == 0 or lines[r][c] > val:
                visible.add((r, c))
                val = lines[r][c]

    for c in range(X):
        val = lines[-1][c]
        for r in reversed(range(Y)):
            if r == X - 1 or lines[r][c] > val:
                visible.add((r, c))
                val = lines[r][c]

    return len(visible)


def part2(data):
    """ 2022 Day 8 Part 2

    >>> part2(['30373', '25512', '65332', '33549', '35390'])
    8
    """

    lines = [[int(x) for x in line] for line in data]
    Y = len(lines)
    X = len(lines[0])

    maxScore = 0
    for y in range(1, Y - 1):
        for x in range(1, X - 1):
            val = lines[y][x]
            score = 1

            count = 0
            for r in range(y + 1, Y):
                count += 1 if lines[r][x] < val else 0
                if lines[r][x] >= val:
                    count += 1
                    break

            score *= count

            count = 0
            for r in reversed(range(0, y)):
                count += 1 if lines[r][x] < val else 0
                if lines[r][x] >= val:
                    count += 1
                    break

            score *= count

            count = 0
            for c in range(x + 1, X):
                count += 1 if lines[y][c] < val else 0
                if lines[y][c] >= val:
                    count += 1
                    break

            score *= count

            count = 0
            for c in reversed(range(0, x)):
                count += 1 if lines[y][c] < val else 0
                if lines[y][c] >= val:
                    count += 1
                    break

            score *= count

            maxScore = max(score, maxScore)

    return maxScore


def main(input_path: Optional[Path | str]=None, verbose: bool=False) -> Tuple[Tuple[Any, float]]:
    if not input_path:
        if not (input_path := sys.argv[1] if len(sys.argv) > 1 else None):
            year, day = re.findall(r'\d+', str(__file__))[-2:]
            input_path = Path(Path(__file__).parent.parent.parent, "Inputs", f"{year}_{day}.txt")
    
    with open(input_path, encoding='UTF-8') as f:
        data = [line.strip('\n') for line in f.readlines()]

    with Timer() as p1_time:
        p1 = part1(data)

    if verbose:
        print(f"\nPart 1:\nVisible Trees: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nHighest Scenic Score: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)