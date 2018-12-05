def factors(n):
  for i in range(1, n+1):
    sq = i*i
    if sq >= n:
      if sq == n:
        yield i
      break
    if n % i == 0:
      yield i
      yield n // i

def house(n):
  return sum(factors(n)) * 10

def factors2(n):
  for i in range(1, min(50, n)+1):
    if n % i == 0:
      yield n // i

def house2(n):
  return sum(factors2(n)) * 11

for i in range(1000, 2**64):
  s = house2(i)
  if i % 10000 == 0:
    print(i, s)
  if s >= 36000000:
    print(i, s)
    break
