#!/usr/bin/env python3

"""
Solution to the first part of: https://adventofcode.com/2020/day/1
"""

from itertools import combinations
from sys import argv


def main():
    fn = argv[1]
    numbers = [int(s) for s in open(fn).readlines()]
    for a, b in combinations(numbers, 2):
        if a + b == 2020:
            print(a * b)
            return


if __name__ == "__main__":
    main()