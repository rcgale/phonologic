import itertools
import math
from dataclasses import dataclass
from functools import lru_cache
from typing import Tuple, Union, Iterable, Dict, Mapping, Iterator, Set, Optional

import regex

from ._error_analysis import PhonologicalActionStep, FeatureErrorAnalysis, ErrorAnalysis, \
    edit_distance, ActionStep
from ._error_analysis._levenshtein import default_cost_del, default_cost_ins
from ._exceptions import MustHaveDefaultError, RedefinedSymbolError, InvalidFeatureVectorError, SymbolNotDefinedError, \
    IncompleteFeatureVectorDefinitionError
from ._file_parsing import SYMBOL, FEATURE_VALUES
from ._file_parsing._spec import IGNORE_SYMBOLS, DEFAULT_SYMBOL, ZERO_COST_SYMBOLS


class FeatureValue(float):
    def __str__(self):
        return next(s for s, v in FEATURE_VALUES.items() if v == self)

    def __repr__(self):
        return f"{type(self).__name__}({str(self)})"

    def __eq__(self, other):
        if math.isnan(self) and math.isnan(other):
            return True
        return float(self) == float(other)

    def __ne__(self, other):
        return not self == other

    __hash__ = float.__hash__


@dataclass(frozen=True)
class FeatureDelta:
    left: "Feature"
    right: "Feature"
    cost: float

    def __iter__(self):
        return iter((self.left, self.right))

    def __post_init__(self):
        assert self.left is None or self.right is None or self.left.name == self.right.name


@dataclass(unsafe_hash=True)
class Feature(float):
    value: FeatureValue
    name: str

    def __new__(cls, value, *args, **kwargs):
        return float.__new__(cls, value)

    def __post_init__(self):
        assert isinstance(self.value, FeatureValue)

    def __str__(self):
        return f"{self.value}{self.name}"

    def __repr__(self):
        return f"{type(self).__name__}({self})"


class FeatureCollection(Mapping[str, Feature]):
    def __init__(self, items: Union[Iterable[Feature], Mapping[str, Feature]] = None):
        self._feature_dict, self.__hash = self._init_feature_dict(items)
        for key, feature in self._feature_dict.items():
            setattr(self, key, feature)

    def _init_feature_dict(self, items):
        if not items:
            return {}, None
        if isinstance(items, (Dict)):
            return items, None
        if isinstance(items, (FeatureCollection)):
            return dict(items), hash(items)
        elif isinstance(items, Iterable):
            d = {}
            for feature in items:
                if isinstance(feature, Feature):
                    d[feature.name] = feature
                else:
                    raise TypeError(type(feature))
            return d, None
        else:
            raise TypeError(type(items))

    def update(self, other: "FeatureCollection"):
        items = dict(self)
        specified = {
            key: f
            for key, f in other._feature_dict.items()
            if f.value != FEATURE_VALUES["?"]
        }
        items.update(specified)
        return FeatureCollection(items)

    def __getitem__(self, key: str) -> FeatureValue:
        return self._feature_dict[key]

    def __len__(self) -> int:
        return len(self._feature_dict)

    def __iter__(self) -> Iterator[str]:
        return iter(self._feature_dict)

    def __eq__(self, other):
        for key in set(self) | set(other):
            undefined = Feature(FeatureValue(0), key)
            if self.get(key, undefined) != other.get(key, undefined):
                return False
        return True

    def __str__(self):
        features = ", ".join([
            str(self[f])
            for f in self
        ])
        return f"[{features}]"

    def __repr__(self):
        return f"{type(self).__name__}({self})"

    def __hash__(self):
        if self.__hash is None:
            self.__hash = hash(tuple((key, float(self[key])) for key in self))
        return self.__hash

    def delta(self, other) -> Tuple[FeatureDelta, ...]:
        return tuple((
            FeatureDelta(self[key], other[key], feature_cost(self[key].value, other[key].value))
            for key in self
            if self[key] != other[key]
        ))


