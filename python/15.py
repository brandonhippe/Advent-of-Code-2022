import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from shapely import LineString, Polygon, geometry
from shapely.ops import unary_union


def part1(data, testLine = 2000000):
    """ 2022 Day 15 Part 1

    >>> part1(['Sensor at x=2, y=18: closest beacon is at x=-2, y=15', 'Sensor at x=9, y=16: closest beacon is at x=10, y=16', 'Sensor at x=13, y=2: closest beacon is at x=15, y=3', 'Sensor at x=12, y=14: closest beacon is at x=10, y=16', 'Sensor at x=10, y=20: closest beacon is at x=10, y=16', 'Sensor at x=14, y=17: closest beacon is at x=10, y=16', 'Sensor at x=8, y=7: closest beacon is at x=2, y=10', 'Sensor at x=2, y=0: closest beacon is at x=2, y=10', 'Sensor at x=0, y=11: closest beacon is at x=2, y=10', 'Sensor at x=20, y=14: closest beacon is at x=25, y=17', 'Sensor at x=17, y=20: closest beacon is at x=21, y=22', 'Sensor at x=16, y=7: closest beacon is at x=15, y=3', 'Sensor at x=14, y=3: closest beacon is at x=15, y=3', 'Sensor at x=20, y=1: closest beacon is at x=15, y=3'], 10)
    26
    """
    
    sensorData = []
    beacons = set()
    minX = float('inf')
    maxX = float('-inf')

    for line in data:
        nums = [int(x) for x in re.findall("-?\d+", line)]
        sensor = tuple(nums[:2])
        beacon = tuple(nums[2:])
        d = manhatDist(sensor, beacon)

        minX = min(minX, sensor[0] - d)
        maxX = max(maxX, sensor[0] + d)

        sensorData.append(Polygon([[sensor[0] + d, sensor[1]], [sensor[0], sensor[1] + d], [sensor[0] - d, sensor[1]], [sensor[0], sensor[1] - d]]))
        beacons.add(beacon)

    return int(LineString([[minX, testLine], [maxX, testLine]]).intersection(unary_union(sensorData)).length) + 1 - len([b for b in beacons if b[1] == testLine])


def part2(data, testLine = 2000000):
    """ 2022 Day 15 Part 2

    # >>> part2(['Sensor at x=2, y=18: closest beacon is at x=-2, y=15', 'Sensor at x=9, y=16: closest beacon is at x=10, y=16', 'Sensor at x=13, y=2: closest beacon is at x=15, y=3', 'Sensor at x=12, y=14: closest beacon is at x=10, y=16', 'Sensor at x=10, y=20: closest beacon is at x=10, y=16', 'Sensor at x=14, y=17: closest beacon is at x=10, y=16', 'Sensor at x=8, y=7: closest beacon is at x=2, y=10', 'Sensor at x=2, y=0: closest beacon is at x=2, y=10', 'Sensor at x=0, y=11: closest beacon is at x=2, y=10', 'Sensor at x=20, y=14: closest beacon is at x=25, y=17', 'Sensor at x=17, y=20: closest beacon is at x=21, y=22', 'Sensor at x=16, y=7: closest beacon is at x=15, y=3', 'Sensor at x=14, y=3: closest beacon is at x=15, y=3', 'Sensor at x=20, y=1: closest beacon is at x=15, y=3'], 10)
    # 56000011
    """

    sensorData = []
    for line in data:
        nums = [int(x) for x in re.findall("-?\d+", line)]
        sensor = tuple(nums[:2])
        beacon = tuple(nums[2:])
        d = manhatDist(sensor, beacon)

        sensorData.append(Polygon([[sensor[0] + d, sensor[1]], [sensor[0], sensor[1] + d], [sensor[0] - d, sensor[1]], [sensor[0], sensor[1] - d]]))

    distressBox = geometry.box(0, 0, testLine * 2, testLine * 2)
    distress = [int(c) + 1 for c in distressBox.difference(unary_union(sensorData)).bounds]

    return distress[0] * 4000000 + distress[1]


def manhatDist(p1, p2):
    return sum([abs(c1 - c2) for c1, c2 in zip(p1, p2)])


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
        print(f"\nPart 1:\nPositions that cannot contain a beacon: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nTuning Frequency: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)