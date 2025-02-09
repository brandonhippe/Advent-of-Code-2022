import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data):
    """ 2022 Day 21 Part 1

    >>> part1(['root: pppw + sjmn', 'dbpl: 5', 'cczh: sllz + lgvd', 'zczc: 2', 'ptdq: humn - dvpt', 'dvpt: 3', 'lfqf: 4', 'humn: 5', 'ljgn: 2', 'sjmn: drzm * dbpl', 'sllz: 4', 'pppw: cczh / lfqf', 'lgvd: ljgn * ptdq', 'drzm: hmdt - zczc', 'hmdt: 32'])
    152
    """

    monkeys = {}
    while len(monkeys) != len(data) and 'root' not in monkeys:
        for line in data:
            monkey = line[:4]
            if monkey in monkeys:
                continue
            
            if len(re.findall('-?\d+', line)) != 0:
                monkeys[monkey] = [int(x) for x in re.findall('-?\d+', line)][0]
            else:
                _, m1, op, m2 = line[4:].split(' ')
                if m1 in monkeys and m2 in monkeys:
                    if op == '+':
                        monkeys[monkey] = monkeys[m1] + monkeys[m2]
                    elif op == '-':
                        monkeys[monkey] = monkeys[m1] - monkeys[m2]
                    elif op == '*':
                        monkeys[monkey] = monkeys[m1] * monkeys[m2]
                    elif op == '/':
                        monkeys[monkey] = monkeys[m1] // monkeys[m2]
    
    return monkeys['root']


def part2(data):
    """ 2022 Day 21 Part 2

    >>> part2(['root: pppw + sjmn', 'dbpl: 5', 'cczh: sllz + lgvd', 'zczc: 2', 'ptdq: humn - dvpt', 'dvpt: 3', 'lfqf: 4', 'humn: 5', 'ljgn: 2', 'sjmn: drzm * dbpl', 'sllz: 4', 'pppw: cczh / lfqf', 'lgvd: ljgn * ptdq', 'drzm: hmdt - zczc', 'hmdt: 32'])
    301
    """

    monkeys = {line.split(': ')[0]: line.split(': ')[1].split(' ') for line in data}
    monkeys['root'][1] = '='

    return traceBack(monkeys, 'root')


def yellResult(monkeys, currM):
    if currM == 'humn':
        return None

    if len(monkeys[currM]) == 1:
        return int(monkeys[currM][0])

    m1, op, m2 = monkeys[currM]
    m1Res = yellResult(monkeys, m1)
    m2Res = yellResult(monkeys, m2)

    if m1Res is not None and m2Res is not None:
        if op == '+':
            return m1Res + m2Res

        if op == '-':
            return m1Res - m2Res

        if op == '*':
            return m1Res * m2Res
            
        if op == '/':
            return m1Res // m2Res
    else:
        return None


def traceBack(monkeys, currM, desired = None):
    if currM == 'humn':
        monkeys['humn'] = desired
        return desired

    yell = [yellResult(monkeys, m) for m in monkeys[currM][::2]]

    op = monkeys[currM][1]
    
    ix = (yell.index(None) + 1) % 2

    if desired is None:
        desired = yell[ix]

    if op == '+':
        newDesired = desired - yell[ix]
    elif op == '-':
        if ix == 0:
            newDesired = yell[ix] - desired
        else:
            newDesired = desired + yell[ix]
    elif op == '*':
        newDesired = desired // yell[ix]
    elif op == '/':
        if ix == 0:
            newDesired = yell[ix] // desired
        else:
            newDesired = desired * yell[ix]
    elif op == '=':
        newDesired = desired

    return traceBack(monkeys, monkeys[currM][ix - 1], newDesired)


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
        print(f"\nPart 1:\nRoot monkey yells: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nYell to pass equality test: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)