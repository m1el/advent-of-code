with open('11.txt') as fd:
  data = fd.read().strip().split(',')

def sign(x):
  if x > 0: return 1
  elif x < 0: return -1
  else: return 0

def hex_dist(dx, dy):
  if sign(dx) == sign(dy):
    return abs(dx + dy)
  else:
    return max(abs(dx), abs(dy))

x = 0
y = 0
md = 0
for d in data:
  if d == 'n': y += 1
  elif d == 's': y -= 1
  elif d == 'ne': x += 1
  elif d == 'sw': x -= 1
  elif d == 'nw':
    x -= 1
    y += 1
  elif d == 'se':
    x += 1
    y -= 1

  dist = hex_dist(x, y)
  if dist > md: md = dist

print(dist)
print(md)
