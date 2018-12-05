text = open('day001.txt', 'r').read()
floor = 0
for i, c in enumerate(text):
  if c == '(': floor += 1
  if c == ')': floor -= 1
  if floor < 0:
    print(i + 1)
    break
