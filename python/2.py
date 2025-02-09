import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2022 Day 2 Part 1

    >>> part1(['A Y', 'B X', 'C Z'])
    15
    """

    score = 0
    for line in data:
        p1, p2 = line.split(' ')
        played = [ord(p1) - ord('A'), ord(p2) - ord('X')]
        
        score += played[1] + 1
        if played[0] == played[1]:
            score += 3
        elif (played[1] - 1) % 3 == played[0]:
            score += 6    

    return score


def part2(data):
    """ 2022 Day 2 Part 2

    >>> part2(['A Y', 'B X', 'C Z'])
    12
    """

    score = 0
    for line in data:
        p1, p2 = line.split(' ')
        opp = ord(p1) - ord('A')
        if p2 == 'X':
            offset = 2
        elif p2 == 'Y':
            offset = 0
            score += 3
        else:
            offset = 1
            score += 6

        score += (opp + offset) % 3 + 1

    return score


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
        print(f"\nPart 1:\nScore: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nScore: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)