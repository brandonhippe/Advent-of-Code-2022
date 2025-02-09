import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2022 Day 7 Part 1

    >>> part1(['$ cd /', '$ ls', 'dir a', '14848514 b.txt', '8504156 c.dat', 'dir d', '$ cd a', '$ ls', 'dir e', '29116 f', '2557 g', '62596 h.lst', '$ cd e', '$ ls', '584 i', '$ cd ..', '$ cd ..', '$ cd d', '$ ls', '4060174 j', '8033020 d.log', '5626152 d.ext', '7214296 k'])
    95437
    """

    fileSystem = Directory("/")

    for line in data[1:]:
        if "$ cd .." in line:
            fileSystem = fileSystem.parent
        elif "$ cd" in line:
            for subDir in fileSystem.files:
                if isinstance(subDir, Directory) and subDir.name == line.split(" ")[2]:
                    fileSystem = subDir
                    break
        elif "dir" in line:
            fileSystem.files.append(Directory(line.split(" ")[1], fileSystem))
        elif line != "$ ls":
            fileSystem.files.append(File(line.split(" ")[1], int(line.split(" ")[0])))

    while fileSystem.parent:
        fileSystem = fileSystem.parent

    fileSystem.size()

    return sumDirectories(fileSystem)


def part2(data):
    """ 2022 Day 7 Part 2

    >>> part2(['$ cd /', '$ ls', 'dir a', '14848514 b.txt', '8504156 c.dat', 'dir d', '$ cd a', '$ ls', 'dir e', '29116 f', '2557 g', '62596 h.lst', '$ cd e', '$ ls', '584 i', '$ cd ..', '$ cd ..', '$ cd d', '$ ls', '4060174 j', '8033020 d.log', '5626152 d.ext', '7214296 k'])
    24933642
    """

    fileSystem = Directory("/")

    for line in data[1:]:
        if "$ cd .." in line:
            fileSystem = fileSystem.parent
        elif "$ cd" in line:
            for subDir in fileSystem.files:
                if isinstance(subDir, Directory) and subDir.name == line.split(" ")[2]:
                    fileSystem = subDir
                    break
        elif "dir" in line:
            fileSystem.files.append(Directory(line.split(" ")[1], fileSystem))
        elif line != "$ ls":
            fileSystem.files.append(File(line.split(" ")[1], int(line.split(" ")[0])))

    while fileSystem.parent:
        fileSystem = fileSystem.parent

    fileSystem.size()

    freeSpace = 70000000 - fileSystem.s
    needToDelete = 30000000 - freeSpace

    return smallestDelete(fileSystem, needToDelete)


class File:
    def __init__(self, name, size) -> None:
        self.name = name
        self.size = size


class Directory:
    def __init__(self, name, Parent = None) -> None:
        self.name = name
        self.parent = Parent
        self.files = []


    def size(self):
        self.s = 0
        for f in self.files:
            if isinstance(f, File):
                self.s += f.size
            else:
                f.size()
                self.s += f.s


def sumDirectories(filesystem):
    if filesystem.s <= 100000:
        count = filesystem.s
    else:
        count = 0

    for f in filesystem.files:
        if isinstance(f, Directory):
            count += sumDirectories(f)

    return count


def smallestDelete(fileSystem, needToDelete):
    if fileSystem.s < needToDelete:
        return None

    smallest = fileSystem.s
    for f in fileSystem.files:
        if isinstance(f, Directory):
            result = smallestDelete(f, needToDelete)
            if result:
                smallest = min(smallest, result)

    return smallest


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
        print(f"\nPart 1:\nTotal size of directories under 100000: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSmallest directory to make space for update: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)