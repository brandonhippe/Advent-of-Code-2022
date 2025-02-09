import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict


def part1(data):
    """ 2022 Day 17 Part 1

    >>> part1(['>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>'])
    3068
    """

    jets = [-1 if c == '<' else 1 for c in data[0]]

    rockCount = 0
    rockIx = 0
    jetIx = 0
    rockPos = set((n, 0) for n in range(7))
    maxY = 0

    while rockCount < 2022:        
        newRock = [tuple(p + o for p, o in zip(pos, [0, maxY])) for pos in ROCKS[rockIx]]
        pRock = newRock[:]

        while not any(pos in rockPos for pos in newRock):            
            if (min(p[0] for p in newRock) > 0 and jets[jetIx] == -1) or (max(p[0] for p in newRock) < 6 and jets[jetIx] == 1):
                pRock = newRock[:]
                newRock = [tuple(p + o for p, o in zip(pos, [jets[jetIx], 0])) for pos in newRock]

                if any(pos in rockPos for pos in newRock):
                    newRock = pRock

            jetIx += 1
            jetIx %= len(jets)

            pRock = newRock[:]
            newRock = [tuple(p + o for p, o in zip(pos, [0, -1])) for pos in newRock]

        rockPos = rockPos.union(set(pRock))
        maxY = max(maxY, max(p[1] for p in pRock))

        rockIx += 1
        rockIx %= len(ROCKS)
        rockCount += 1

    return maxY


def part2(data):
    """ 2022 Day 17 Part 2

    >>> part2(['>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>'])
    1514285714288
    """

    jets = [-1 if c == '<' else 1 for c in data[0]]

    rockCount = 0
    rockIx = 0
    jetIx = 0
    rockPos = set((n, 0) for n in range(7))
    maxY = 0

    cycles = defaultdict(lambda: [])

    while True:        
        newRock = [tuple(p + o for p, o in zip(pos, [0, maxY])) for pos in ROCKS[rockIx]]
        pRock = newRock[:]

        cycles[(rockIx, jetIx)].append([rockCount, maxY])

        if len(cycles[(rockIx, jetIx)]) >= 3 and cycles[(rockIx, jetIx)][-1][1] - cycles[(rockIx, jetIx)][-2][1] == cycles[(rockIx, jetIx)][-2][1] - cycles[(rockIx, jetIx)][-3][1]:
            cycleLen = cycles[(rockIx, jetIx)][-1][0] - cycles[(rockIx, jetIx)][-2][0]
            heightCycle = cycles[(rockIx, jetIx)][-1][1] - cycles[(rockIx, jetIx)][-2][1]

            if (1000000000000 - rockCount) % cycleLen == 0:
                maxY += (1000000000000 - rockCount) // cycleLen * heightCycle
                break

        while not any(pos in rockPos for pos in newRock):            
            if (min(p[0] for p in newRock) > 0 and jets[jetIx] == -1) or (max(p[0] for p in newRock) < 6 and jets[jetIx] == 1):
                pRock = newRock[:]
                newRock = [tuple(p + o for p, o in zip(pos, [jets[jetIx], 0])) for pos in newRock]

                if any(pos in rockPos for pos in newRock):
                    newRock = pRock

            jetIx += 1
            jetIx %= len(jets)

            pRock = newRock[:]
            newRock = [tuple(p + o for p, o in zip(pos, [0, -1])) for pos in newRock]

        rockPos = rockPos.union(set(pRock))
        maxY = max(maxY, max(p[1] for p in pRock))

        rockIx += 1
        rockIx %= len(ROCKS)
        rockCount += 1

    return maxY


ROCKS = [[(2, 4), (3, 4), (4, 4), (5, 4)], [(2, 5), (3, 4), (3, 5), (3, 6), (4, 5)], [(2, 4), (3, 4), (4, 4), (4, 5), (4, 6)], [(2, 4), (2, 5), (2, 6), (2, 7)], [(2, 4), (2, 5), (3, 4), (3, 5)]]



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
        print(f"\nPart 1:\nTower Height after 2022 Rocks: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nTower Height after 1000000000000 Rocks: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)