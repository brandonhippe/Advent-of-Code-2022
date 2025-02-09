import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict


def part1(data):
    """ 2022 Day 22 Part 1

    >>> part1(['        ...#', '        .#..', '        #...', '        ....', '...#.......#', '........#...', '..#....#....', '..........#.', '        ...#....', '        .....#..', '        .#......', '        ......#.', '', '10R5L5R10L4R5L5'])
    6032
    """

    walls = set()
    area = set()
    start = None
    for y, line in enumerate(data):
        if len(line) == 0:
            break

        for x, l in enumerate(line):
            if l == '.':
                if y == 0 and start is None:
                    start = (x, y)

                area.add((x, y))
            elif l == '#':
                walls.add((x, y))

    wrapAround = defaultdict(dict)
    for pos in area:
        for offset in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
            nPos = tuple(p + o for p, o in zip(pos, offset))

            if nPos in area or nPos in walls:
                continue

            nPos = tuple(p - o for p, o in zip(pos, offset))
            while nPos in area or nPos in walls:
                nPos = tuple(p - o for p, o in zip(nPos, offset))

            wrapAround[pos][offset] = tuple(p + o for p, o in zip(nPos, offset))

    pos = start[:]
    facing = (1, 0)
    inProgress = ''
    for c in data[-1] + 'R':
        if c in 'LR':
            amt = int(inProgress)
            inProgress = ''

            while amt != 0:
                if facing in wrapAround[pos]:
                    newPos = wrapAround[pos][facing]
                else:
                    newPos = tuple(p + o for p, o in zip(pos, facing))

                if newPos in walls:
                    break

                pos = newPos
                amt -= 1

            if c == 'L':
                facing = (facing[1], -facing[0])
            elif c == 'R':
                facing = (-facing[1], facing[0])
        else:
            inProgress += c
    
    facing = (facing[1], -facing[0])
    facingScore = {(1, 0): 0, (0, 1): 1, (-1, 0): 2, (0, -1): 3}

    return (1000 * (pos[1] + 1)) + (4 * (pos[0] + 1)) + facingScore[facing]


def part2(data):
    """ 2022 Day 22 Part 2
    """

    walls = set()
    area = set()
    start = None
    for y, line in enumerate(data):
        if len(line) == 0:
            break

        for x, l in enumerate(line):
            if l == '.':
                if y == 0 and start is None:
                    start = (x, y)

                area.add((x, y))
            elif l == '#':
                walls.add((x, y))

    # RIGHT: (1, 0), LEFT: (-1, 0), UP: (0, -1), DOWN: (0, 1)
    wrapAround = defaultdict(dict)
    for i in range(0, 50):
        wrapAround[(i, 199)][(0, 1)] = [(i + 100, 0), (0, 1)]
        wrapAround[(149, i)][(1, 0)] = [(99, 149 - i), (-1, 0)]
        wrapAround[(i, 100)][(0, -1)] = [(50, i + 50), (1, 0)]
        wrapAround[(50, i)][(-1, 0)] = [(0, 149 - i), (1, 0)]

    for i in range(50, 100):
        wrapAround[(i, 0)][(0, -1)] = [(0, 100 + i), (1, 0)]
        wrapAround[(i, 149)][(0, 1)] = [(49, 100 + i), (-1, 0)]
        wrapAround[(99, i)][(1, 0)] = [(i + 50, 49), (0, -1)]
        wrapAround[(50, i)][(-1, 0)] = [(i - 50, 100), (0, 1)]

    for i in range(100, 150):
        wrapAround[(i, 0)][(0, -1)] = [(i - 100, 199), (0, -1)]
        wrapAround[(99, i)][(1, 0)] = [(149, 49 - (i - 100)), (-1, 0)]
        wrapAround[(i, 49)][(0, 1)] = [(99, i - 50), (-1, 0)]
        wrapAround[(0, i)][(-1, 0)] = [(50, 49 - (i - 100)), (1, 0)]

    for i in range(150, 200):
        wrapAround[(49, i)][(1, 0)] = [(i - 100, 149), (0, -1)]
        wrapAround[(0, i)][(-1, 0)] = [(i - 100, 0), (0, 1)]

    pos = start[:]
    facing = (1, 0)
    inProgress = ''
    for c in data[-1] + 'R':
        if c in 'LR':
            amt = int(inProgress)
            inProgress = ''

            while amt != 0:
                if facing in wrapAround[pos]:
                    newPos, newFacing = wrapAround[pos][facing]
                else:
                    newPos = tuple(p + o for p, o in zip(pos, facing))
                    newFacing = facing

                if newPos in walls:
                    break

                pos = newPos
                facing = newFacing
                amt -= 1

            if c == 'L':
                facing = (facing[1], -facing[0])
            elif c == 'R':
                facing = (-facing[1], facing[0])
        else:
            inProgress += c
    
    facing = (facing[1], -facing[0])
    facingScore = {(1, 0): 0, (0, 1): 1, (-1, 0): 2, (0, -1): 3}

    return (1000 * (pos[1] + 1)) + (4 * (pos[0] + 1)) + facingScore[facing]


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
        print(f"\nPart 1:\nPassword: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nPassword: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)