with open('day008.txt', 'r') as fd:
  inp = fd.readlines()

board = [[0 for _ in range(50)]
            for _ in range(6)]
for line in inp:
  words = line.split(' ')
  verb = words[0]
  args = words[1:]
  if verb == 'rect':
    [w, h] = [int(v) for v in args[0].split('x')]
    for y in range(h):
      for x in range(w):
        board[y][x] = 1
  elif verb == 'rotate':
    direction = args[0]
    [_, coord] = args[1].split('=')
    coord = int(coord)
    offset = int(args[3])
    if direction == 'row':
      row = board[coord]
      board[coord] = [row[(x - offset) % 50]
                       for x in range(50)]
    elif direction == 'column':
      col = [board[y][coord] for y in range(6)]
      for y in range(6):
        board[y][coord] = col[(y - offset) % 6]
    else:
      raise Exception('wtf')
  else:
    print('verb: ',verb)
    raise Exception('wtf')

chars=[' ', '#']
for row in board:
  print(''.join([chars[c] for c in row]))
print(sum(map(sum, board)))

# EOARGPHYAO
