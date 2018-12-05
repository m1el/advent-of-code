import re
data = []
with open('07.txt') as fd:
  for l in fd.readlines():
    m = re.match(r'(\w+) \((\d+)\)(:? -> (.*))?', l)
    n = m.group(1)
    c = m.group(2)
    w = m.group(4)
    if w: w = w.split(', ')
    else: w = []
    data.append((n, int(c), w))

g = {}
ps = set()
cs = set()
for (n, c, w) in data:
  ps.add(n)
  cs.update(w)
  g[n] = [c, w, 0]
ts = ps.difference(cs)
top = [x for x in ts][0]

def get_twr(n):
  t = g[n]
  return [n, t[0]] + [get_twr(x) for x in t[1]]
print(top)

def balanced(n):
  t = g[n]
  w = t[0]
  children = [balanced(c) for c in t[1]]
  if not children: return w
  cx = max(children)
  ci = min(children)
  if cx != ci:
    if children.count(cx) == 1:
      idx = children.index(cx)
      s = -1
    if children.count(ci) == 1:
      idx = children.index(ci)
      s = 1
    ub = t[1][idx]
    ubn = g[ub]
    print(t[1])
    print(children)
    print(ub, 'unbalanced', ubn[0], ubn[0] + (cx - ci) * s)
  w += sum(children)
  return w
balanced(top)
