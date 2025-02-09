import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
import graphviz
import numpy as np
from itertools import product
from collections import defaultdict


def part1(data):
    """ 2022 Day 16 Part 1

    >>> part1(['Valve AA has flow rate=0; tunnels lead to valves DD, II, BB', 'Valve BB has flow rate=13; tunnels lead to valves CC, AA', 'Valve CC has flow rate=2; tunnels lead to valves DD, BB', 'Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE', 'Valve EE has flow rate=3; tunnels lead to valves FF, DD', 'Valve FF has flow rate=0; tunnels lead to valves EE, GG', 'Valve GG has flow rate=0; tunnels lead to valves FF, HH', 'Valve HH has flow rate=22; tunnel leads to valve GG', 'Valve II has flow rate=0; tunnels lead to valves AA, JJ', 'Valve JJ has flow rate=21; tunnel leads to valve II'])
    1651
    """

    flowRates = {}
    connections = {}

    for line in data:
        splitLine = line.split(" ")
        flowRates[splitLine[1]] = [int(x) for x in re.findall("\d+", line)][0]
        connections[splitLine[1]] = defaultdict(lambda: float('inf'))
        for v in splitLine[9:]:
            connections[splitLine[1]][v.strip(',')] = 1

    connections = floydWarshall(connections)

    unimportant = [v for v in flowRates.keys() if flowRates[v] == 0 and v != 'AA']

    for v in unimportant:
        for k in connections.keys():
            del(connections[k][v])

        del(flowRates[v])
        del(connections[v])

    valveBits = {'AA': 0}
    bit = 1
    for k in connections.keys():
        if k != 'AA':
            valveBits[k] = bit
            bit += 1

        for k1 in list(connections[k].keys()):
            if k1 == k:
                del(connections[k][k1])
            else:
                connections[k][k1] += 1

    memo = {}
    return dfs('AA', flowRates, connections, valveBits, 0, 30, memo)


def part2(data):
    """ 2022 Day 16 Part 2

    >>> part2(['Valve AA has flow rate=0; tunnels lead to valves DD, II, BB', 'Valve BB has flow rate=13; tunnels lead to valves CC, AA', 'Valve CC has flow rate=2; tunnels lead to valves DD, BB', 'Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE', 'Valve EE has flow rate=3; tunnels lead to valves FF, DD', 'Valve FF has flow rate=0; tunnels lead to valves EE, GG', 'Valve GG has flow rate=0; tunnels lead to valves FF, HH', 'Valve HH has flow rate=22; tunnel leads to valve GG', 'Valve II has flow rate=0; tunnels lead to valves AA, JJ', 'Valve JJ has flow rate=21; tunnel leads to valve II'])
    1707
    """

    flowRates = {}
    connections = {}

    for line in data:
        splitLine = line.split(" ")
        flowRates[splitLine[1]] = [int(x) for x in re.findall("\d+", line)][0]
        connections[splitLine[1]] = defaultdict(lambda: float('inf'))
        for v in splitLine[9:]:
            connections[splitLine[1]][v.strip(',')] = 1

    connections = floydWarshall(connections)

    unimportant = [v for v in flowRates.keys() if flowRates[v] == 0 and v != 'AA']

    for v in unimportant:
        for k in connections.keys():
            del(connections[k][v])

        del(flowRates[v])
        del(connections[v])

    valveBits = {'AA': 0}
    bit = 1
    for k in connections.keys():
        if k != 'AA':
            valveBits[k] = bit
            bit += 1

        for k1 in list(connections[k].keys()):
            if k1 == k:
                del(connections[k][k1])
            else:
                connections[k][k1] += 1

    arrDim = len(connections) - 1
    degree = np.zeros((arrDim, arrDim))
    adj = np.zeros((arrDim, arrDim))
    indexes = {k: len(valveBits) - v - 1 for k, v in valveBits.items() if k != 'AA'}

    for k, i in indexes.items():
        degree[i][i] = len(connections[k])
        
        for n, weight in connections[k].items():
            if n == 'AA':
                continue

            j = indexes[n]
            adj[i][j] = weight

    laplacian = degree - adj
    v = np.linalg.svd(laplacian)[2]
    fiedler = v[-2]

    fiedlerDiff = 0.521875
    memo = {}
    fiedlerResult = 0
    for xors in product([False, True], repeat = sum(abs(n) < fiedlerDiff for n in fiedler)):
        fiedlerSplit = ''
        ix = 0
        for n in fiedler:
            if abs(n) < fiedlerDiff:
                fiedlerSplit += '1' if (n > 0) ^ xors[ix] else '0'
                ix += 1
            else:
                fiedlerSplit += '1' if n > 0 else '0'

        fiedlerResult = max(fiedlerResult, dfs('AA', flowRates, connections, valveBits, int(fiedlerSplit, 2) << 1, 26, memo) + dfs('AA', flowRates, connections, valveBits, (~int(fiedlerSplit, 2)) << 1, 26, memo))

    return fiedlerResult

    maxPressure = 0
    maxIx = 0
    for i in range(2 ** (len(flowRates) - 1)):
        # maxPressure = max(maxPressure, dfs('AA', flowRates, connections, valveBits, i << 1, 26, memo) + dfs('AA', flowRates, connections, valveBits, (~i) << 1, 26, memo))
        thisVal = dfs('AA', flowRates, connections, valveBits, i << 1, 26, memo) + dfs('AA', flowRates, connections, valveBits, (~i) << 1, 26, memo)

        if thisVal > maxPressure:
            maxPressure = thisVal
            maxIx = i

    return maxPressure


def dfs(currValve, flowRates, connections, valveBits, openValves, timeRem, memo):
    if timeRem <= 1:
        memo[(currValve, openValves, timeRem)] = 0
        return 0

    if (currValve, openValves, timeRem) in memo:
        return memo[(currValve, openValves, timeRem)]

    openValves |= 1 << valveBits[currValve]
    released = timeRem * flowRates[currValve]

    maxReleased = 0
    for v, t in zip(connections[currValve].keys(), connections[currValve].values()):
        if openValves & 1 << valveBits[v] == 1 << valveBits[v]:
            continue
        
        if (v, openValves, timeRem - t) not in memo:
            dfs(v, flowRates, connections, valveBits, openValves, timeRem - t, memo)
        
        maxReleased = max(maxReleased, memo[(v, openValves, timeRem - t)])

    openValves &= ~(1 << valveBits[currValve])

    memo[(currValve, openValves, timeRem)] = released + maxReleased
    return released + maxReleased


def floydWarshall(connections):
    for k in connections.keys():
        connections[k][k] = 0

    for k in connections.keys():
        for i in connections.keys():
            for j in connections.keys():
                connections[i][j] = min(connections[i][j], connections[i][k] + connections[k][j])

    return connections


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
        print(f"\nPart 1:\nPressure Released: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nPressure Released: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)