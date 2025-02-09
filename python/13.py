import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2022 Day 13 Part 1

    >>> part1(['[1,1,3,1,1]', '[1,1,5,1,1]', '', '[[1],[2,3,4]]', '[[1],4]', '', '[9]', '[[8,7,6]]', '', '[[4,4],4,4]', '[[4,4],4,4,4]', '', '[7,7,7,7]', '[7,7,7]', '', '[]', '[3]', '', '[[[]]]', '[[]]', '', '[1,[2,[3,[4,[5,6,7]]]],8,9]', '[1,[2,[3,[4,[5,6,0]]]],8,9]'])
    13
    """

    index = 0
    correctSum = 0
    packets = []
    for i in range(0, len(data), 3):
        index += 1
        packet1 = Packet(data[i][1:-1])
        packet2 = Packet(data[i + 1][1:-1])

        packets.append(packet1)
        packets.append(packet2)

        if packet1 < packet2:
            correctSum += index

    return correctSum


def part2(data):
    """ 2022 Day 13 Part 2

    >>> part2(['[1,1,3,1,1]', '[1,1,5,1,1]', '', '[[1],[2,3,4]]', '[[1],4]', '', '[9]', '[[8,7,6]]', '', '[[4,4],4,4]', '[[4,4],4,4,4]', '', '[7,7,7,7]', '[7,7,7]', '', '[]', '[3]', '', '[[[]]]', '[[]]', '', '[1,[2,[3,[4,[5,6,7]]]],8,9]', '[1,[2,[3,[4,[5,6,0]]]],8,9]'])
    140
    """

    index = 0
    packets = []
    for i in range(0, len(data), 3):
        index += 1
        packet1 = Packet(data[i][1:-1])
        packet2 = Packet(data[i + 1][1:-1])

        packets.append(packet1)
        packets.append(packet2)

    packets.append(Packet("[[2]]", True))
    packets.append(Packet("[[6]]", True))

    packets.sort()
    product = 1
    for i, packet in enumerate(packets):
        if packet.div:
            product *= i + 1

    return product


class Packet:
    def __init__(self, packetString = "", div = False) -> None:
        self.packet = []
        self.div = div
        count = 0
        val = ""
        for i, c in enumerate(packetString):
            if c == '[':
                if count == 0:
                    start = i

                count += 1
            elif c == ']':
                count -= 1

                if count == 0:
                    self.packet.append(Packet(packetString[start + 1:i]))
            elif count == 0:
                if c == ',':
                    if len(val) != 0:
                        self.packet.append(int(val))
                        val = ""
                else:
                    val += c

        if len(val) != 0:
            self.packet.append(int(val))

    def __lt__(self, other):
        ix1, ix2 = 0, 0
        while ix1 < len(self.packet) and ix2 < len(other.packet):
            curr1, curr2 = self.packet[ix1], other.packet[ix2]
            if isinstance(curr1, int) and isinstance(curr2, int):
                if curr1 < curr2:
                    return True

                if curr1 > curr2:
                    return False
            elif isinstance(curr1, int) and isinstance(curr2, Packet):
                testPacket = Packet()
                testPacket.packet.append(curr1)
                if testPacket < curr2:
                    return True

                if testPacket > curr2:
                    return False
            elif isinstance(curr1, Packet) and isinstance(curr2, int):
                testPacket = Packet()
                testPacket.packet.append(curr2)
                if curr1 < testPacket:
                    return True

                if curr1 > testPacket:
                    return False
            else:
                if curr1 < curr2:
                    return True

                if curr1 > curr2:
                    return False

            ix1 += 1
            ix2 += 1

        return ix1 == len(self.packet) and ix2 != len(other.packet)


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
        print(f"\nPart 1:\nSum of indecies of packets in correct order: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nProduct of divider packet indecies: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)