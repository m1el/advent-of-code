from gmpy2 import popcount

inp = '.^..^....^....^^.^^.^.^^.^.....^.^..^...^^^^^^.^^^^.^.^^^^^^^.^^^^^..^.^^^.^^..^.^^.^....^.^...^^.^.'
mask = (1 << len(inp)) - 1
row = int(inp.replace('.', '0').replace('^', '1'), 2)

safe = 100 - popcount(row)
for i in range(400000-1):
  row = ((row << 1) ^ (row >> 1)) & mask
  safe += 100 - popcount(row)
print(safe)
