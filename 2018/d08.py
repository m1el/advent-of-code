with open('08.txt') as fd:
  inp = [int(i) for i in fd.readline().strip().split(' ')]

pos = 0

def read_node(inp):
  global pos
  nc, pos = inp[pos], pos+1
  md,pos = inp[pos], pos+1
  children = []
  meta = []
  for _ in range(nc):
    children.append(read_node(inp))
  for _ in range(md):
    meta.append(inp[pos])
    pos += 1
  children = dict(enumerate(children))
  return {'meta': meta, 'children': children}

def total(root):
  return sum(root['meta']) + sum(map(total, root['children'].values()))

empty = {'meta': [], 'children': []}
def total2(root):
  if len(root['children']) == 0:
    return sum(root['meta'])
  else:
    children = root['children']
    return sum(map(lambda key: total2(children.get(key-1, empty)), root['meta']))

root = read_node(inp)
print(root)
print(total(root))
print(total2(root))
