#!/usr/bin/env python3
from sys import stdin

dance = 's1,x3/4,pe/b'
dance = stdin.readline().strip()
programs = [chr(ord('a') + i) for i in range(16)]
for _ in range(10 ** 9):
    for move in dance.split(','):
        if move[0] == 's':
            spin = int(move[1:])
            programs = programs[-spin:] + programs[:-spin]
        elif move[0] == 'x':
            a, b = map(int, move[1:].split('/'))
            programs[a], programs[b] = programs[b], programs[a]
        else:
            a, b = programs.index(move[1]), programs.index(move[3])
            programs[a], programs[b] = programs[b], programs[a]
    
print(''.join(programs))
