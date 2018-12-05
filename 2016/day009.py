import re
with open('day009.txt', 'r') as fd:
  inp = fd.read().strip()

rx = re.compile(r'\((?P<chars>\d+)x(?P<times>\d+)\)')
def nextcmd(s, pos=0):
  mat = rx.search(s, pos)
  if not mat:
    return None
  data = mat.groupdict()
  chars = int(data['chars'])
  times = int(data['times'])
  start = mat.start()
  end = mat.end()
  return (chars, times, start, end)

def expand(s, recurse=False):
  pos = 0
  out = 0
  while True:
    cmd = nextcmd(s, pos)
    if not cmd: break
    chars, times, start, end = cmd
    out += start - pos
    if recurse:
      sub = s[end:end+chars]
      subs = expand(sub, recurse=recurse)
      out += subs * times
    else:
      out += chars * times
    pos = end + chars

  out += len(s) - pos
  return out

print('###')
print(expand(inp))
print(expand(inp, recurse=True))
