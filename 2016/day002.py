with open('day002.txt', 'r') as fd:
  inp = fd.read()

rows = inp.split('\n')

def move(p, d):
  x, y = p
  if d == 'U': p = (x, max(y - 1, 0))
  if d == 'D': p = (x, min(y + 1, 2))
  if d == 'R': p = (min(x + 1, 2), y)
  if d == 'L': p = (max(x - 1, 0), y)
  return p
def key(p):
  return p[0] + p[1] * 3 + 1

p = (1, 1)
keys = []
for row in rows:
  for d in row:
    p = move(p, d)
  keys.append(str(key(p)))
print(''.join(keys))
