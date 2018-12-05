import re
import functools

num_re = re.compile(r'^\d+$')
not_re = re.compile(r'^NOT (?P<src>\S+)$')
bin_re = re.compile(r'^(?P<src0>\S+) (?P<op>AND|OR|LSHIFT|RSHIFT) (?P<src1>\S+)$')
ops = {
    'AND': '&',
    'OR': '|',
    'LSHIFT': '<<',
    'RSHIFT': '>>',
    }

def subst_bin(match):
  g = match.groupdict()
  return '({} {} {}) & 0xFFFF'.format(g['src0'], ops[g['op']], g['src1'])

text = open('day007.txt', 'r').read().splitlines()
glob = globals()
memoize = functools.lru_cache
for l in text:
  [src, dst] = l.split(' -> ')
  src = re.sub(not_re, r'\1 ^ 0xFFFF', src)
  src = re.sub(bin_re, subst_bin, src)
  src = re.sub(r'(\b[a-z]+\b)', r'_\1()', src)
  glob['_' + dst] = memoize()(eval('lambda:' + src))

print(_a())
