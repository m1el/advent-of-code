import re
regs = {'a': 1, 'b': 0}
prog = []
for line in open('day023.txt', 'r').readlines():
  m = re.match(r'hlf (\w)', line)
  if m: prog.append(('hlf', m.group(1)))
  m = re.match(r'tpl (\w)', line)
  if m: prog.append(('tpl', m.group(1)))
  m = re.match(r'inc (\w)', line)
  if m: prog.append(('inc', m.group(1)))
  m = re.match(r'jmp ([+-]\d+)', line)
  if m: prog.append(('jmp', int(m.group(1))))
  m = re.match(r'jie (\w), ([+-]\d+)', line)
  if m: prog.append(('jie', m.group(1), int(m.group(2))))
  m = re.match(r'jio (\w), ([+-]\d+)', line)
  if m: prog.append(('jio', m.group(1), int(m.group(2))))

PC = 0
while True:
  if PC < 0 or PC >= len(prog):
    break
  inst = prog[PC]
  print(PC, inst)
  offs = 1
  if inst[0] == 'hlf':
    regs[inst[1]] = regs[inst[1]] // 2
  elif inst[0] == 'tpl':
    regs[inst[1]] = regs[inst[1]] * 3
  elif inst[0] == 'inc':
    regs[inst[1]] = regs[inst[1]] + 1
  elif inst[0] == 'jmp':
    offs = inst[1]
  elif inst[0] == 'jie':
    if regs[inst[1]] % 2 == 0:
      offs = inst[2]
  elif inst[0] == 'jio':
    if regs[inst[1]] == 1:
      offs = inst[2]
  PC += offs

print(regs['b'])
