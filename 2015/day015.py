import re

ingridients = []
for line in open('day015.txt', 'r').readlines():
  [name, rest] = line.split(':')
  matches = re.findall('(\w+) (-?\d+)', rest)
  ing = {'name': name}
  for m in matches:
    ing[m[0]] = int(m[1])
  ingridients.append(ing)

def addup(slots, total):
  if slots == 1:
    yield [total]
    return
  for i in range(total + 1):
    for rest in addup(slots - 1, total - i):
      yield [i] + rest

props = ['capacity', 'durability', 'flavor', 'texture']
aprops = ['capacity', 'durability', 'flavor', 'texture', 'calories']
best = None
for p in addup(len(ingridients), 100):
  scores = {prop: 0 for prop in aprops}
  for ing, amt in zip(ingridients, p):
    for prop in aprops:
      scores[prop] += ing[prop]*amt
  if scores['calories'] != 500:
    continue
  score = 1
  for prop in props:
    score *= max(0, scores[prop])
  if best is None or best < score:
    best = score
print(best)