class FeatureVector(FeatureCollection):
    def __init__(self, items: Tuple[Feature, ...], default: FeatureCollection, is_class: bool):
        super().__init__(items)
        if tuple(self) != tuple(default):
            raise InvalidFeatureVectorError(self, default)
        if not is_class and any(self[v].value == FeatureValue(float("NaN")) for v in self):
            raise IncompleteFeatureVectorDefinitionError(str(self))


class Symbol(str):
    def __init__(self, s):
        assert regex.match(SYMBOL, s)


class DefinitionItems(Tuple[Union[FeatureCollection, Symbol], ...]):
    def __init__(self, items=tuple()):
        if not len(self):
            raise ValueError("definition may not be empty")

        for item in self:
            if not isinstance(item, (FeatureCollection, Symbol)):
                raise ValueError(f"Unacceptable type: {type(item).__name__}")

    def __str__(self):
        items = " ".join(str(item) for item in self)
        return f"{items}"

    def __repr__(self):
        return f"{type(self).__name__}({str(self)})"


@dataclass(frozen=True)
class Definition:
    symbol: Symbol
    definition: DefinitionItems

    def __post_init__(self):
        if not isinstance(self.symbol, Symbol):
            raise ValueError(f"Invalid symbol: {self.symbol}")
        if not isinstance(self.definition, DefinitionItems):
            raise ValueError(f"Invalid definition: {self.definition}")


@dataclass(frozen=True)
class PhonologicalFeatureEntry(Mapping[str, FeatureValue]):
    symbol: Symbol
    definition: DefinitionItems
    features: FeatureVector

    def __post_init__(self):
        if not isinstance(self.features, FeatureVector):
            raise ValueError(f"Invalid features: {self.features}")

    def __str__(self):
        return f"{self.symbol} = {self.definition}"

    def __repr__(self):
        return f"{type(self).__name__}({str(self)})"

    def __getitem__(self, feature_name: str) -> FeatureValue:
        return self.features[feature_name]

    def __len__(self) -> int:
        return len(self.features)

    def __iter__(self) -> Iterator[str]:
        return iter(self.features)


def compute_features(
        symbol_map: Dict[str, PhonologicalFeatureEntry],
        definition: Definition,
) -> FeatureVector:
    if definition.symbol == DEFAULT_SYMBOL:
        if len(definition.definition) != 1 or not isinstance(definition.definition[0], FeatureCollection):
            raise MustHaveDefaultError()
        features = FeatureCollection(definition.definition[0])
        return FeatureVector(features, features, is_class=True)

    default_entry = symbol_map.get(DEFAULT_SYMBOL)
    if default_entry is None:
        raise MustHaveDefaultError()
    default_features = symbol_map.get(DEFAULT_SYMBOL)
    features = default_features.features

    for item in definition.definition:
        if isinstance(item, Symbol):
            if item not in symbol_map:
                raise SymbolNotDefinedError(item)
            features = features.update(symbol_map[item].features)
        elif isinstance(item, FeatureCollection):
            if len(symbol_map):
                features = features.update(item)
            if definition.symbol == DEFAULT_SYMBOL:
                features = item
        else:
            raise NotImplemented(type(item).__name__)

    is_class = definition.symbol[0] == "<" and definition.symbol[-1] == ">"
    return FeatureVector(features, default_features, is_class=is_class)


def build_system(definitions: Iterable[Definition], ignore_symbols=IGNORE_SYMBOLS):
    symbol_map = {}
    feature_map = {}
    for definition in definitions:
        if definition.symbol in symbol_map:
            raise RedefinedSymbolError(definition.symbol)

        features = compute_features(symbol_map, definition)
        entry = PhonologicalFeatureEntry(definition.symbol, definition.definition, features)
        symbol_map[definition.symbol] = entry
        if not symbol_is_class(entry.symbol):
            feature_map[None] = feature_map.get(None, set())
            feature_map[None].add(entry)
            for feature_value in features.values():
                feature_map[feature_value] = feature_map.get(feature_value, set())
                feature_map[feature_value].add(entry)
    return PhonologicalFeatureSystem(symbol_map=symbol_map, feature_map=feature_map, ignore_symbols=ignore_symbols)


