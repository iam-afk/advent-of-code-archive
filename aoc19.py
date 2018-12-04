#!/usr/bin/env python3
from sys import stdin


g = [line for line in stdin.readlines()]
r, c, dr, dc = 0, g[0].index('|'), 1, 0
count = 1
while True:
    if dr != 0 and g[r][c] in '|-':
        r += dr
    elif dc != 0 and g[r][c] in '|-':
        c += dc
    elif g[r][c] == '+':
        if dr != 0 and 0 <= r + dr < len(g) and g[r + dr][c] in '-+':
            r += dr
        elif dc != 0 and 0 <= c + dc < len(g[r]) and g[r][c + dc] in '|+':
            c += dc
        elif dr != 0 and 0 < c and g[r][c - 1] in '-+':
            c -= 1
            dr = 0
            dc = -1
        elif dr != 0 and c + 1 < len(g[r]) and g[r][c + 1] in '-+':
            c += 1
            dr = 0
            dc = 1
        elif 0 < r and g[r - 1][c] in '|+':
            r -= 1
            dr = -1
            dc = 0
        elif r + 1 < len(g) and g[r + 1][c] in '|+':
            r += 1
            dr = 1
            dc = 0
    else:
        print(g[r][c], end='')
        r += dr
        c += dc
    if g[r][c] == ' ':
        break
    count += 1


print()
print(count)
