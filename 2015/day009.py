from itertools import permutations
import re

cities = set()
dists = {}
for line in open('day009.txt', 'r').readlines():
  m = re.match(r'(\w+) to (\w+) = (\d+)', line)
  if not m: continue
  [src, dst, dist] = m.groups()
  dists[(src, dst)] = int(dist)
  dists[(dst, src)] = int(dist)
  cities.add(src)
  cities.add(dst)
cities = list(cities)
minimal = None
maximal = None
for perm in permutations(cities):
  perm = zip(perm, perm[1:])
  total = sum(dists[pair] for pair in perm)
  if maximal is None or total > maximal:
    maximal = total
  if minimal is None or total < minimal:
    minimal = total
print(minimal)
print(maximal)
