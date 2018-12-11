from collections import Counter,defaultdict
from itertools import product
import re

with open('07.txt') as fd:
  inp = [l.strip() for l in fd.readlines()]
deps = defaultdict(list)
rdeps = defaultdict(list)
keys = set()
for l in inp:
  [a,b] = re.findall('[sS]tep (.)', l)
  keys.add(a)
  keys.add(b)
  deps[b].append(a)
  rdeps[a].append(b)

path = ''
while True:
  start = [key for key in keys
           if key not in path and all((d in path) for d in deps[key])]
  if not start: break
  start.sort()
  path += start[0]
print(path)

started = set()
finished = set()
jobs = []

for step in range(1000000):
  finished = finished.union([j for (j,s) in jobs if s == step])
  jobs = [(j,s) for (j,s) in jobs if s > step]
  while len(jobs) < 5:
    visited = started.union(finished)
    start = [key for key in keys
             if key not in visited and all((d in finished) for d in deps[key])]
    start.sort()
    if not start: break
    key = start[0]
    started.add(key)
    jobs.append((key, 60+step+ord(key)-ord('@')))
  if len(jobs) == 0:
    break

print(step)
