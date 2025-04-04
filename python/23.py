import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple
from collections import deque
from itertools import product

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict


def move_elves(elves: set[tuple[int,]], MOVE_ORDER) -> tuple[set[tuple[int,]], bool]:
    """Move elves based on the given MOVE_ORDER."""
    proposed = defaultdict(set)
    spot_counts = defaultdict(int)
    for elf in filter(lambda e: any(offset != (0, 0) and tuple(p + o for p, o in zip(e, offset)) in elves for offset in product(range(-1, 2), repeat=2)), elves):
        for checks, move in MOVE_ORDER:
            if any(tuple(p + o for p, o in zip(elf, offset)) in elves for offset in checks):
                continue

            spot_counts[tuple(p + o for p, o in zip(elf, move))] += 1
            proposed[elf] = tuple(p + o for p, o in zip(elf, move))
            break

    new_elves = set()
    moved = False
    for elf in elves:
        if elf in proposed:
            new_pos = proposed[elf]
            if spot_counts[new_pos] == 1:
                moved = True
                new_elves.add(new_pos)
            else:
                new_elves.add(elf)
        else:
            new_elves.add(elf)

    return new_elves, moved


def part1(data):
    """ 2022 Day 23 Part 1

    >>> part1(['....#..', '..###.#', '#...#.#', '.#...##', '#.###..', '##.#.##', '.#..#..'])
    110
    """

    elves = set()

    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l == '#':
                elves.add((x, y))

    MOVE_ORDER = deque([[[[-1, -1], [0, -1], [1, -1]], [0, -1]], [[[-1, 1], [0, 1], [1, 1]], [0, 1]], [[[-1, -1], [-1, 0], [-1, 1]], [-1, 0]], [[[1, -1], [1, 0], [1, 1]], [1, 0]]])

    round = 1
    moved = True
    while moved and round <= 10:
        elves, moved = move_elves(elves, MOVE_ORDER)
        MOVE_ORDER.rotate(-1)
        round += 1

    mins = [min(e[i] for e in elves) for i in range(2)]
    maxs = [max(e[i] for e in elves) for i in range(2)]
    return (maxs[0] - mins[0] + 1) * (maxs[1] - mins[1] + 1) - len(elves)


def part2(data):
    """ 2022 Day 23 Part 2

    >>> part2(['....#..', '..###.#', '#...#.#', '.#...##', '#.###..', '##.#.##', '.#..#..'])
    20
    """

    elves = set()

    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l == '#':
                elves.add((x, y))

    MOVE_ORDER = deque([[[[-1, -1], [0, -1], [1, -1]], [0, -1]], [[[-1, 1], [0, 1], [1, 1]], [0, 1]], [[[-1, -1], [-1, 0], [-1, 1]], [-1, 0]], [[[1, -1], [1, 0], [1, 1]], [1, 0]]])

    round = 0
    moved = True
    while moved:
        elves, moved = move_elves(elves, MOVE_ORDER)
        MOVE_ORDER.rotate(-1)
        round += 1
    
    return round


def printElves(mins, maxs, elves):
    for y in range(mins[1], maxs[1] + 1):
        for x in range(mins[0], maxs[0] + 1):
            if (x, y) in elves:
                print('#', end='')
            else:
                print('.', end='')

        print('')

    print('')


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
        print(f"\nPart 1:\nEmpty ground tiles: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nFirst round where no elf moves: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)