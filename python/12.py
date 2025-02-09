import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict


def part1(data):
    """ 2022 Day 12 Part 1

    >>> part1(['Sabqponm', 'abcryxxl', 'accszExk', 'acctuvwj', 'abdefghi'])
    31
    """

    height = {}
    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l == "S":
                height[(x, y)] = 0
                start = (x, y)
            elif l == "E":
                height[(x, y)] = 25
                end = (x, y)
            else:
                height[(x, y)] = ord(l) - ord('a')

    return bfs(startState = end, end = start, height = height, nextStateFunc = nextState, abortFunc = lambda state, end, **kwargs: state == end, trackFunc = lambda state, t, tracked, height, **kwargs: t if tracked is None and height[state] == 0 else tracked)[0]


def part2(data):
    """ 2022 Day 12 Part 2

    >>> part2(['Sabqponm', 'abcryxxl', 'accszExk', 'acctuvwj', 'abdefghi'])
    29
    """

    height = {}
    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l == "S":
                height[(x, y)] = 0
                start = (x, y)
            elif l == "E":
                height[(x, y)] = 25
                end = (x, y)
            else:
                height[(x, y)] = ord(l) - ord('a')

    return bfs(startState = end, end = start, height = height, nextStateFunc = nextState, abortFunc = lambda state, end, **kwargs: state == end, trackFunc = lambda state, t, tracked, height, **kwargs: t if tracked is None and height[state] == 0 else tracked)[1]


def bfs(startState, nextStateFunc, abortFunc, trackFunc, **kwargs):
    states = defaultdict(set)
    states[0].add(startState)

    visited = set()

    tracked = None
    while len(states) != 0:
        minT = min(states.keys())

        for state in states[minT]:
            if state in visited:
                continue

            visited.add(state)

            tracked = trackFunc(tracked = tracked, state = state, t = minT, visited = visited, **kwargs)

            if abortFunc(state = state, tracked = tracked, visited = visited, **kwargs):
                return minT if tracked is None else [minT, tracked]

            for newState, t in nextStateFunc(state = state, t = minT, tracked = tracked, visited = visited, **kwargs):
                states[t].add(newState)
                
        del(states[minT])

    return tracked


def nextState(state, height, t, **kwargs):
    newStates = []

    for nOff in [[1, 0], [-1, 0], [0, 1], [0, -1]]:
        nPos = tuple(p + o for p, o in zip(state, nOff))

        if nPos in height and height[state] - height[nPos] <= 1:
            newStates.append([nPos, t + 1])

    return newStates


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
        print(f"\nPart 1:\nFewest steps from start to summit: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nShortest scenic path: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)