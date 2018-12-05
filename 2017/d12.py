from collections import deque

with open('12.txt') as fd:
  data = {}
  for l in fd.readlines():
    [node, links] = l.strip().split(' <-> ')
    data[node] = links.split(', ')

visited = set()

def unvisited():
  for k in data.keys():
    if k not in visited:
      yield k

groups = []

for u in unvisited():
  queue = deque([u])
  group = []
  while len(queue) > 0:
    node = queue.popleft()
    visited.add(node)
    group.append(node)
    for link in data[node]:
      if link not in visited:
        queue.append(link)
  groups.append(group)

print(len(groups[0]))
print(len(groups))
