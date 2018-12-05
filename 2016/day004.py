import re
from collections import defaultdict

with open('day004.txt', 'r') as fd:
  inp = fd.readlines()

def count(s):
  counts = defaultdict(int)
  for c in s:
    counts[c] += 1
  return counts
'''
inp = [
    'aaaaa-bbb-z-y-x-123[abxyz]',
    'a-b-c-d-e-f-g-h-987[abcde]',
    'not-a-real-room-404[oarel]',
    'totally-real-room-200[decoy]',
    ]
'''
alphabet = 'abcdefghijklmnopqrstuvwxyz'
def decode(s, shift):
  res = ''
  for c in s:
    if c == '-':
      res += ' '
    else:
      idx = alphabet.index(c) + shift
      res += alphabet[idx % len(alphabet)]
  return res
idsum = 0
for line in inp:
  mat = re.match(r'^(?P<id>(?:[a-z]+-)*)(?P<sector>\d+)\[(?P<csum>[a-z]+)\]$', line)
  if not mat: continue
  data = mat.groupdict()
  sector = int(data['sector'])
  counts = count(data['id'].replace('-', ''))
  counts = list(counts.items())
  counts.sort(key=lambda x: (-x[1], x[0]))
  common = ''.join([c for c, n in counts][0:5])
  decoded = decode(data['id'], sector)
  print(decoded)
  if 'northpole' in decoded:
    print(sector)
  if common == data['csum']:
    idsum += sector
print(idsum)
