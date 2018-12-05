import re
with open('04.txt') as fd:
  inp = fd.readlines()

inp.sort()
guards = {}
current = None
start = None
for line in inp:
  minute = int(line[15:17])
  m = re.search(r'(\d+) begins shift', line)
  if m:
    current = int(m.group(1))
    if current not in guards:
      guards[current] = [0]*60
  elif 'falls asleep' in line:
    start = minute
  elif 'wakes up' in line:
    guard = guards[current]
    for m in range(start,minute+1):
      guard[m] += 1

(id, guard) = max(guards.items(), key=lambda kv:sum(kv[1]))
max_minute = max(range(60), key=lambda m:guard[m])
print(id*max_minute)

(id, guard) = max(guards.items(), key=lambda kv:max(kv[1]))
max_minute = max(range(60), key=lambda m:guard[m])
print(id*max_minute)
