#!/usr/bin/env python3
from sys import stdin
from itertools import count

lines = stdin.readlines()

levels = [tuple(map(int, line.strip().split(': '))) for line in lines]

severity = sum(d * r for d, r in levels if d % (2 * r - 2) == 0)
print(severity)

for i in count(1):
    caught = False
    for d, r in levels:
        if (d + i) % (2 * r - 2) == 0:
            caught = True
            break
    if not caught:
        print(i)
        break
