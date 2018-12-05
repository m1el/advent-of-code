import hashlib
def md5(s): return hashlib.md5(s.encode()).hexdigest()
magic = 'qzthpkfp'

dirs = {
    'U': (0, 0, -1),
    'D': (1, 0, 1),
    'L': (2, -1, 0),
    'R': (3, 1, 0),
    }

free = 'bcdef'

def neighboors(x, y, path):
  cs = md5(magic + path)[0:4]
  for d, (i, dx, dy) in dirs.items():
    nx = x + dx
    ny = y + dy
    if nx < 0 or ny < 0 or nx > 3 or ny > 3 or cs[i] not in free:
      continue
    yield (nx, ny, d)

steps = 0
queue = [(0,0,'')]
found = False
paths = []
while len(queue):
  nextqueue = []
  for (x, y, path) in queue:
    for nx, ny, d in neighboors(x, y, path):
      if (nx, ny) == (3, 3):
        if not found:
          found = True
          print('found!')
          print(path + d)
        else:
          paths.append(path + d)
      else:
        nextqueue.append((nx, ny, path + d))
  if not len(nextqueue):
    print(paths[-1])
    print(len(paths[-1]))
  queue = nextqueue

  steps += 1
