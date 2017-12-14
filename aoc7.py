#!/usr/bin/env python3
from sys import stdin
import re

programs = {}
notroot = set()
for line in stdin.readlines():
    name, weight, *above = re.split(' \(|\) -> |\)|, ', line.strip())
    above = [name for name in above if name]
    programs[name] = int(weight), above
    for name in above:
        notroot.add(name)
root = next(iter(programs.keys() - notroot))
print(root)


def tree_weight(root):
    weight, above = programs[root]
    if not above:
        return weight
    if len(above) < 3:
        return weight + sum((tree_weight(name) for name in above))
    zero, first, second, *others = [tree_weight(name) for name in above]
    if zero == first:
        if zero != second:
            print(programs[above[2]][0] + (zero - second))
            second = zero
        else:
            for i in range(3, len(above)):
                if others[i - 3] != zero:
                    print(programs[above[i]]._1 + (zero - others[i - 3]))
                    others[i - 3] = zero
    else:
        if zero == second:
            print(programs[above[1]][0] + (zero - first))
            first = zero
        else:
            print(programs[above[0]][0] + (first - zero))
            zero = first
    return weight + zero + first + second + sum(others)


tree_weight(root)
