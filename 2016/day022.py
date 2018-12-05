import re
from pprint import pprint
with open('day022.txt') as fd:
  fd.readline()
  fd.readline()
  lines = [l for l in fd.readlines() if l]

grid = [[0]*38 for _ in range(26)]
for l in lines:
  l = re.split(r' +', l)
  print(l)
  pos = l[0].split('-')
  x, y = (int(pos[1][1:]), int(pos[2][1:]))
  size, used = (int(l[1][:-1]), int(l[2][:-1]))
  free = size - used
  grid[y][x] = (size, used, free)


pprint(grid)
sizes = []
useds = []
frees = []
for y, r in enumerate(grid):
  for x, (s, u, f) in enumerate(r):
    useds.append((u, x, y))
useds.sort()
frees.sort()
pprint(useds)
pprint(frees)
mgrid = [[0]*38 for _ in range(26)]
path = [(x, y) for u, x, y in useds if u <= 94 and u > 0]
for x, y in path:
  print(x, y)
  mgrid[y][x] = 1
for r in mgrid:
  print(''.join(map(str, r)))
