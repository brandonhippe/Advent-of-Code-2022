import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from collections import deque


def part1(data):
    """ 2022 Day 18 Part 1

    >>> part1(['2,2,2', '1,2,2', '3,2,2', '2,1,2', '2,3,2', '2,2,1', '2,2,3', '2,2,4', '2,2,6', '1,2,5', '3,2,5', '2,1,5', '2,3,5'])
    64
    """

    cubes = set(tuple(int(x) for x in re.findall('-?\d+', line)) for line in data)

    cubeFaces = {c: [True] * 6 for c in cubes}
    for c in cubes:
        for i, offset in enumerate([[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]]):
            if tuple(p + o for p, o in zip(c, offset)) in cubeFaces:
                cubeFaces[tuple(p + o for p, o in zip(c, offset))][i] = False
    
    return sum(sum(cF) for cF in cubeFaces.values())


def part2(data):
    """ 2022 Day 18 Part 2

    >>> part2(['2,2,2', '1,2,2', '3,2,2', '2,1,2', '2,3,2', '2,2,1', '2,2,3', '2,2,4', '2,2,6', '1,2,5', '3,2,5', '2,1,5', '2,3,5'])
    58
    """

    cubes = set(tuple(int(x) for x in re.findall('-?\d+', line)) for line in data)

    cubeFaces = {c: [True] * 6 for c in cubes}
    for c in cubes:
        for i, offset in enumerate([[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]]):
            if tuple(p + o for p, o in zip(c, offset)) in cubeFaces:
                cubeFaces[tuple(p + o for p, o in zip(c, offset))][i] = False

    possible = set()
    for c in cubes:
        for offset in [[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]]:
            oCube = tuple(p + o for p, o in zip(c, offset))
            if oCube not in cubes and oCube not in possible:
                possible.add(oCube)

    mins = tuple(min(c[i] for c in cubes) - 1 for i in range(3))
    maxs = tuple(max(c[i] for c in cubes) + 1 for i in range(3))

    outside = possible.intersection(bfs(mins, mins, maxs, cubes))
    possible = possible.difference(outside)

    for c in possible:
        for i, offset in enumerate([[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]]):
            if tuple(p + o for p, o in zip(c, offset)) in cubeFaces:
                cubeFaces[tuple(p + o for p, o in zip(c, offset))][i] = False

    return sum(sum(cF) for cF in cubeFaces.values())


def bfs(start, mins, maxs, cubes):
    openList = deque([start])
    openSet = {start}
    visited = set()

    while len(openList) != 0:
        pos = openList.pop()

        if pos in openSet:
            openSet.remove(pos)
        else:
            continue

        for offset in [[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]]:
            oCube = tuple(p + o for p, o in zip(pos, offset))
            if oCube not in cubes and oCube not in openSet and oCube not in visited and all(oCube[i] >= mins[i] and oCube[i] <= maxs[i] for i in range(3)):
                openList.appendleft(oCube)
                openSet.add(oCube)

        visited.add(pos)

    return visited


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
        print(f"\nPart 1:\nTotal surface area: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nExterior surface area: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)