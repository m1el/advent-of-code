from collections import defaultdict, Counter
from itertools import product
import re

with open('03.txt') as fd:
  inp = []
  for l in fd.readlines():
    groups = re.findall(r'\d+', l)
    inp.append(list(map(int, groups)))

claims = defaultdict(int)
for (id, l,t, w,h) in inp:
  for y in range(t,t+h):
    for x in range(l,l+w):
      claims[(x,y)] += 1
c=0
for n in claims.values():
  if n > 1: c+= 1

print(c)

for (id, l,t, w,h) in inp:
  bad = False
  for y in range(t,t+h):
    for x in range(l,l+w):
      if claims[(x,y)] > 1:
        bad = True
        break
    if bad: break
  if not bad:
    print(id)
