import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2022 Day 10 Part 1

    >>> part1(['addx 15', 'addx -11', 'addx 6', 'addx -3', 'addx 5', 'addx -1', 'addx -8', 'addx 13', 'addx 4', 'noop', 'addx -1', 'addx 5', 'addx -1', 'addx 5', 'addx -1', 'addx 5', 'addx -1', 'addx 5', 'addx -1', 'addx -35', 'addx 1', 'addx 24', 'addx -19', 'addx 1', 'addx 16', 'addx -11', 'noop', 'noop', 'addx 21', 'addx -15', 'noop', 'noop', 'addx -3', 'addx 9', 'addx 1', 'addx -3', 'addx 8', 'addx 1', 'addx 5', 'noop', 'noop', 'noop', 'noop', 'noop', 'addx -36', 'noop', 'addx 1', 'addx 7', 'noop', 'noop', 'noop', 'addx 2', 'addx 6', 'noop', 'noop', 'noop', 'noop', 'noop', 'addx 1', 'noop', 'noop', 'addx 7', 'addx 1', 'noop', 'addx -13', 'addx 13', 'addx 7', 'noop', 'addx 1', 'addx -33', 'noop', 'noop', 'noop', 'addx 2', 'noop', 'noop', 'noop', 'addx 8', 'noop', 'addx -1', 'addx 2', 'addx 1', 'noop', 'addx 17', 'addx -9', 'addx 1', 'addx 1', 'addx -3', 'addx 11', 'noop', 'noop', 'addx 1', 'noop', 'addx 1', 'noop', 'noop', 'addx -13', 'addx -19', 'addx 1', 'addx 3', 'addx 26', 'addx -30', 'addx 12', 'addx -1', 'addx 3', 'addx 1', 'noop', 'noop', 'noop', 'addx -9', 'addx 18', 'addx 1', 'addx 2', 'noop', 'noop', 'addx 9', 'noop', 'noop', 'noop', 'addx -1', 'addx 2', 'addx -37', 'addx 1', 'addx 3', 'noop', 'addx 15', 'addx -21', 'addx 22', 'addx -6', 'addx 1', 'noop', 'addx 2', 'addx 1', 'noop', 'addx -10', 'noop', 'noop', 'addx 20', 'addx 1', 'addx 2', 'addx 2', 'addx -6', 'addx -11', 'noop', 'noop', 'noop'])
    13140
    """

    x = 1
    cycles = 0

    signalStrength = 0
    for line in data:
        if line == "noop":
            cycles += 1
            if cycles in [20, 60, 100, 140, 180, 220]:
                signalStrength += x * cycles
        else:
            cycles += 1
            if cycles in [20, 60, 100, 140, 180, 220]:
                signalStrength += x * cycles
            
            cycles += 1
            if cycles in [20, 60, 100, 140, 180, 220]:
                signalStrength += x * cycles

            x += int(line.split(' ')[1])

    return signalStrength


def part2(data):
    """ 2022 Day 10 Part 2

    >>> part2(['addx 15', 'addx -11', 'addx 6', 'addx -3', 'addx 5', 'addx -1', 'addx -8', 'addx 13', 'addx 4', 'noop', 'addx -1', 'addx 5', 'addx -1', 'addx 5', 'addx -1', 'addx 5', 'addx -1', 'addx 5', 'addx -1', 'addx -35', 'addx 1', 'addx 24', 'addx -19', 'addx 1', 'addx 16', 'addx -11', 'noop', 'noop', 'addx 21', 'addx -15', 'noop', 'noop', 'addx -3', 'addx 9', 'addx 1', 'addx -3', 'addx 8', 'addx 1', 'addx 5', 'noop', 'noop', 'noop', 'noop', 'noop', 'addx -36', 'noop', 'addx 1', 'addx 7', 'noop', 'noop', 'noop', 'addx 2', 'addx 6', 'noop', 'noop', 'noop', 'noop', 'noop', 'addx 1', 'noop', 'noop', 'addx 7', 'addx 1', 'noop', 'addx -13', 'addx 13', 'addx 7', 'noop', 'addx 1', 'addx -33', 'noop', 'noop', 'noop', 'addx 2', 'noop', 'noop', 'noop', 'addx 8', 'noop', 'addx -1', 'addx 2', 'addx 1', 'noop', 'addx 17', 'addx -9', 'addx 1', 'addx 1', 'addx -3', 'addx 11', 'noop', 'noop', 'addx 1', 'noop', 'addx 1', 'noop', 'noop', 'addx -13', 'addx -19', 'addx 1', 'addx 3', 'addx 26', 'addx -30', 'addx 12', 'addx -1', 'addx 3', 'addx 1', 'noop', 'noop', 'noop', 'addx -9', 'addx 18', 'addx 1', 'addx 2', 'noop', 'noop', 'addx 9', 'noop', 'noop', 'noop', 'addx -1', 'addx 2', 'addx -37', 'addx 1', 'addx 3', 'noop', 'addx 15', 'addx -21', 'addx 22', 'addx -6', 'addx 1', 'noop', 'addx 2', 'addx 1', 'noop', 'addx -10', 'noop', 'noop', 'addx 20', 'addx 1', 'addx 2', 'addx 2', 'addx -6', 'addx -11', 'noop', 'noop', 'noop'])
    '\\n██  ██  ██  ██  ██  ██  ██  ██  ██  ██  \n███   ███   ███   ███   ███   ███   ███ \\n████    ████    ████    ████    ████    \\n█████     █████     █████     █████     \\n██████      ██████      ██████      ████\\n███████       ███████       ███████     '
    """

    x = 1
    cycles = 0
    string = ''
    for line in data:
        if cycles % 40 == 0:
            string += '\n'

        if abs(x - (cycles % 40)) <= 1:
            string += '█'
        else:
            string += ' '

        cycles += 1

        if line != "noop":
            if cycles % 40 == 0:
                string += '\n'

            if abs(x - (cycles % 40)) <= 1:
                string += '█'
            else:
                string += ' '

            x += int(line.split(' ')[1])

            cycles += 1

    return string


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
        print(f"\nPart 1:\nSum of signal strengths: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nText on CRT: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)