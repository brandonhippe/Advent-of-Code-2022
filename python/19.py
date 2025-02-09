import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from math import ceil
from collections import defaultdict


def part1(data):
    """ 2022 Day 19 Part 1

    >>> part1(['Blueprint 1: Each ore robot costs 4 ore.  Each clay robot costs 2 ore.  Each obsidian robot costs 3 ore and 14 clay.  Each geode robot costs 2 ore and 7 obsidian.', 'Blueprint 2:  Each ore robot costs 2 ore.  Each clay robot costs 3 ore.  Each obsidian robot costs 3 ore and 8 clay.  Each geode robot costs 3 ore and 12 obsidian.'])
    33
    """

    qualitySum = 0
    for line in data:
        blueprint, oreCost, clayOre, obsOre, obsClay, geoOre, geoObs = [int(x) for x in re.findall('\d+', line)]
        qualitySum += blueprint * bfs(oreCost, clayOre, obsOre, obsClay, geoOre, geoObs, 24)

    return qualitySum


def part2(data):
    """ 2022 Day 19 Part 2

    >>> part2(['Blueprint 1: Each ore robot costs 4 ore.  Each clay robot costs 2 ore.  Each obsidian robot costs 3 ore and 14 clay.  Each geode robot costs 2 ore and 7 obsidian.', 'Blueprint 2:  Each ore robot costs 2 ore.  Each clay robot costs 3 ore.  Each obsidian robot costs 3 ore and 8 clay.  Each geode robot costs 3 ore and 12 obsidian.'])
    3472
    """

    product = 1
    for line in data[:3]:
        _, oreCost, clayOre, obsOre, obsClay, geoOre, geoObs = [int(x) for x in re.findall('\d+', line)]
        product *= mostGeodes(oreCost, clayOre, obsOre, obsClay, geoOre, geoObs, 32)

    return product


def bfs(oreCost, clayOre, obsOre, obsClay, geoOre, geoObs, maxT):
    states = defaultdict(set)
    states[0].add((0, 0, 0, 0, 1, 0, 0))

    maxGeodes = 0
    while sum(len(states[k]) for k in states.keys()) != 0:
        minK = min(list(states.keys()))

        for state in list(states[minK]):
            geodes, ore, clay, obs, oreR, clayR, obsR = state

            maxGeodes = max(maxGeodes, geodes)

            ## Save up to make Geode Robot
            if obsR != 0:
                timeTaken = ceil(max((geoOre - ore) / oreR, (geoObs - obs) / obsR)) + 1

                if timeTaken <= maxT - minK and timeTaken > 0:
                    states[minK + timeTaken].add((geodes + maxT - (minK + timeTaken), ore + (oreR * timeTaken) - geoOre, clay + (clayR * timeTaken), obs + (obsR * timeTaken) - geoObs, oreR, clayR, obsR))

            ## Save up to make Obsidian Robot
            if clayR != 0 and obsR < geoObs and timeTaken > 0:
                timeTaken = ceil(max((obsOre - ore) / oreR, (obsClay - clay) / clayR)) + 1

                if timeTaken <= maxT - minK and timeTaken > 0:
                    states[minK + timeTaken].add((geodes, ore + (oreR * timeTaken) - obsOre, clay + (clayR * timeTaken) - obsClay, obs + (obsR * timeTaken), oreR, clayR, obsR + 1))


            ## Save up to make Clay Robot
            if clayR < obsClay:
                timeTaken = ceil((clayOre - ore) / oreR) + 1

                if timeTaken <= maxT - minK and timeTaken > 0:
                    states[minK + timeTaken].add((geodes, ore + (oreR * timeTaken) - clayOre, clay + (clayR * timeTaken), obs + (obsR * timeTaken), oreR, clayR + 1, obsR))


            ## Save up to make Ore Robot
            if oreR < max(clayOre, obsOre, geoOre):
                timeTaken = ceil((oreCost - ore) / oreR) + 1

                if timeTaken <= maxT - minK and timeTaken > 0:
                    states[minK + timeTaken].add((geodes, ore + (oreR * timeTaken) - oreCost, clay + (clayR * timeTaken), obs + (obsR * timeTaken), oreR + 1, clayR, obsR))

            states[minK].remove(state)

        if len(states[minK]) == 0:
            del(states[minK])

    return maxGeodes


def mostGeodes(oreCost, clayOre, obsOre, obsClay, geoOre, geoObs, maxT):
    states = {(0, 0, 0, 0, 1, 0, 0)}

    maxGeodes = 0
    for t in range(maxT):
        newStates = set()

        for state in states:
            ore, clay, obs, geo, oreR, clayR, obsR = state
            newOre = ore + oreR
            newClay = clay + clayR
            newObs = obs + obsR

            maxGeodes = max(maxGeodes, geo)

            if ore >= geoOre and obs >= geoObs:
                newStates.add((newOre - geoOre, newClay, newObs - geoObs, geo + maxT - t - 1, oreR, clayR, obsR))
            else:
                newStates.add((newOre, newClay, newObs, geo, oreR, clayR, obsR))

                if ore >= oreCost and oreR <= max(clayOre, obsOre, geoOre):
                    newStates.add((newOre - oreCost, newClay, newObs, geo, oreR + 1, clayR, obsR))

                if ore >= clayOre and clayR <= obsClay:
                    newStates.add((newOre - clayOre, newClay, newObs, geo, oreR, clayR + 1, obsR))

                if ore >= obsOre and clay >= obsClay and obsR <= geoObs:
                    newStates.add((newOre - obsOre, newClay - obsClay, newObs, geo, oreR, clayR, obsR + 1))


        states = set()
        if t == maxT - 1:
            break

        for state in newStates:
            if state[3] + maxT - 1 - t >= maxGeodes:
                states.add(state)

    return maxGeodes


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
        print(f"\nPart 1:\nQuality Sum: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nProduct of first 3 blueprints: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)