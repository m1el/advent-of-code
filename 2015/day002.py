text = open('day002.txt', 'r').read().splitlines()
area = 0
length = 0
for l in text:
  dims = l.split('x')
  if len(dims) != 3: continue
  (w, h, l) = map(int, dims)
  (a, b, c) = (l * w, w * h, h * l)
  area += 2*(a + b + c) + min(a, b, c)
  length += 2 * (w + h + l - max(w, h, l)) + (w * h * l)
print('wrapping paper area: ', area)
print('ribbon length: ', length)
