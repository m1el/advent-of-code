from collections import defaultdict
from string import ascii_lowercase

with open('18.txt') as fd:
  prog = [line.strip().split(' ') for line in fd.readlines()]

IP = 0
regs = defaultdict(int)

while True:
  if IP < 0 or IP >= len(prog): break
  inst = prog[IP]

  if inst[0] == 'snd':
    regs['snd'] = regs[inst[1]]
  elif inst[0] == 'rcv':
    val = regs['snd']
    if val != 0:
      print(val)
      regs[inst[1]] = val
      break
  elif inst[0] == 'set':
    val = inst[2]
    val = regs[val] if val in ascii_lowercase else int(val)
    regs[inst[1]] = val
  elif inst[0] == 'add':
    val = inst[2]
    val = regs[val] if val in ascii_lowercase else int(val)
    regs[inst[1]] += val
  elif inst[0] == 'mul':
    val = inst[2]
    val = regs[val] if val in ascii_lowercase else int(val)
    regs[inst[1]] *= val
  elif inst[0] == 'mod':
    val = inst[2]
    val = regs[val] if val in ascii_lowercase else int(val)
    regs[inst[1]] %= val
  elif inst[0] == 'jgz':
    val = regs[inst[1]] if inst[1] in ascii_lowercase else int(inst[1])
    off = regs[inst[2]] if inst[2] in ascii_lowercase else int(inst[2])
    if val > 0:
      IP += off
      continue
  else:
    raise Exception('unknown instruction ' + inst[0])
  IP += 1
