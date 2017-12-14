#!/usr/bin/env python3
N = 256
KEY = 'jxqlasbh'


def knot_hash(key):
    lengths = list(map(ord, key)) + [17, 31, 73, 47, 23]
    h = list(range(N))
    pos = 0
    skip = 0
    for _ in range(64):
        for length in lengths:
            if pos + length <= N:
                h[pos:pos+length] = h[pos:pos+length][::-1]
            else:
                t = h + h
                t[pos:pos+length] = t[pos:pos+length][::-1]
                h = t[N:N+pos] + t[pos:N]
            pos = (pos + length + skip) % N
            skip += 1
    r = ''
    block = 0
    for i, x in enumerate(h):
        block ^= x
        if i & 0xF == 0xF:
            r += hex(block)[2:].zfill(2)
            block = 0
    return r


bits = {}
for x in range(16):
    bits[hex(x)[2:]] = bin(x)[2:].zfill(4)

grid = []
for row in range(128):
    h = knot_hash(KEY + '-' + str(row))
    grid.append([b for c in h for b in bits[c]])

print(sum(1 for row in grid for c in row if c == '1'))


def dfs(i, j):
    global grid
    grid[i][j] = '.'
    if i > 0 and grid[i - 1][j] == '1':
        dfs(i - 1, j)
    if i < 127 and grid[i + 1][j] == '1':
        dfs(i + 1, j)
    if j > 0 and grid[i][j - 1] == '1':
        dfs(i, j - 1)
    if j < 127 and grid[i][j + 1] == '1':
        dfs(i, j + 1)


regions = 0
for i in range(128):
    for j in range(128):
        if grid[i][j] == '1':
            dfs(i, j)
            regions += 1
print(regions)
