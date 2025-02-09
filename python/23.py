import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict


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

    MOVE_ORDER = [[[[-1, -1], [0, -1], [1, -1]], [0, -1]], [[[-1, 1], [0, 1], [1, 1]], [0, 1]], [[[-1, -1], [-1, 0], [-1, 1]], [-1, 0]], [[[1, -1], [1, 0], [1, 1]], [1, 0]]]

    round = 1
    moved = True
    while moved and round <= 10:
        proposed = defaultdict(set)
        moved = False
        for elf in elves:
            if all([tuple(p + o for p, o in zip(elf, offset)) not in elves for offset in [[-1, -1], [-1, 0], [-1, 1], [0, 1], [1, 1], [1, 0], [1, -1], [0, -1]]]):
                    continue

            moved = True

            for checks, move in MOVE_ORDER:
                if any([tuple(p + o for p, o in zip(elf, offset)) in elves for offset in checks]):
                    continue

                proposed[tuple(p + o for p, o in zip(elf, move))].add(elf)
                break

        for elf, pickedBy in zip(proposed.keys(), proposed.values()):
            if len(pickedBy) == 1:
                elves = elves.difference(pickedBy)
                elves.add(elf)

        MOVE_ORDER.append(MOVE_ORDER.pop(0))

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

    MOVE_ORDER = [[[[-1, -1], [0, -1], [1, -1]], [0, -1]], [[[-1, 1], [0, 1], [1, 1]], [0, 1]], [[[-1, -1], [-1, 0], [-1, 1]], [-1, 0]], [[[1, -1], [1, 0], [1, 1]], [1, 0]]]

    round = 1
    moved = True
    while moved:            
        proposed = defaultdict(set)
        moved = False
        for elf in elves:
            if all([tuple(p + o for p, o in zip(elf, offset)) not in elves for offset in [[-1, -1], [-1, 0], [-1, 1], [0, 1], [1, 1], [1, 0], [1, -1], [0, -1]]]):
                    continue

            moved = True

            for checks, move in MOVE_ORDER:
                if any([tuple(p + o for p, o in zip(elf, offset)) in elves for offset in checks]):
                    continue

                proposed[tuple(p + o for p, o in zip(elf, move))].add(elf)
                break

        for elf, pickedBy in zip(proposed.keys(), proposed.values()):
            if len(pickedBy) == 1:
                elves = elves.difference(pickedBy)
                elves.add(elf)

        MOVE_ORDER.append(MOVE_ORDER.pop(0))

        round += 1
    
    return round - 1


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