import re

aunts = []
for line in open('day016.txt', 'r').readlines():
  [name, rest] = line.split(':', 1)
  matches = re.findall('(\w+): (\d+)', rest)
  aunt = {'name': name}
  for m in matches:
    aunt[m[0]] = int(m[1])
  aunts.append(aunt)

pattern = {
  'children': 3,
  'samoyeds': 2,
  'pomeranians': 3,
  'akitas': 0,
  'vizslas': 0,
  'goldfish': 5,
  'cats': 7,
  'trees': 3,
  'cars': 2,
  'perfumes': 1,
}

for aunt in aunts:
  matches = True
  for k, v in pattern.items():
    if k in aunt and aunt[k] != v:
      matches = False
  if matches:
    print(aunt['name'])
