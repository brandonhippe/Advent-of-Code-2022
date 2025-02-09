import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data):
    """ 2022 Day 11 Part 1

    >>> part1(['Monkey 0:', '  Starting items: 79, 98', '  Operation: new = old * 19', '  Test: divisible by 23', '    If true: throw to monkey 2', '    If false: throw to monkey 3', '', 'Monkey 1:', '  Starting items: 54, 65, 75, 74', '  Operation: new = old + 6', '  Test: divisible by 19', '    If true: throw to monkey 2', '    If false: throw to monkey 0', '', 'Monkey 2:', '  Starting items: 79, 60, 97', '  Operation: new = old * old', '  Test: divisible by 13', '    If true: throw to monkey 1', '    If false: throw to monkey 3', '', 'Monkey 3:', '  Starting items: 74', '  Operation: new = old + 3', '  Test: divisible by 17', '    If true: throw to monkey 0', '    If false: throw to monkey 1'])
    10605
    """

    monkeyBusiness = {}
    monkeyOps = {}
    monkeyTests = {}

    items = []

    for line in data:
        if "Monkey" in line:
            monkeyNum = [int(x) for x in re.findall("-?\d+", line)][0]
            monkeyBusiness[monkeyNum] = 0
        elif "Starting items" in line:
            items.append([])
            for i in [int(x) for x in re.findall("-?\d+", line)]:
                items[-1].append(i)
        elif "Operation" in line:
            if "old * old" in line:
                monkeyOps[monkeyNum] = [square, 0]
            elif "*" in line:
                monkeyOps[monkeyNum] = [mult, [int(x) for x in re.findall("-?\d+", line)][0]]
            else:
                monkeyOps[monkeyNum] = [add, [int(x) for x in re.findall("-?\d+", line)][0]]
        elif len(line) != 0:
            if monkeyNum not in monkeyTests:
                monkeyTests[monkeyNum] = []

            monkeyTests[monkeyNum].append([int(x) for x in re.findall("-?\d+", line)][0])

    for _ in range(20):
        for monkeyNum, monkeyItems in enumerate(items):
            items[monkeyNum] = []
            monkeyBusiness[monkeyNum] += len(monkeyItems)

            for item in monkeyItems:
                item = monkeyOps[monkeyNum][0](item, monkeyOps[monkeyNum][1])
                item //= 3

                if item % monkeyTests[monkeyNum][0] == 0:
                    items[monkeyTests[monkeyNum][1]].append(item)
                else:
                    items[monkeyTests[monkeyNum][2]].append(item)

    result = sorted(monkeyBusiness.values(), reverse=True)
    return result[0] * result[1]


def part2(data):
    """ 2022 Day 11 Part 2

    >>> part2(['Monkey 0:', '  Starting items: 79, 98', '  Operation: new = old * 19', '  Test: divisible by 23', '    If true: throw to monkey 2', '    If false: throw to monkey 3', '', 'Monkey 1:', '  Starting items: 54, 65, 75, 74', '  Operation: new = old + 6', '  Test: divisible by 19', '    If true: throw to monkey 2', '    If false: throw to monkey 0', '', 'Monkey 2:', '  Starting items: 79, 60, 97', '  Operation: new = old * old', '  Test: divisible by 13', '    If true: throw to monkey 1', '    If false: throw to monkey 3', '', 'Monkey 3:', '  Starting items: 74', '  Operation: new = old + 3', '  Test: divisible by 17', '    If true: throw to monkey 0', '    If false: throw to monkey 1'])
    2713310158
    """

    monkeyBusiness = {}
    monkeyOps = {}
    monkeyTests = {}

    items = []

    for line in data:
        if "Monkey" in line:
            monkeyNum = [int(x) for x in re.findall("-?\d+", line)][0]
            monkeyBusiness[monkeyNum] = 0
        elif "Starting items" in line:
            items.append([])
            for i in [int(x) for x in re.findall("-?\d+", line)]:
                items[-1].append(i)
        elif "Operation" in line:
            if "old * old" in line:
                monkeyOps[monkeyNum] = [square, 0]
            elif "*" in line:
                monkeyOps[monkeyNum] = [mult, [int(x) for x in re.findall("-?\d+", line)][0]]
            else:
                monkeyOps[monkeyNum] = [add, [int(x) for x in re.findall("-?\d+", line)][0]]
        elif len(line) != 0:
            if monkeyNum not in monkeyTests:
                monkeyTests[monkeyNum] = []

            monkeyTests[monkeyNum].append([int(x) for x in re.findall("-?\d+", line)][0])

    monkeyBusiness = {m: 0 for m in monkeyBusiness.keys()}

    monkeyMod = 1
    for i in range(len(monkeyBusiness)):
        monkeyMod *= monkeyTests[i][0]

    for _ in range(10000):
        for monkeyNum, monkeyItems in enumerate(items):
            items[monkeyNum] = []
            monkeyBusiness[monkeyNum] += len(monkeyItems)

            for item in monkeyItems:
                item = monkeyOps[monkeyNum][0](item, monkeyOps[monkeyNum][1])
                item %= monkeyMod

                if item % monkeyTests[monkeyNum][0] == 0:
                    items[monkeyTests[monkeyNum][1]].append(item)
                else:
                    items[monkeyTests[monkeyNum][2]].append(item)

    result = sorted(monkeyBusiness.values(), reverse=True)
    return result[0] * result[1]


def add(a, n):
    return a + n


def mult(a, n):
    return a * n


def square(a, _):
    return a * a


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
        print(f"\nPart 1:\nMonkey Business after 20 rounds: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nMonkey Business after 10000 rounds: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)