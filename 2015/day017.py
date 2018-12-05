bottles = [int(line.strip()) for line in open('day017.txt', 'r').readlines()]

def upto(ary, amt):
  if amt < 0:
    return
  if amt == 0:
    yield []
    return
  for i in range(len(ary)):
    for resp in upto(ary[i+1:], amt - ary[i]):
      yield [ary[i]] + resp

#bottles = [20, 15, 10, 5, 5]
c = 0
opti = 0
for r in upto(sorted(bottles, reverse=True), 150):
  if len(r) == 4:
    opti += 1
  print(r)
  c += 1
print(c)
print(opti)