class PhonologicalFeatureSystem:

    def __init__(
            self,
            symbol_map: Dict[str, PhonologicalFeatureEntry],
            feature_map: Dict[FeatureValue, PhonologicalFeatureEntry],
            ignore_symbols: Iterable[str] = IGNORE_SYMBOLS,
            zero_cost_symbols: Iterable[str] = ZERO_COST_SYMBOLS,
    ):
        self._symbol_map = symbol_map
        self._feature_map = feature_map
        self._ignore_symbols = ignore_symbols
        self._zero_cost_symbols = zero_cost_symbols
        if not len(self._symbol_map):
            raise ValueError(f"No definitions found.")

    def __getitem__(self, item: Union[str, Symbol]):
        if item in self._symbol_map:
            return self._symbol_map[item]
        else:
            raise KeyError(item)

    def query(
            self,
            with_features: Union[str, FeatureCollection, Iterable[FeatureValue]]
    ) -> Set[PhonologicalFeatureEntry]:
        if not isinstance(with_features, (str, FeatureCollection, Iterable[FeatureValue])):
            raise TypeError(with_features)
        if isinstance(with_features, str):
            from ._file_parsing import parse_features
            with_features = parse_features(with_features)
        if isinstance(with_features, FeatureCollection):
            with_features = with_features.values()
        found = set(self._feature_map[None])
        for feature in with_features:
            feature_set = self._feature_map.get(feature, set())
            found = found.intersection(feature_set)
        return found

    @property
    def n_features(self):
        return len(self.default)

    @property
    def features(self):
        return tuple(self.default.features)

    @property
    def entries(self) -> Mapping[Symbol, PhonologicalFeatureEntry]:
        return dict(self._symbol_map)

    @property
    def phoneme_entries(self) -> Mapping[Symbol, PhonologicalFeatureEntry]:
        return {
            key: value
            for key, value in self._symbol_map.items()
            if not symbol_is_class(key)
        }

    @property
    def default(self):
        return self[DEFAULT_SYMBOL]

    def feature_edit_distance(self, expected: str, actual: str):
        return edit_distance(
            self.tokenize(expected),
            self.tokenize(actual),
            cost_sub=self._feature_cost_sub,
            cost_ins=self._feature_cost_ins,
            cost_del=self._feature_cost_del,
        )

    def phoneme_edit_distance(self, expected: str, actual: str):
        return edit_distance(
            self.tokenize(expected),
            self.tokenize(actual),
            cost_sub=self._phoneme_cost_sub,
            cost_ins=self._phoneme_cost_ins,
            cost_del=self._phoneme_cost_del,
        )

    def analyze_phoneme_errors(self, expected: str, actual: str) -> ErrorAnalysis:
        distance = self.phoneme_edit_distance(expected, actual)
        expected_length = sum(t not in self._zero_cost_symbols for t in self.tokenize(expected))
        steps = distance.trace()
        error_rate = 0 if expected_length == 0 else distance / expected_length
        return ErrorAnalysis(int(distance), error_rate, expected_length=expected_length, steps=steps)

    def analyze_feature_errors(
            self,
            expected: str,
            actual: str,
    ) -> FeatureErrorAnalysis:
        distance = self.feature_edit_distance(expected, actual)
        expected_length = self.n_features * sum(t not in self._zero_cost_symbols for t in self.tokenize(expected))
        action_steps = distance.trace()
        deltas = tuple(self._get_step_feature_delta(step) for step in action_steps)
        error_rate = 0 if expected_length == 0 else distance / expected_length
        return FeatureErrorAnalysis(float(distance), error_rate, expected_length=expected_length, steps=deltas)

    @property
    @lru_cache()
    def _tokenizer(self) -> "PhonemeTokenizer":
        symbols = tuple((*self._symbol_map, *self._zero_cost_symbols))
        return PhonemeTokenizer.build(symbols, self._ignore_symbols)

    def tokenize(self, s):
        return self._tokenizer(s)

    def _phoneme_cost_sub(self, expected: str, actual: str) -> int:
        if {expected, actual}.intersection((None, *self._zero_cost_symbols)):
            return 0 if expected == actual else float("inf")
        return int(expected != actual)

    def _phoneme_cost_del(self, expected: str) -> int:
        if expected in self._zero_cost_symbols:
            return 0
        return default_cost_del(expected)

    def _phoneme_cost_ins(self, actual: str) -> int:
        if actual in self._zero_cost_symbols:
            return 0
        return default_cost_ins(actual)

    @lru_cache(maxsize=None)
    def _feature_cost_sub(self, expected: str, actual: str) -> float:
        if {expected, actual}.intersection((None, *self._zero_cost_symbols)):
            return 0 if expected == actual else float("inf")

        expected_iter = (self[expected][key] for key in self[expected])
        actual_iter = (self[actual][key] for key in self[actual])

        pairs = list(zip(expected_iter, actual_iter))
        if len(list(expected_iter)) or len(list(actual_iter)):
            raise ValueError(f"Feature vectors must be the same length for feature distance calculation.")
        if not len(expected):
            raise ValueError(f"No feature vectors to compute cost.")

        return sum(feature_cost(x, y) for x, y in pairs)

    @lru_cache(maxsize=None)
    def _feature_cost_ins(self, actual: str) -> float:
        if actual is None:
            return float("inf")
        if actual in self._zero_cost_symbols:
            return 0
        return sum(feature_cost(None, self[actual][key]) for key in self[actual])

    @lru_cache(maxsize=None)
    def _feature_cost_del(self, expected: str) -> float:
        if expected is None:
            return float("inf")
        if expected in self._zero_cost_symbols:
            return 0
        return sum(feature_cost(self[expected][key], None) for key in self[expected])

    def _get_step_feature_delta(self, step: ActionStep):
        undefined_features = {None, *self._zero_cost_symbols}
        if step.expected not in undefined_features and step.actual not in undefined_features:
            feature_delta = self[step.expected].features.delta(self[step.actual].features)
        elif step.expected not in undefined_features:
            feature_delta = tuple((
                FeatureDelta(f, None, feature_cost(f.value, None))
                for f in self[step.expected].features.values()
            ))
        elif step.actual not in undefined_features:
            feature_delta = tuple((
                FeatureDelta(None, f, feature_cost(None, f.value))
                for f in self[step.actual].features.values()
            ))
        else:
            feature_delta = FeatureCollection()

        return PhonologicalActionStep(
            expected=step.expected,
            actual=step.actual,
            cost=step.cost,
            action=step.action,
            deltas=feature_delta
        )


