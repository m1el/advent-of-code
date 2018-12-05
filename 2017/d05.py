with open('05.txt') as fd:
  jumps = [int(x) for x in fd.readlines()]

ip = 0
c = 0

while ip < len(jumps):
  c += 1
  (jumps[ip], ip) = (jumps[ip] + 1, jumps[ip] + ip)

print(c)

ip = 0
c = 0

with open('05.txt') as fd:
  jumps = [int(x) for x in fd.readlines()]

while ip < len(jumps):
  c += 1
  o = jumps[ip]
  (jumps[ip], ip) = (o + (-1 if o >= 3 else 1), o + ip)

print(c)
