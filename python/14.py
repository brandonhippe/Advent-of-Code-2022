import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data):
    """ 2022 Day 14 Part 1

    >>> part1(['498,4 -> 498,6 -> 496,6', '503,4 -> 502,4 -> 502,9 -> 494,9'])
    24
    """

    area = set()
    for line in data:
        nums = [int(x) for x in re.findall("\d+", line)]

        pos = tuple(nums[:2])

        for i in range(2, len(nums), 2):
            offset = [(abs(n - p) // (n - p)) if n - p != 0 else 0 for p, n in zip(pos, nums[i:i + 2])]

            while pos != tuple(nums[i:i + 2]):
                area.add(pos)
                pos = tuple(p + o for p, o in zip(pos, offset))

        area.add(pos)

    maxY = max([e[1] for e in list(area)]) + 1

    pastPos = [(500, 0)]
    sand = set()

    while len(pastPos) != 0:
        pos = pastPos.pop()

        if pos[1] == maxY:
            return len(sand)
        else:
            for move in [0, -1, 1]:
                newPos = tuple(p + o for p, o in zip(pos, [move, 1]))

                if newPos not in area and newPos not in sand:
                    pastPos.append(pos)
                    pastPos.append(newPos)
                    break

        if len(pastPos) == 0 or pastPos[-1] != newPos:
            sand.add(pos)

    return -1


def part2(data):
    """ 2022 Day 14 Part 2

    >>> part2(['498,4 -> 498,6 -> 496,6', '503,4 -> 502,4 -> 502,9 -> 494,9'])
    93
    """

    area = set()
    for line in data:
        nums = [int(x) for x in re.findall("\d+", line)]

        pos = tuple(nums[:2])

        for i in range(2, len(nums), 2):
            offset = [(abs(n - p) // (n - p)) if n - p != 0 else 0 for p, n in zip(pos, nums[i:i + 2])]

            while pos != tuple(nums[i:i + 2]):
                area.add(pos)
                pos = tuple(p + o for p, o in zip(pos, offset))

        area.add(pos)

    maxY = max([e[1] for e in list(area)]) + 1

    pastPos = [(500, 0)]
    sand = set()

    while len(pastPos) != 0:
        pos = pastPos.pop()

        if pos[1] != maxY:
            for move in [0, -1, 1]:
                newPos = tuple(p + o for p, o in zip(pos, [move, 1]))

                if newPos not in area and newPos not in sand:
                    pastPos.append(pos)
                    pastPos.append(newPos)
                    break

        if len(pastPos) == 0 or pastPos[-1] != newPos:
            sand.add(pos)

    return len(sand)


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
        print(f"\nPart 1:\nUnits of sand that come to rest: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nUnits of sand that come to rest: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)