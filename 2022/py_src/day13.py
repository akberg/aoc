

def part1(inputs: str) -> int:
    '''
    >>> part1("[1,1,3,1,1]\n[1,1,5,1,1]\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\n[7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]")
    13
    '''
    for pair in inputs.split("\n\n"):
        ps0, ps1 = pair.splitlines()
        p0, p1 = [], []
        stack = [[]]
        for c in ps0:
            if c == '[':
                stack.append([])
            elif c == ']':
                stack[-2].append(stack.pop())
