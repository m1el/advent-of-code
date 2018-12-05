from collections import defaultdict

with open('04.txt') as fd:
  data = [line.strip().split(' ') for line in fd.readlines()]

valid = 0
for l in data:
  counts = defaultdict(int)
  for w in l:
    counts[''.join(sorted(w))] += 1
  if not [w for w in counts.values() if w > 1]:
    valid += 1
print(valid)
