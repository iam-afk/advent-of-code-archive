#!/usr/bin/env python3
from sys import stdin
from collections import defaultdict
from re import split

lines = stdin.readlines()

g = defaultdict(list)
for line in lines:
    v, *us = split(' <-> |, ', line.strip())
    for u in us:
        g[v].append(u)


def dfs(v):
    if v in use:
        return
    use.add(v)
    for u in g[v]:
        dfs(u)


use = set()
dfs('0')
print(len(use))

groups = 1
for v in g.keys():
    if v not in use:
        dfs(v)
        groups += 1
print(groups)
