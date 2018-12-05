from collections import defaultdict
data = []
available = defaultdict(list)
with open('24.txt') as fd:
  for i, l in enumerate(fd.readlines()):
    a, b = map(int, l.strip().split('/'))
    data.append((a, b))
    available[a].append(i)
    available[b].append(i)

def build(num, visited):
  yield visited
  for i in available[num]:
    if i in visited: continue
    visited.append(i)
    a, b = data[i]
    if num == b: (a, b) = (b, a)
    yield from build(b, visited)
    visited.pop()

mx = 0
ml = 0
mlx = 0
for n, v in enumerate(build(0, [])):
  if n % 100000 == 0: print(v)
  s = sum(sum(data[i]) for i in v)
  if s > mx: mx = s
  if len(v) == ml and s > mlx: mlx = s
  if len(v) > ml:
    ml = len(v)
    mlx = s
print(mx)
print(ml, mlx)
