#!/usr/bin/env python3
from sys import stdin

N = 256

sparse_hash = list(range(N))
lengths = list(map(ord, stdin.readline().strip())) + [17, 31, 73, 47, 23]
current = 0
skip = 0
for _ in range(64):
    for length in lengths:
        if current + length < N:
            sparse_hash[current:current+length] = sparse_hash[current:current+length][::-1]
        else:
            t = sparse_hash + sparse_hash
            t[current:current+length] = t[current:current+length][::-1]
            sparse_hash = t[N:N+current] + t[current:N]

        current += length + skip
        current = current % N
        skip += 1
dense_hash = ''
element = 0
for i, e in enumerate(sparse_hash):
    element ^= e
    if i & 0xf == 0xf:
        dense_hash += hex(element)[2:]
        element = 0

print(sparse_hash[0] * sparse_hash[1])
print(dense_hash)
