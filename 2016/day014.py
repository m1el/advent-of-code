import hashlib
import re
def md5(s):
  return hashlib.md5(s.encode()).hexdigest()

class md5strim(object):
  buf = None
  val = None
  pos = None
  bufsize = 1000
  def __init__(self, start):
    self.val = start
    self.pos = -1
    self.buf = [None]*self.bufsize

  def get(self, idx):
    if idx < self.pos - self.bufsize:
      raise Exception('whoops')
    if idx <= self.pos:
      return self.buf[idx % self.bufsize]
    while self.pos < idx:
      self.pos += 1
      h = md5(self.val + str(self.pos))
      for i in range(2016):
        h = md5(h)
      self.buf[self.pos % self.bufsize] = h
    return self.buf[idx % self.bufsize]
strim = md5strim('cuanljph')
pos = 0
rx = re.compile(r'(.)\1\1')
found = 0
while True:
  mat = rx.search(strim.get(pos))
  if mat:
    char = mat.group(1)
    needle = char * 5
    for i in range(1, 1001):
      if needle in strim.get(pos + i):
        found += 1
        break

  if found == 64:
    print(pos)
    break
  pos += 1
