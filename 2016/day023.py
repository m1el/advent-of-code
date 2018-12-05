def intish(s):
  try:
    return int(s)
  except ValueError as e:
    return s

with open('day023.txt') as fd:
  inp = [list(map(intish, l.strip().split(' '))) for l in fd.readlines() if l]

regs = {r: 0 for r in 'abcd'}
swtbl = {
    'jnz': 'cpy',
    'cpy': 'jnz',
    'inc': 'dec',
    'dec': 'inc',
    'tgl': 'inc',
    }
regs['a'] = 12
pc = 0
while True:
  if pc >= len(inp):
    break
  line = inp[pc]
  #print(pc, line)
  if line[0] == 'cpy':
    [src, dst] = line[1:3]
    if src in regs:
      src = regs[src]
    if dst in regs:
      regs[dst] = src
    pc += 1
  elif line[0] == 'inc':
    reg = line[1]
    regs[reg] += 1
    pc += 1
  elif line[0] == 'dec':
    reg = line[1]
    regs[reg] -= 1
    pc += 1
  elif line[0] == 'jnz':
    [val, dst] = line[1:3]
    if val in regs:
      val = regs[val]
    if dst in regs:
      dst = regs[dst]
    if val:
      pc += dst
    else:
      pc += 1
  elif line[0] == 'tgl':
    dst = regs[line[1]] + pc
    if dst >= 0 and dst < len(inp):
      ch = inp[dst]
      print('tgl', pc, dst, ch)
      ch[0] = swtbl[ch[0]]
    pc += 1
  elif line[0] == 'mulab':
    regs['a'] = regs['a'] * regs['b']
    regs['c'] = 0
    regs['d'] = 0
    pc += 1
  elif line[0] == 'nop':
    pc += 1
  else:
    print(line[0])
    raise Exception('w00t')

print(regs['a'])
