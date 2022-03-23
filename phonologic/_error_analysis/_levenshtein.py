import itertools
from dataclasses import dataclass
from enum import Enum
from numbers import Number
from typing import Callable, Iterable, Tuple


class Action(Enum):
    EQ = "equal"
    SUB = "substitution"
    INS = "insertion"
    DEL = "deletion"

    def __repr__(self):
        return self.name


@dataclass(frozen=True)
class ActionStep:
    action: Action
    expected: str
    actual: str
    cost: float


@dataclass(frozen=True)
class LevenshteinCost(float):
    action: Action = None
    expected: str = None
    actual: str = None
    cost: float = None
    previous: "LevenshteinCost" = None

    def __new__(cls, *args, cost, **kwargs):
        return float.__new__(cls, cost)

    def __add__(self, other):
        value = float(self) + other
        if isinstance(other, (LevenshteinCost,)):
            return LevenshteinCost(other.action, other.expected, other.actual, cost=value, previous=self)
        return value

    def __eq__(self, other):
        return float(self) == float(other)

    def __ne__(self, other):
        return float(self) != float(other)

    def __lt__(self, other):
        if float(self) == float(other):
            return self.action == Action.EQ  # Prefer an EQ if all else is equal
        return float(self) < float(other)

    def __gt__(self, other):
        if float(self) == float(other):
            return getattr(other, "action", None) == Action.EQ    # Prefer an EQ if all else is equal
        return float(self) > float(other)

    def trace(self) -> Tuple[ActionStep, ...]:
        cursor = self
        history = []
        while cursor.previous is not None:
            if cursor.expected is None and cursor.actual is None:
                raise RuntimeError("Problem with the backtrace!")
            if cursor.action is not None:
                cost = cursor.cost
                if cursor.previous is not None:
                    cost -= cursor.previous.cost
                history.insert(0, ActionStep(cursor.action, cursor.expected, cursor.actual, cost))
            cursor = cursor.previous
        return tuple(history)


def default_cost_sub(expected: str, actual: str) -> int:
    if expected is None or actual is None:
        return float("inf")
    return int(expected != actual)


def default_cost_del(expected: str) -> int:
    if expected is None:
        return float("inf")
    return 1


def default_cost_ins(actual: str) -> int:
    if actual is None:
        return float("inf")
    return 1


def trace_sub(cost_sub: Callable[[str, str], Number], expected: str, actual: str) -> LevenshteinCost:
    cost = cost_sub(expected, actual)
    action = Action.EQ if cost == 0.0 else Action.SUB
    return LevenshteinCost(action, expected, actual, cost=cost)


def trace_ins(cost_ins: Callable[[str, str], Number], actual: str) -> LevenshteinCost:
    cost = cost_ins(actual)
    return LevenshteinCost(Action.INS, None, actual, cost=cost)


def trace_del(cost_del: Callable[[str, str], Number], expected: str) -> LevenshteinCost:
    cost = cost_del(expected)
    return LevenshteinCost(Action.DEL, expected, None, cost=cost)


def edit_distance(
        expected: Iterable[str],
        actual: Iterable[str],
        *,
        cost_sub: Callable[[str, str], Number] = default_cost_sub,
        cost_del: Callable[[str], Number] = default_cost_del,
        cost_ins: Callable[[str], Number] = default_cost_ins,
) -> LevenshteinCost:
    if expected is None or type(expected) != type(actual):
        raise TypeError()

    table = {}

    i = 0
    j = 0

    expected_enumerate = enumerate(itertools.chain([None], expected))
    actual_enumerate = enumerate(itertools.chain([None], actual))
    for (i, token_expected), (j, token_actual) in itertools.product(expected_enumerate, actual_enumerate):
        if i == 0 and j == 0:
            table[i, j] = LevenshteinCost(cost=0)
            continue

        sub_prev = table[i - 1, j - 1] if i > 0 and j > 0 else LevenshteinCost(cost=0)
        ins_prev = table[i, j - 1] if j > 0 else LevenshteinCost(cost=0)
        del_prev = table[i - 1, j] if i > 0 else LevenshteinCost(cost=0)

        table[i, j] = min(
            del_prev + trace_del(cost_del, token_expected),
            ins_prev + trace_ins(cost_ins, token_actual),
            sub_prev + trace_sub(cost_sub, token_expected, token_actual),
        )

    return table[i, j]
