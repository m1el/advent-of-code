from collections import defaultdict
from itertools import cycle

with open('01.txt') as fd:
  data = list(map(int, fd.readlines()))

def part2(data):
  count = defaultdict(int)
  freq = 0
  count[freq] += 1
  for d in cycle(data):
    freq += d
    if count[freq] != 0:
      return freq
    else:
      count[freq] += 1

def part2_analytical(data):
  current = 0
  freqs = [current]
  for diff in data:
    current += diff
    freqs.append(current)

  matches = {}
  size = len(data)
  shift = sum(data)
  min_key = None # (cycle, index)
  for y in range(0, size):
    for x in range(y+1, size):
      (fx, fy) = (freqs[x], freqs[y])
      diff = fx - fy

      if shift == 0 and diff == 0:
        cycle = 0
      elif shift != 0 and diff % shift == 0:
        cycle = diff // shift
      else:
        continue

      if cycle >= 0:
        key = (cycle, y)
        matches[key] = fy + cycle * shift
      else:
        key = (-cycle, x)
        matches[key] = fx - cycle * shift

      if min_key is None or min_key > key:
        min_key = key

  if min_key in matches:
    return matches[min_key]

# data = [+1000000000, -999999999]
print('part1: {}'.format(sum(data)))
# print('part2: {}'.format(part2(data)))
print('part2: {}'.format(part2_analytical(data)))
