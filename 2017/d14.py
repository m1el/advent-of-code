from d10 import knot_hash

def bit_count(int_type):
    count = 0
    while(int_type):
        int_type &= int_type - 1
        count += 1
    return(count)

inp = 'wenycdww'

count = 0
disk = []
for i in range(128):
  line = '{}-{}'.format(inp, i).encode()
  h = knot_hash(list(line))
  row = []
  for b in h:
    row += [-int(x) for x in '{:08b}'.format(b)]
    count += bit_count(b)
  disk.append(row)

def mark_region(x, y, n):
  stack = [(x, y)]
  while len(stack) > 0:
    (x, y) = stack.pop()
    disk[y][x] = n
    for dx, dy in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
      (nx, ny) = (x + dx, y + dy)
      if nx < 0 or nx >= 128 or ny < 0 or ny >= 128: continue
      if disk[ny][nx] != -1: continue
      stack.append((nx, ny))

n = 0
for y in range(128):
  for x in range(128):
    if disk[y][x] == -1:
      n += 1
      mark_region(x, y, n)

print(count)
print(n)
