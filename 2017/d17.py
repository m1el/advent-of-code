st = [0]
pos = 0
for i in range(1,2018):
  pos = (pos + 370) % len(st)
  st = st[:pos+1] + [i] + st[pos+1:]
  pos = (pos + 1) % len(st)
print(st[pos + 1])
l = 1
lz = 0
for i in range(1,50000000):
  pos = (pos + 370) % l
  if pos == 0: lz = i
  l += 1
  pos = (pos + 1) % l
print(lz)
