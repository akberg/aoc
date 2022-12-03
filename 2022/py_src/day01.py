

def part1(inputs: str) -> int:
    """
    >>> s = "1000\\n2000\\n3000\\n\\n4000\\n\\n5000\\n6000\\n\\n7000\\n8000\\n9000\\n\\n10000"
    >>> part1(s)
    24000
    """
    return max([sum(map(int, block.splitlines())) for block in inputs.split("\n\n")])
def part2(inputs: str) -> int:
    """
    >>> s = "1000\\n2000\\n3000\\n\\n4000\\n\\n5000\\n6000\\n\\n7000\\n8000\\n9000\\n\\n10000"
    >>> part2(s)
    45000
    """
    arr = sorted([sum(map(int, block.splitlines())) for block in inputs.split("\n\n")])
    return sum(arr[-3:])

if __name__ == "__main__":
    with open("./inputs/day1.txt", "r") as f:
        inputs = f.read()

    print(f"Part 1: {part1(inputs)}")
    print(f"Part 2: {part2(inputs)}")
