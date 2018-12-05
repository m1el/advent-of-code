inp = bytearray(int(c) for c in '10111100110001111')

def step(ary):
  copy = bytearray(i ^ 1 for i in ary)
  copy.reverse()
  return ary + bytearray([0]) + copy

def csum(ary):
  while len(ary) % 2 == 0:
    it = iter(ary)
    ary = bytearray((a ^ b ^ 1) for a, b in zip(it, it))
  return ary

while len(inp) < 35651584:
  inp = step(inp)

cs = csum(inp[0:35651584])
out = ''.join(str(i) for i in cs)
print(out)
