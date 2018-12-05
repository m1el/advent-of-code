import re
lines = iter(open('day019.txt', 'r').readlines())

distinct = set()
reps = []
for line in lines:
  if not line.strip(): break
  [a,b] = line.strip().split(' => ')
  reps.append((a,b))

source = next(lines)

for s, r in reps:
  for match in re.finditer('({})'.format(s), source):
    distinct.add(source[:match.start(0)] + r + source[match.end(0):])

for s in distinct:
  print(s)

print(len(distinct))
