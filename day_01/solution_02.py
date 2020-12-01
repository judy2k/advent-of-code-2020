#!/usr/bin/env python3

"""
Solution to the second part of: https://adventofcode.com/2020/day/1
"""

from itertools import combinations
from sys import argv


def main():
    fn = argv[1]
    numbers = [int(s) for s in open(fn).readlines()]
    for a, b, c in combinations(numbers, 3):
        if a + b + c == 2020:
            print(a * b * c)
            return


if __name__ == "__main__":
    main()