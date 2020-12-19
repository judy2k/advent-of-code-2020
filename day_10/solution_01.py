#!/usr/bin/env python3

from collections import Counter
from itertools import *
import sys


def adapter_chain(inputs):
    if inputs:
        yield 0
        for i in sorted(inputs):
            yield i
        yield i + 3


def solve_01(inputs):
    s = list(adapter_chain(inputs))
    diff_counts = Counter(b - a for a, b in zip(s, islice(s, 1, None)))
    return diff_counts[1] * diff_counts[3]


def solve_02(inputs):
    probs = {}
    for i in reversed(list(adapter_chain(inputs))):
        probs[i] = (
            sum(probs[d] for d in [d for d in range(i + 1, i + 4) if d in probs]) or 1
        )
    return probs[0]


if __name__ == "__main__":
    print(solve_01([int(s) for s in open(sys.argv[1])]))
    print(solve_02([int(s) for s in open(sys.argv[1])]))