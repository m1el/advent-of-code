from gmpy2 import popcount

inp = '.^^..^...^..^^.^^^.^^^.^^^^...^^^^.^.^.^^^.^^.^^.'
mask = (1 << len(inp)) - 1
row = int(inp.replace('.', '0').replace('^', '1'), 2)

visited = {}
safe = 100 - popcount(row)
i = 0
while True:
  if row in visited:
    print(i)
    die()
  row = ((row << 1) ^ (row >> 1)) & mask
  safe += 100 - popcount(row)
  i += 1
print(safe)
