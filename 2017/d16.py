with open('d16.txt') as fd:
  data = fd.read().strip().split(',')

progs = list('abcdefghijklmnop')
#progs = list('abcde')
#data = ['s1', 'x3/4', 'pe/b']
def dance(progs, data):
  for i in data:
    if i[0] == 's':
      num = int(i[1:])
      progs = progs[-num:] + progs[:-num]
    if i[0] == 'x':
      (p1, p2) = map(int, i[1:].split('/'))
      progs[p1], progs[p2] = progs[p2], progs[p1]
    if i[0] == 'p':
      (p1, p2) = i[1:].split('/')
      for i, p in enumerate(progs):
        if p == p1: progs[i] = p2
        if p == p2: progs[i] = p1
  return progs


print(''.join(dance(list(progs), data)))

seen = []
seenset = set()
BILLION = 1000000000
for i in range(BILLION):
  s = ''.join(progs)
  if s in seenset:
    progs = seen[BILLION % i]
    break
  seenset.add(s)
  seen.append(s)
  progs = dance(progs, data)

print(''.join(progs))
