id = 8772

def powl(id, x,y):
  return (((x+10)*y+id)*(x+10) // 100) % 10 - 5

grid = [0]*(300*300)
for y0 in range(300):
  for x0 in range(300):
    c = x0 + y0*300
    grid[c] = powl(id, x0+1,y0+1)

ms = 0
mc = (0,0,0)
sums = [0]*(300*300)
for sz in range(300):
  for y in range(300-sz):
    for x in range(300-sz):
      cd = x + y*300
      cs = x + sz + (y+sz)*300
      sums[cd] += grid[cs]
      for dx in range(sz):
        cs = x+dx+(y+sz)*300
        sums[cd] += grid[cs]
      for dy in range(sz):
        cs = x+sz+(y+dy)*300
        sums[cd] += grid[cs]
      if sums[cd] > ms:
        ms = sums[cd]
        mc = (x+1,y+1,sz+1)
  print(sz,ms,mc)

print(ms,mc)
