import hashlib

key = b'ckczppom'
for i in range(1, 2**64):
  m = hashlib.md5()
  m.update(key + str(i).encode())
  s = m.hexdigest()[0:6]
  if s[0:5] == '00000':
    print('#', i)
  if s == '000000':
    print(i)
    break
