import re

num_re = re.compile(r'^\d+$')
uni_re = re.compile(r'^(?P<src>\S+) -> (?P<dst>\S+)$')
not_re = re.compile(r'^NOT (?P<src>\S+) -> (?P<dst>\S+)$')
bin_re = re.compile(r'^(?P<src0>\S+) (?P<op>\S+) (?P<src1>\S+) -> (?P<dst>\S+)$')
ops = {
    'AND': lambda x, y: x & y,
    'OR': lambda x, y: x | y,
    'LSHIFT': lambda x, y: (x << y) & 0xFFFF,
    'RSHIFT': lambda x, y: (x >> y) & 0xFFFF,
    }

def isint(x):
  return re.match(num_re, x) != None

def evil(wire, conns, data, level=0):
  if isint(wire):
    return int(wire)
  if wire in data:
    return data[wire]
  typ, arg = conns[wire]
  if typ == 'UNIT':
    ret = evil(arg, conns, data, level+1)
  elif typ == 'NOT':
    ret = evil(arg, conns, data, level+1) ^ 0xFFFF
  elif typ == 'BIN':
    src0, op, src1 = arg
    src0 = evil(src0, conns, data, level+1)
    src1 = evil(src1, conns, data, level+1)
    ret = ops[op](src0, src1)
  else:
    raise 'unknown type'
  data[wire] = ret
  return ret

text = open('day007.txt', 'r').read().splitlines()
conns = {}
for l in text:
  m = re.match(uni_re, l)
  if m:
    g = m.groupdict()
    conns[g['dst']] = ('UNIT', g['src'])
  m = re.match(not_re, l)
  if m:
    g = m.groupdict()
    conns[g['dst']] = ('NOT', g['src'])
  m = re.match(bin_re, l)
  if m:
    g = m.groupdict()
    conns[g['dst']] = ('BIN', (g['src0'], g['op'], g['src1']))
res = evil('a', conns, {})
print(res)

conns['b'] = ('UNIT', str(res))
print(evil('a', conns, {}))
