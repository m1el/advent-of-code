import re
vovels = re.compile(r'[aeiou]')
doubles = re.compile(r'(.)\1')
naughty = re.compile(r'ab|cd|pq|xy')
def test(s):
  vc = len(re.findall(vovels, s))
  dc = len(re.findall(doubles, s))
  nc = len(re.findall(naughty, s))
  return vc >= 3 and dc >= 1 and nc == 0

if __name__ == '__main__':
  text = open('day005.txt', 'r').read().splitlines()
  count = 0
  for l in text:
    if test(l):
      count += 1

  print(count)
