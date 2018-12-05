import re
with open('day007.txt', 'r') as fd:
  inp = fd.readlines()

def split(s):
  groups = s.split(']')
  outside = []
  inside = []
  for g in groups:
    oi = g.split('[')
    outside.append(oi[0])
    if len(oi) > 1:
      inside.append(oi[1])
  return (outside, inside)

def haspoly(s):
  for idx in range(0, len(s) - 3):
    if s[idx] != s[idx + 1] and (s[idx], s[idx + 1]) == (s[idx + 3], s[idx + 2]):
      return True
  return False

def poly2(s):
  for idx in range(0, len(s) - 2):
    if s[idx] != s[idx + 1] and s[idx] == s[idx + 2]:
      yield (s[idx], s[idx + 1])

counter = 0
counter2 = 0
for line in inp:
  outside, inside = split(line)
  good_outside = any((haspoly(g) for g in outside))
  good_inside = all((not haspoly(g) for g in inside))
  if good_outside and good_inside:
    print(line)
    counter += 1

  do_break = False
  for g in outside:
    if do_break: break
    for a, b in poly2(g):
      bab = b + a + b
      if any((bab in g for g in inside)):
        do_break = True
        counter2 += 1
print(counter)
print(counter2)
