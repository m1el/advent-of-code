magic = 1364
def lookup(x, y):
  number = x*x + 3*x + 2*x*y + y + y*y + magic
  bits = bin(number).count("1")
  return bits % 2

def neighboors(x, y):
  for dx, dy in [(0, -1), (1, 0), (0, 1), (-1, 0)]:
    nx = x + dx
    ny = y + dy
    if nx < 0 or ny < 0 or lookup(nx, ny):
      continue
    yield (nx, ny)

visited = dict()
steps = 0
queue = set([(1,1)])
while True:
  nextqueue = set()
  for x, y in queue:
    #if (x, y) == (31, 39):
    #  print(steps)
    #  die()
    visited[(x, y)] = steps
    for nx, ny in neighboors(x, y):
      if (nx, ny) in visited or (nx, ny) in nextqueue:
        continue
      nextqueue.add((nx, ny))
  queue = nextqueue
  if steps == 50:
    print(len(visited.keys()))
    for y in range(40):
      print(''.join([
        'O' if (x,y) in visited else ('#' if lookup(x,y) else '.')
        for x in range(40)]))
    break

  steps += 1
