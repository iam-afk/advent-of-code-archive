#!/usr/bin/env python3
from sys import stdin

b = [int(s) for s in stdin.readline().strip().split()]
l = len(b)
banks = {}
cycles = 0
while True:
    cycles += 1
    m = max(b)
    p = b.index(m)
    b[p] = 0
    for i in range(l):
        b[i] += m // l
    if m % l != 0:
        for i in range(m % l):
            b[(p + i + 1) % l] += 1
    tb = tuple(b)
    if tb in banks:
        print(cycles)
        print(cycles - banks[tb])
        break
    banks[tb] = cycles
