from itertools import permutations
with open('02.txt') as fd:
  data = fd.read()
  lines = [
      [int(x) for x in l.split('\t') if x]
        for l in data.split('\n') if l]

s = 0
for l in lines:
  s += max(l) - min(l)
print(s)

s = 0
for l in lines:
  for [a, b] in permutations(l, 2):
    if a % b == 0:
      s += a // b
print(s)
