'''
The first floor contains a thulium generator, a thulium-compatible microchip, a plutonium generator, and a strontium generator.
The second floor contains a plutonium-compatible microchip and a strontium-compatible microchip.
The third floor contains a promethium generator, a promethium-compatible microchip, a ruthenium generator, and a ruthenium-compatible microchip.
The fourth floor contains nothing relevant.

TG, TM, PG, SG
PM, SM
OG, OM, RG, RM

T, P, S, O, R
0  0  0  2  2
0  1  1  2  2
'''
class state(object):
  E = 0
  objs = [
    [0, 0],
    [0, 1],
    [0, 1],
    [2, 2],
    [2, 2],
  ]
  def score()
  def is_good(self, objs):
    gfloors = [[] for _ in range(4)]
    mfloors = [[] for _ in range(4)]
    for i, o in enumerate(objs):
      gfloors[o[0]].append(i)
      mfloors[o[1]].append(i)
    for gs, ms in zip(gfloors, mfloors):
      if not len(gs): break
      for m in ms:
        if m not in gs:
          return False
    return True
