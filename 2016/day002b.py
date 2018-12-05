with open('day002.txt', 'r') as fd:
  inp = fd.read()

rows = inp.split('\n')

keypad = [
    [0, 0, 1, 0, 0],
    [0, 2, 3, 4, 0],
    [5, 6, 7, 8, 9],
    [0, 'A', 'B', 'C', 0],
    [0, 0, 'D', 0, 0],
    ]
def move(p, d):
  x, y = p
  if d == 'U': y -= 1
  if d == 'D': y += 1
  if d == 'R': x += 1
  if d == 'L': x -= 1
  if x > 4 or x < 0 or y > 4 or y < 0:
    return p
  elif key((x, y)) == 0:
    return p
  else:
    return (x, y)
def key(p):
  return keypad[p[1]][p[0]]

p = (0, 2)
keys = []
for row in rows:
  for d in row:
    p = move(p, d)
  keys.append(str(key(p)))
print(''.join(keys))
