import re
line_re = re.compile(r'(?P<name>\w+) can fly (?P<speed>\d+) km/s for (?P<fly>\d+) seconds, but then must rest for (?P<rest>\d+) seconds\.')
deers = []
for line in open('day014.txt', 'r').readlines():
  print(line)
  m = re.match(line_re, line)
  if not m: continue
  d = m.groupdict()
  deer = {
      'fly': int(d['fly']),
      'rest': int(d['rest']),
      'speed': int(d['speed']),
      'seconds': int(d['fly']),
      'state': 'fly',
      'distance': 0,
      'score': 0,
      }
  deers.append(deer)

switch = {'rest': 'fly', 'fly': 'rest'}

for i in range(2503):
  leader_dist = None
  for deer in deers:
    if deer['state'] == 'fly':
      deer['distance'] += deer['speed']
    elif deer['state'] == 'rest':
      pass
    deer['seconds'] -= 1
    if deer['seconds'] == 0:
      deer['state'] = switch[deer['state']]
      deer['seconds'] = deer[deer['state']]
    if leader_dist is None or deer['distance'] > leader_dist:
      leader_dist = deer['distance']
  for deer in deers:
    if deer['distance'] == leader_dist:
      deer['score'] += 1
for deer in deers:
  print(deer)
