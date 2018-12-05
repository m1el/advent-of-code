rep = {'#': 1, '.': 0}
board = []
for line in open('day018.txt', 'r').readlines():
  board.append([rep[c] for c in line.strip()])

def neighbors(board, n, dim, coord):
  w, h = dim
  x, y = coord
  s = 0
  for ny in range(max(0, y-1), min(h, y+2)):
    for nx in range(max(0, x-1), min(w, x+2)):
      if nx == x and ny == y:
        continue
      if board[nx][ny] & (1<<(n % 2)):
        s += 1
  return s


def step(board, n):
  height = len(board)
  width = len(board[0])
  for y in range(height):
    for x in range(width):
      state = board[x][y] & (1<<(n%2))
      if (x == 0 and (y == 0 or y == height - 1)) or (
         x == width - 1 and (y == 0 or y == height - 1)):
        board[x][y] = state | (1<<((n+1)%2))
        continue

      neigb = neighbors(board, n, (width, height), (x, y))
      if state and neigb == 2 or neigb == 3:
        state = state | (1<<((n+1)%2))
      elif not state and neigb == 3:
        state = state | (1<<((n+1)%2))
      board[x][y] = state

for i in range(100):
  step(board, i)
s = 0
for l in board:
  for c in l:
    if c & 1:
      s += 1
print(s)
