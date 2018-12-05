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
pos = (0, 0)

houses[pos] += 1

for c in text:
  if c not in dirs: continue
  d = dirs[c]
  pos = add(pos, d)
  houses[pos] += 1
print(len(houses.keys()))
