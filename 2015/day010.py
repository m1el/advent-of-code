import re
def say(s):
  return re.sub(r'((\d)\2*)', lambda s: str(len(s.group(0)))+s.group(0)[0], s)
s = '3113322113'
for i in range(50):
  s = say(s)
print(len(s))
