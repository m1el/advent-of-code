import json

def jsum(obj):
  if type(obj) is int:
    return obj
  elif type(obj) is list:
    return sum(map(jsum, obj))
  elif type(obj) is dict:
    if "red" in obj.values():
      return 0
    return sum(map(jsum, obj.values()))
  return 0

with open('day012.txt') as f:
  print(jsum(json.load(f)))
