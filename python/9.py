import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2022 Day 9 Part 1

    >>> part1(['R 4', 'U 4', 'L 3', 'D 1', 'R 4', 'D 1', 'L 5', 'R 2'])
    13
    """

    head = [0, 0]
    tail = [0, 0]
    tailVisits = set()

    for line in data:
        d, amt = line.split(' ')
        d = dirs[d]
        amt = int(amt)

        while amt > 0:
            head = [p + o for p, o in zip(head, d)]

            tailVisits.add(tuple(tail))
            tail = moveTail(head, tail)

            amt -= 1

    tailVisits.add(tuple(tail))
    return len(tailVisits)


def part2(data):
    """ 2022 Day 9 Part 2

    >>> part2(['R 4', 'U 4', 'L 3', 'D 1', 'R 4', 'D 1', 'L 5', 'R 2'])
    1
    >>> part2(['R 5', 'U 8', 'L 8', 'D 3', 'R 17', 'D 10', 'L 25', 'U 20'])
    36
    """

    tails = [[0, 0] for _ in range(10)]
    tailVisits = set()

    for line in data:
        d, amt = line.split(' ')
        d = dirs[d]
        amt = int(amt)

        while amt > 0:
            tails[0] = [t + o for t, o in zip(tails[0], d)]

            tailVisits.add(tuple(tails[-1]))
            for i in range(1, 10):
                tails[i] = moveTail(tails[i - 1], tails[i])

            amt -= 1

    tailVisits.add(tuple(tails[-1]))
    return len(tailVisits)


dirs = {"L": (-1, 0), "R": (1, 0), "U": (0, 1), "D": (0, -1)}


def manhatDist(p1, p2):
    return sum(abs(c1 - c2) for c1, c2 in zip(p1, p2))


def moveTail(head, tail):
    if (any(h == t for h, t in zip(head, tail)) and max(abs(h - t) for h, t in zip(head, tail)) == 2) or manhatDist(head, tail) >= 3:
        diff = [((h - t) // abs(h - t)) if h - t != 0 else 0 for h, t in zip(head, tail)]
        return [c1 + c2 for c1, c2 in zip(tail, diff)]
    else:
        return tail


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
        print(f"\nPart 1:\nNumber of positions visited by tail of rope: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNumber of positions visited by tail of rope: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)