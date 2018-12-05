with open('day003.txt', 'r') as fd:
  inp = fd.read()

rows = iter(inp.split('\n'))
bad = 0
good = 0
while True:
  slice = []
  for _ in range(3):
    row = rows.next()
    if not row: break
    slice.append([int(col.strip()) for col in [row[0:5], row[5:10], row[10:15]]])
  if not slice: break
  for d in range(3):
    a, b, c = [slice[i][d] for i in range(3)]
    if a + b > c and a + c > b and b + c > a:
      good += 1
    else:
      bad += 1
print(good)
