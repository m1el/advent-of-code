from functools import reduce
from binascii import hexlify

def knot_hash_round(inp, memory, pos, skip_size):
  lenn = len(memory)
  for size in inp:
    if pos + size <= lenn:
      chunk = memory[pos:pos+size]
      chunk = chunk[::-1]
      memory[pos:pos+size] = chunk
    else:
      chunk = memory[pos:] + memory[:pos+size-lenn]
      chunk = chunk[::-1]
      memory[pos:] = chunk[:lenn-pos]
      memory[:pos+size-lenn] = chunk[lenn-pos:]
    pos = (pos + size + skip_size) % lenn
    skip_size += 1
  return (pos, skip_size)

def knot_hash(seq):
  seq += [17, 31, 73, 47, 23]
  memory = list(range(256))
  pos = 0
  skip = 0
  for _ in range(64):
    (pos, skip) = knot_hash_round(seq, memory, pos, skip)
  outp = bytes(reduce(lambda a, b: a^b, memory[i*16:(i+1)*16]) for i in range(16))
  return outp

if __name__ == '__main__':
  inp = [187,254,0,81,169,219,1,190,19,102,255,56,46,32,2,216]
  memory = list(range(256))
  knot_hash_round(inp, memory, 0, 0)

  print(memory[0] * memory[1])
  print(hexlify(knot_hash(list(b'AoC 2017'))))
  print(hexlify(knot_hash(list(b'187,254,0,81,169,219,1,190,19,102,255,56,46,32,2,216'))))
