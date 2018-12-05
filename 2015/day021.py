from pprint import pprint as pp
def wins(player, dargen):
  player_moves = dargen['hp'] / max(player['atk'] - dargen['def'], 1)
  dargen_moves = player['hp'] / max(dargen['atk'] - player['def'], 1)
  return player_moves <= dargen_moves

def rings(ary):
  yield []
  for i in ary:
    yield [i]
    for j in ary:
      if i != j:
        yield [i, j]

def combos(items):
  if not len(items):
    yield []
    return
  for e in items[0]:
    for a in combos(items[1:]):
      yield [e] + a

items = [
    [{'cost':  8, 'atk': 4, 'def': 0},
     {'cost': 10, 'atk': 5, 'def': 0},
     {'cost': 25, 'atk': 6, 'def': 0},
     {'cost': 40, 'atk': 7, 'def': 0},
     {'cost': 74, 'atk': 8, 'def': 0}],
    [{'cost': 13, 'atk': 0, 'def': 1},
     {'cost': 31, 'atk': 0, 'def': 2},
     {'cost': 53, 'atk': 0, 'def': 3},
     {'cost': 75, 'atk': 0, 'def': 4},
     {'cost':102, 'atk': 0, 'def': 5}],
    list(rings(
      [{'cost': 25, 'atk': 1, 'def': 0},
       {'cost': 50, 'atk': 2, 'def': 0},
       {'cost':100, 'atk': 3, 'def': 0},
       {'cost': 20, 'atk': 0, 'def': 1},
       {'cost': 40, 'atk': 0, 'def': 2},
       {'cost': 80, 'atk': 0, 'def': 3}]
      ))
    ]

def total(combo):
  player = {'cost': 0, 'atk': 0, 'def': 0}
  for item in combo:
    if type(item) == list:
      item = total(item)
    for k, v in item.items():
      player[k] += v
  return player

dargen = {
    'hp': 100,
    'atk': 8,
    'def': 2,
    }

optimal = None
for combo in combos(items):
  player = total(combo)
  player['hp'] = 100
  if not wins(player, dargen):
    continue
  if optimal is None or player['cost'] < optimal:
    print(combo)
    optimal = player['cost']
    print(optimal)

print(optimal)
