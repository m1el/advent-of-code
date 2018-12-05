from itertools import permutations

test = [
'swap position 4 with position 0',
'swap letter d with letter b',
'reverse positions 0 through 4',
'rotate left 1 step',
'move position 1 to position 4',
'move position 3 to position 0',
'rotate based on position of letter b',
'rotate based on position of letter d',
]
with open('day021.txt') as fd:
  lines = [line.strip().split(' ') for line in fd.readlines() if line]

def invdict(d):
  return {v: k for k, v in d.items()}
def invary(a):
  return [a.indexof(i) for i in range(len(a))]


def permute(letters):
  letternum = {c: i for i, c in enumerate(letters)}
  letterpos = [i for i in range(len(letters))]
  swappos = [i for i in range(len(letters))]
  for line in lines:
    if line[0] == 'swap':
      src = line[2]
      dst = line[5]
      if line[1] == 'position':
        src, dst = (int(src), int(dst))
        ary = swappos
      elif line[1] == 'letter':
        src = letterpos.index(letternum[src])
        dst = letterpos.index(letternum[dst])
        ary = letterpos
      else:
        print(line[1])
        die()
      ary[src], ary[dst] = ary[dst], ary[src]
    elif line[0] == 'rotate':
      if line[1] == 'left':
        shift = -int(line[2])
      elif line[1] == 'right':
        shift = int(line[2])
      elif line[1] == 'based':
        letter = letternum[line[6]]
        letter = letterpos.index(letter)
        idx = swappos.index(letter)
        shift = 1 + idx
        if idx >= 4: shift+=1
      else:
        print(line[1])
        die()
      shift = shift % len(letters)
      if shift != 0:
        swappos = swappos[-shift:] + swappos[:-shift]
    elif line[0] == 'move':
      src = int(line[2])
      dst = int(line[5])
      ary = swappos
      val = ary[src]
      ary = ary[:src] + ary[src+1:]
      ary = ary[:dst] + [val] + ary[dst:]
      swappos = ary
    elif line[0] == 'reverse':
      src = int(line[2])
      dst = int(line[4])
      swappos[src:dst+1] = list(reversed(swappos[src:dst+1]))
    else:
      print(line[0])
      die()
  poss = (letterpos[i] for i in swappos)
  chars = (letters[i] for i in poss)
  return ''.join(chars)

print(permute('abcdefgh'))
for p in permutations('abcdefgh'):
  if permute(''.join(p)) == 'fbgdceah':
    print(''.join(p))
