import numpy as np
from numpy.linalg import matrix_power

alpha = 'abcdefghijklmnop'
alpha_r = {c: i for i, c in enumerate(alpha)}
size = len(alpha)

def spin(n):
  rv = np.identity(size, dtype=int)
  return np.concatenate((rv[n:], rv[:n]), axis=0)

def exchange(a, b):
  rv = np.identity(size, dtype=int)
  rv[a,a], rv[b,b] = (0, 0)
  rv[a,b], rv[b,a] = (1, 1)
  return rv

def pperm(perm):
  vec = np.array(range(size), dtype=int).dot(perm)
  print(''.join(alpha[i] for i in vec))

with open('d16.txt') as fd:
  commands = fd.read().strip().split(',')

left = np.identity(size, dtype=int)
right = np.identity(size, dtype=int)

start = np.identity(size, dtype=int)
for cmd in commands:
  if cmd[0] == 's':
    num = int(cmd[1:])
    right = right.dot(spin(num))
  if cmd[0] == 'x':
    p1, p2 = map(int, cmd[1:].split('/'))
    right = right.dot(exchange(p1, p2))
  if cmd[0] == 'p':
    p1, p2 = cmd[1:].split('/')
    p1, p2 = alpha_r[p1], alpha_r[p2]
    left = exchange(p1, p2).dot(left)

pperm(left.dot(start).dot(right))

BILLION = 1000000000
left = matrix_power(left, BILLION)
right = matrix_power(right, BILLION)
pperm(left.dot(start).dot(right))
