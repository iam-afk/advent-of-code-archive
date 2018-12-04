#!/usr/bin/env python3
from sys import stdin
from collections import defaultdict


instr = stdin.readlines()


class Program(object):
    def __init__(self, id, read, write):
        self.ip = 0
        self.reg = defaultdict(int)
        self.reg['p'] = id
        self.send = 0
        self.read = read
        self.write = write

    def val(self, x):
        return self.reg[x] if x[0] not in '-0123456789' else int(x)

    def run(self):
        while self.ip >= 0 and self.ip < len(instr):
            cmd = instr[self.ip].strip().split()
            if cmd[0] == 'snd':
                self.send += 1
                self.write.append(self.val(cmd[1]))
            elif cmd[0] == 'set':
                self.reg[cmd[1]] = self.val(cmd[2])
            elif cmd[0] == 'add':
                self.reg[cmd[1]] = self.reg[cmd[1]] + self.val(cmd[2])
            elif cmd[0] == 'mul':
                self.reg[cmd[1]] = self.reg[cmd[1]] * self.val(cmd[2])
            elif cmd[0] == 'mod':
                self.reg[cmd[1]] = self.reg[cmd[1]] % self.val(cmd[2])
            elif cmd[0] == 'rcv':
                if not self.read:
                    return 'rcv'
                self.reg[cmd[1]] = self.read.pop(0)
            elif cmd[0] == 'jgz':
                if self.val(cmd[1]) > 0:
                    self.ip += self.val(cmd[2]) - 1
            self.ip += 1
        return 'end'


q1 = []
q2 = []
p1 = Program(1, q1, q2)
p2 = Program(0, q2, q1)

while True:
    s1 = p1.run()
    if s1 == 'rcv':
        s2 = p2.run()
        if not q1:
            break
    elif s1 == 'end':
        break
print(p1.send)
