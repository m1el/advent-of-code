import json
import re
def unesc(s):
  return re.sub(r'\\\\|\\x..|\\"', '.', s)

def esc(s):
  return re.sub(r'\\|"', r'\\\0', s)

total = 0
total2 = 0
for line in open('day008.txt', 'r').readlines():
  line = line.strip()
  raw = unesc(line)
  escesc = esc(line)
  total += len(line) - (len(raw) - 2)
  total2 += len(escesc) + 2 - len(line)
print(total)
print(total2)
