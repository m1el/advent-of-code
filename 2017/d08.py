from collections import defaultdict
regs = defaultdict(int)
cmpd = {
    '>': lambda a, b: a > b,
    '<': lambda a, b: a < b,
    '>=': lambda a, b: a >= b,
    '<=': lambda a, b: a <= b,
    '==': lambda a, b: a == b,
    '!=': lambda a, b: a != b,
    }
mx = 0
with open('08.txt') as fd:
  for l in fd.readlines():
    [sreg, sop, snum, _, rreg, rop, rnum] = l.split(' ')
    sop = -1 if sop == 'dec' else 1
    snum = int(snum)
    rnum = int(rnum)
    if cmpd[rop](regs[rreg], rnum):
      regs[sreg] += sop * snum
      if mx < regs[sreg]: mx = regs[sreg]

print(max(regs.values()))
print(mx)
