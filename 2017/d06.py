with open('06.txt') as fd:
  data = [int(n) for n in fd.readline().strip().split('\t')]

def highest():
  mx = -1
  ix = -1
  for (i, x) in enumerate(data):
    if mx < x:
      (ix, mx) = (i, x)
  return (ix, mx)

def stri():
  return tuple(data)

seen = {}
c = 0
l = len(data)
while True:
  s = stri()
  if s in seen: break
  seen[s] = c
  c += 1
  (i, n) = highest()
  data[i] = 0
  for a in range(n):
    data[(i + a + 1) % l] += 1
print(c - seen[s], c)
