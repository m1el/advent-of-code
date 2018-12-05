from functools import reduce
from itertools import combinations

def main():
    weights = []
    fin = open('day024.txt', 'r')
    for line in fin.readlines():
        weight = int(line.strip())
        weights.append(weight)

    part = sum(weights)
    options = []
    ans1, ans2 = 1e20, 1e20

    # This code is unsound, but I got lucky.
    for i in range(1, 8):
        for c in combinations(weights, i):
            if sum(c) == part / 3:
                ans1 = min(ans1, reduce(lambda a, b: a * b, list(c)))
            elif sum(c) == part / 4:
                ans2 = min(ans2, reduce(lambda a, b: a * b, list(c)))

    print("Part 1", ans1)
    print("Part 2", ans2)

if __name__ == '__main__':
    main()
