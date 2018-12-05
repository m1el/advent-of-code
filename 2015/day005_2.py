import re
double = re.compile(r'(..).*?\1')
repeat = re.compile(r'(.).\1')
def test(s):
  return re.findall(double, s) and re.findall(repeat, s)

if __name__ == '__main__':
  text = open('day005.txt', 'r').read().splitlines()
  count = 0
  for l in text:
    if test(l):
      count += 1

  print(count)
