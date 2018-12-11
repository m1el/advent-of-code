from itertools import product
from collections import Count, defaultdict
import re

with open('06.txt') as fd:
  data = list(map(int, fd.readlines()))
  data = []
  for l in fd.readlines:
    m = list(re.findall(r'\d+', l))


