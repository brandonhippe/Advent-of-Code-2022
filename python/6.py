import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2022 Day 6 Part 1

    >>> part1(['mjqjpqmgbljsphdztnvjfqwrcgsmlb'])
    7
    >>> part1(['bvwbjplbgvbhsrlpgdmjqwftvncz'])
    5
    >>> part1(['nppdvjthqldpwncqszvftbrmjlhg'])
    6
    >>> part1(['nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg'])
    10
    >>> part1(['zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw'])
    11
    """

    for i in range(4, len(data[0]) + 1):
        if len(set(data[0][i - 4:i])) == 4:
            return i
        
    return -1


def part2(data):
    """ 2022 Day 6 Part 2

    >>> part2(['mjqjpqmgbljsphdztnvjfqwrcgsmlb'])
    19
    >>> part2(['bvwbjplbgvbhsrlpgdmjqwftvncz'])
    23
    >>> part2(['nppdvjthqldpwncqszvftbrmjlhg'])
    23
    >>> part2(['nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg'])
    29
    >>> part2(['zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw'])
    26
    """

    for i in range(14, len(data[0]) + 1):
        if len(set(data[0][i - 14:i])) == 14:
            return i

    return -1


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
        print(f"\nPart 1:\nFirst occurrance of non-repeating 4 characters: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nFirst occurrance of non-repeating 14 characters: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)