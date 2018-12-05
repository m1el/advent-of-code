from collections import defaultdict
import re

def parse(s):
  ary = s.strip().split(' ')
  if ary[0] == 'value':
    return [ary[i] for i in [0, 1, 5]]
  elif ary[0] == 'bot':
    return [ary[i] for i in [0, 1, 5, 6, 10, 11]]

with open('day010.txt', 'r') as fd:
  inp = [parse(l) for l in fd.readlines()]

bots = defaultdict(list)
outputs = defaultdict(list)

queue = inp[:]
while len(queue):
  nextqueue = []
  for ary in queue:
    if ary[0] == 'value':
      [val, bot] = ary[1:]
      bots[bot].append(int(val))
    elif ary[0] == 'bot':
      [bot, lo_dstt, lo_dst, hi_dstt, hi_dst] = ary[1:]
      if len(bots[bot]) >= 2:
        (lo, hi) = (min(bots[bot]), max(bots[bot]))
        if (lo, hi) == (17, 61):
          print(bot)

        if lo_dstt == 'bot':
          bots[lo_dst].append(lo)
        elif lo_dstt == 'output':
          outputs[lo_dst].append(lo)

        if hi_dstt == 'bot':
          bots[hi_dst].append(hi)
        elif hi_dstt == 'output':
          outputs[hi_dst].append(hi)

      else:
        nextqueue.append(ary)
  queue = nextqueue
print(outputs)
print([outputs[i] for i in ['0', '1', '2']])
