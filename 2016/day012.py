with open('day012.txt', 'r') as fd:
  inp = [line.strip().split(' ') for line in fd.readlines()]

regs = {r: 0 for r in 'abcd'}
regs['c'] = 1
pc = 0
while True:
  if pc < 0 or pc >= len(inp):
    print(regs)
    break

  line = inp[pc]
  if line[0] == 'cpy':
    [src, dst] = line[1:3]
    if src in regs:
      src = regs[src]
    else:
      src = int(src)
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
    else:
      val = int(val)
    if val:
      pc += int(dst)
    else:
      pc += 1
  else:
    print(line[0])
    raise Exception('w00t')
