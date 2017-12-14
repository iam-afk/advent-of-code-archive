#!/usr/bin/env python3
from sys import stdin
from itertools import count

number = int(stdin.readline().strip())

counter = 1
stop1 = stop2 = False

current = [1]
inner = []


def index(r, c, ring):
    if c == ring and r > -ring:
        return 0 * ring + r + ring - 1
    elif r == ring and c < ring:
        return 2 * ring + ring - c - 1
    elif c == -ring and r < ring:
        return 4 * ring + ring - r - 1
    elif r == -ring and c > -ring:
        return 6 * ring + c + ring - 1


def v(r, c, ring):
    global inner, current
    if r < -ring or r > ring or c < -ring or c > ring:
        return 0
    if r == 0 and c == 0:
        return 1
    if abs(r) == ring or abs(c) == ring:
        return current[index(r, c, ring)]
    elif abs(c) == ring - 1 or abs(r) == ring - 1:
        return inner[index(r, c, ring - 1)]


def grid(r, c, ring):
    global counter, stop1, stop2
    counter += 1
    if counter == number:
        print(' *', abs(r) + abs(c))
        stop1 = True
    if not stop2:
        value = sum((v(r + dr, c + dc, ring) for dr in range(-1, 2) for dc in range(-1, 2)))
        current[index(r, c, ring)] = value
        if value > number:
            print('**', value)
            stop2 = True


def side_range(ring):
    return range(-ring + 1, ring + 1)


def reverse_side_range(ring):
    return range(ring - 1, -ring - 1, -1)


for ring in count(1):
    side = 2 * ring
    inner = current
    current = [0] * (8 * ring)
    for r in side_range(ring):
        grid(r, ring, ring)
    for c in reverse_side_range(ring):
        grid(ring, c, ring)
    for r in reverse_side_range(ring):
        grid(r, -ring, ring)
    for c in side_range(ring):
        grid(-ring, c, ring)
    if stop1 and stop2:
        break
