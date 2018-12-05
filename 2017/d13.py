data = []
with open('13.txt') as fd:
  for l in fd.readlines():
    l = l.strip().split(': ')
    data.append((int(l[0]), int(l[1])))

def severity(data, delay=0):
  sev = 0
  caught = False
  for (r, d) in data:
    t = (r + delay) % (d * 2 - 2)
    if t > d:
      t = 2 * d - 1 - t
    if t == 0:
      caught = True
      sev += r * d
  return (caught, sev)

print(severity(data))

for i in range(10000000000000):
  if not severity(data, i)[0]:
    print(i)
    break