class PhonemeTokenizer:
    def __init__(self, mapping: Mapping[int, Set[str]], ignore: Set[str]):
        self._mapping = mapping
        self._sorted_keys = tuple(reversed(sorted(self._mapping)))
        self._ignore = set(ignore)

    def __call__(self, s: str):
        if not isinstance(s, str):
            raise TypeError(f"Expected string, got {repr(s)}")

        tokens = []
        begin = 0
        while begin < len(s):
            found = False
            candidates = list(
                (l, self._mapping[l])
                for l in self._sorted_keys
                if l <= len(s) - begin
            )
            for length, token_set in candidates:
                substring = s[begin:begin + length]
                if substring in token_set:
                    if substring not in self._ignore:
                        tokens.append(substring)
                    found = True
                    begin += length
                    break
            if not found:
                raise ValueError(f"Unrecognized token in '{s}', near '{s[begin:begin + self._sorted_keys[0]]}'")
        return tuple(tokens)

    @classmethod
    def build(cls, symbols: Iterable[str], ignore: Iterable[str]):
        mapping = {}
        for symbol in itertools.chain(symbols, ignore):
            # if symbol_is_class(symbol) and symbol not in ignore:
            #     continue
            length = len(symbol)
            mapping[length] = mapping.get(length, set())
            mapping[length].add(symbol)
        return PhonemeTokenizer(mapping, ignore)


def symbol_is_class(s):
    return len(s) and s[0] == "<" and s[-1] == ">"


def feature_cost(left: Optional[FeatureValue], right: Optional[FeatureValue]):
    if left is not None and right is not None:
        if left == right:
            return 0
        return abs(left - right) / 2
    elif left is not None:
        return 0.5 if float(left) == 0 else 1.0
    elif right is not None:
        return 0.5 if float(right) == 0 else 1.0
    else:
        raise RuntimeError()
