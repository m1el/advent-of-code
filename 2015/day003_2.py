from collections import defaultdict

def add(a, b):
  return (a[0] + b[0], a[1] + b[1])

dirs = {
    '<': (-1, 0),
    '>': (1, 0),
    'v': (0, -1),
    '^': (0, 1),
    }

text = open('day003.txt', 'r').read()
houses = defaultdict(int)
pos1 = (0, 0)
pos2 = (0, 0)

houses[pos1] += 1

a = iter(text)
for c1, c2 in zip(a, a):
  if c1 not in dirs: continue
  if c2 not in dirs: continue
  d1 = dirs[c1]
  d2 = dirs[c2]
  pos1 = add(pos1, d1)
  pos2 = add(pos2, d2)
  houses[pos1] += 1
  houses[pos2] += 1
print(len(houses.keys()))
