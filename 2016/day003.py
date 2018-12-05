with open('day003.txt', 'r') as fd:
  inp = fd.read()

rows = inp.split('\n')
bad = 0
good = 0
for row in rows:
  if not row: break
  a, b, c = [int(col.strip()) for col in [row[0:5], row[5:10], row[10:15]]]
  print(a,b,c)
  if a + b > c and a + c > b and b + c > a:
    good += 1
  else:
    bad += 1
print(good)
