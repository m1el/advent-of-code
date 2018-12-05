import re
line_re = re.compile(r'(?P<action>turn on|turn off|toggle) (?P<x0>\d+),(?P<y0>\d+) through (?P<x1>\d+),(?P<y1>\d+)')
text = open('day006.txt', 'r').read().splitlines()
field = [[0 for x in range(1000)] for y in range(1000)]
for l in text:
  m = re.match(line_re, l)
  if not m: continue
  d = m.groupdict()
  print(d)
  action = d['action']
  x0, y0 = (int(d['x0']), int(d['y0']))
  x1, y1 = (int(d['x1']), int(d['y1']))
  if action == 'turn on':
    for y in range(y0, y1 + 1):
      for x in range(x0, x1 + 1):
        field[x][y] = 1
  elif action == 'turn off':
    for y in range(y0, y1 + 1):
      for x in range(x0, x1 + 1):
        field[x][y] = 0
  elif action == 'toggle':
    for y in range(y0, y1 + 1):
      for x in range(x0, x1 + 1):
        field[x][y] = 1-field[x][y]
s = 0
for y in range(1000):
  s += sum(field[y])
print(s)
