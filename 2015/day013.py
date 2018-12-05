import re
from collections import defaultdict
from itertools import permutations

line_re = re.compile(r'^(?P<name>\w+) would (?P<action>lose|gain) (?P<amount>\d+) happiness units by sitting next to (?P<other>\w+)\.$')

names = set()
graph = defaultdict(int)

for line in open('day013.txt', 'r').readlines():
  m = re.match(line_re, line)
  if not m: continue
  d = m.groupdict()
  amount = int(d['amount'])
  if d['action'] == 'lose':
    amount = -amount
  graph[(d['name'], d['other'])] = amount
  names.add(d['name'])

def close_pairs(perm):
  shifted = [perm[-1]] + perm[0:-1]
  return zip(perm, shifted)

def close_pairs_2(perm):
  return zip(['me'] + perm, perm + ['me'])

perms = list(permutations(list(names)))
optimal = None
for perm in perms:
  pairs = list(close_pairs_2(list(perm)))
  score = sum(graph[(a,b)]+graph[(b,a)] for a,b in pairs)
  if optimal is None or score > optimal:
    optimal = score
print(optimal)
