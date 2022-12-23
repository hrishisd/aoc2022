from __future__ import annotations
from dataclasses import dataclass, Counter
from operator import add, mul
from collections import deque
from typing import Callable, Optional


def prime_factors(n):
    p, i = 2, 1
    while p * p <= n:
        while n % p == 0:
            yield p
            n //= p
        p, i = p + i, 2
    if n > 1:
        yield n


class Worry:
    factors: Counter
    value: Optional[int]

    def __init__(self, n):
        self.factors = Counter(prime_factors(n))

    def mulitply(self, i):
        if self.value:
            pass
            # self.value =
        self.factors.add(i)

    def divide(self, i):
        pass

    def divisible_by(self, i):
        return i in self.factors


@dataclass
class Monkey:
    items: deque[int | Worry]
    worry_update: Callable[[int], int]
    next_monkey_idx: Callable[[int], int]


with open("../input") as f:

    def part1_inspection_worry_update(worry_update, worry):
        return worry_update(worry) // 3

    def part2_inspection_worry_update(worry_update, worry):
        return worry_update(worry)

    def parse_monkey(s: str) -> Monkey:
        _, items, op, test, true_case, false_case = [
            line.strip() for line in s.splitlines()
        ]
        items = deque(
            int(item) for item in items.removeprefix("Starting items: ").split(", ")
        )
        op = op.removeprefix("Operation: new = ")
        l, op, r = op.split()
        op = add if op == "+" else mul
        worry_update = (
            lambda old: op(old, old) if (l, r) == ("old", "old") else op(old, int(r))
        )
        divisor = int(test.removeprefix("Test: divisible by "))
        true_case = int(true_case.removeprefix("If true: throw to monkey "))
        false_case = int(false_case.removeprefix("If false: throw to monkey "))
        next_monkey_idx = (
            lambda worry: true_case if worry % divisor == 0 else false_case
        )
        return Monkey(items, worry_update, next_monkey_idx)

    def simulate(
        monkeys: list[Monkey],
        worry_update: Callable[[Callable[[int], int], int], int],
        num_rounds: int,
    ):
        inspections = [0 for _ in monkeys]
        for r in range(num_rounds):
            print(r)
            for idx, monkey in enumerate(monkeys):
                inspections[idx] += len(monkey.items)
                while monkey.items:
                    worry = monkey.items.popleft()
                    worry_after_inspection = worry_update(monkey.worry_update, worry)
                    worry_during_inspection = monkey.worry_update(worry)
                    worry_after_inspection = worry_during_inspection // 3
                    dest_idx = monkey.next_monkey_idx(worry_after_inspection)
                    monkeys[dest_idx].items.append(worry_after_inspection)
        return mul(*sorted(inspections, reverse=True)[:2])

    input = f.read()
    monkeys = [parse_monkey(s) for s in input.split("\n\n")]
    print(f"part 1: {simulate(monkeys, part1_inspection_worry_update, 20)}")
    print(f"part 1: {simulate(monkeys, part2_inspection_worry_update, 10000)}")
