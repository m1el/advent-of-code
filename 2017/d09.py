with open('09.txt') as fd:
  data = fd.readline().strip()

pos = 0
current = []
stack = []
in_garbage = False
garbage = 0
while pos < len(data):
  c = data[pos]
  pos += 1
  if in_garbage:
    if c == '!': pos += 1
    elif c == '>': in_garbage = False
    else: garbage += 1
  else:
    if c == '{':
      child = []
      current.append(child)
      stack.append(current)
      current = child
    elif c == '<': in_garbage = True
    elif c == '}':
      assert len(stack) > 0, 'unbalanced parens, too many closing'
      current = stack.pop()
    elif c == ',': pass
    else:
      raise Exception('unknown char ' + c)

assert len(stack) == 0, 'unbalanced parens, too few closing'

def score(node, depth=0):
  return depth + sum(score(n, depth + 1) for n in node)

print(current)
print(score(current))
print(garbage)
