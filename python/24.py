import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict


def part1(data):
    """ 2022 Day 24 Part 1

    >>> part1(['#.######', '#>>.<^<#', '#.<..<<#', '#>v.><>#', '#<^v^^>#', '######.#'])
    18
    """

    blizzards = defaultdict(list)
    walls = set()
    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l in DIR_MAP:
                blizzards[(x, y)].append(DIR_MAP[l])
            elif l == '#':
                walls.add((x, y))

    start = (data[0].index('.'), 0)
    end = (data[-1].index('.'), len(data) - 1)

    walls.add((start[0], start[1] - 1))
    walls.add((end[0], end[1] + 1))

    return bfs(start, end, blizzards, walls)[0]


def part2(data):
    """ 2022 Day 24 Part 2

    >>> part2(['#.######', '#>>.<^<#', '#.<..<<#', '#>v.><>#', '#<^v^^>#', '######.#'])
    54
    """

    blizzards = defaultdict(list)
    walls = set()
    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l in DIR_MAP:
                blizzards[(x, y)].append(DIR_MAP[l])
            elif l == '#':
                walls.add((x, y))

    start = (data[0].index('.'), 0)
    end = (data[-1].index('.'), len(data) - 1)

    walls.add((start[0], start[1] - 1))
    walls.add((end[0], end[1] + 1))

    p1, blizzards = bfs(start, end, blizzards, walls)
    r1, blizzards = bfs(end, start, blizzards, walls)
    r2, blizzards = bfs(start, end, blizzards, walls)
    return p1 + r1 + 1 + r2 + 1


DIR_MAP = {'>': (1, 0), '<': (-1, 0), '^': (0, -1), 'v': (0, 1)}


def moveBlizzards(blizzards, walls):
    mins = [min(p[i] for p in walls) for i in range(2)]
    maxs = [max(p[i] for p in walls) for i in range(2)]

    mins[1] += 1
    maxs[1] -= 1

    newBlizzards = defaultdict(list)
    for b in blizzards.keys():
        for d in blizzards[b]:
            newB = tuple(p + o for p, o in zip(b, d))

            if newB[0] == mins[0]:
                newB = (maxs[0] - 1, newB[1])
            
            if newB[0] == maxs[0]:
                newB = (mins[0] + 1, newB[1])

            if newB[1] == mins[1]:
                newB = (newB[0], maxs[1] - 1)
            
            if newB[1] == maxs[1]:
                newB = (newB[0], mins[1] + 1)

            newBlizzards[newB].append(d)

    return newBlizzards


def bfs(start, end, blizzards, walls):
    states = {start}

    steps = 0
    while True:
        blizzards = moveBlizzards(blizzards, walls)
        newStates = set()
        for pos in states:
            if pos == end:
                return steps, blizzards

            for offset in [(1, 0), (-1, 0), (0, 1), (0, -1), (0, 0)]:
                newPos = tuple(p + o for p, o in zip(pos, offset))

                if newPos not in walls and newPos not in blizzards:
                    newStates.add(newPos)

        states = newStates
        steps += 1


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
        print(f"\nPart 1:\nFewest minutes to reach goal: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nFewest minutes to reach goal: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)