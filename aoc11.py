#!/usr/bin/env python3
from sys import stdin

s = stdin.readline().strip()

furthest = 0
x = 0
y = 0
z = 0
for m in s.split(','):
    if m == 'ne':
        y += 1
        z += 1
    elif m == 'sw':
        y -= 1
        z -= 1
    elif m == 'se':
        x -= 1
        z += 1
    elif m == 'nw':
        x += 1
        z -= 1
    elif m == 'n':
        x += 1
        y += 1
    elif m == 's':
        x -= 1
        y -= 1
    furthest = max(furthest, max(x, y, z))

print(max(abs(x), abs(y), abs(z)))
print(furthest)
