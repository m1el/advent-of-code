from collections import Counter
from itertools import product

with open('02.txt') as fd:
  inp = [l.strip() for l in fd.readlines()]

twos = 0
thre = 0
for row in inp:
  c = Counter(row)
  if 2 in c.values():
    twos += 1
  if 3 in c.values():
    thre += 1

print(twos*thre)

def diff(sa,sb):
  c = 0
  for (a, b) in zip(sa,sb):
    if a != b: c+= 1
  return c

for (a, b) in product(inp, inp):
  if diff(a,b) == 1:
    print(a)
    print(b)
    break
