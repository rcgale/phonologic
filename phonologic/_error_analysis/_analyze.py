import dataclasses
import json
import os
from dataclasses import dataclass
from enum import Enum
from typing import Tuple, TypeVar, Dict, Mapping, Iterable

from phonologic._error_analysis._levenshtein import ActionStep, Action


@dataclass(frozen=True)
class ErrorAnalysis:
    distance: int
    error_rate: float
    expected_length: int
    steps: Tuple[ActionStep, ...]


@dataclass(frozen=True)
class PhonologicalActionStep(ActionStep):
    action: Action
    expected: str
    actual: str
    cost: float
    deltas: Iterable["FeatureDelta"]


@dataclass(frozen=True)
class FeatureErrorAnalysis(ErrorAnalysis):
    distance: float
    error_rate: float
    expected_length: int
    steps: Tuple[PhonologicalActionStep, ...]


TAnalysis = TypeVar("TAnalysis", bound=ErrorAnalysis)


@dataclass(frozen=True)
class ErrorAnalysisDict:
    per: float
    fer: float
    items: Dict[str, TAnalysis]

    def save(self, filename):
        def json_handler(o):
            from phonologic._systems import FeatureDelta

            if isinstance(o, Enum):
                return o.name
            if isinstance(o, Mapping):
                return {key: o[key] for key in o}
            if isinstance(o, FeatureDelta):
                return {
                    "left": str(o.left if o.left is not None else ""),
                    "right": str(o.right if o.right is not None else ""),
                    "cost": o.cost
                }
            if dataclasses.is_dataclass(o):
                return dict((field.name, getattr(o, field.name)) for field in dataclasses.fields(o))
            raise NotImplementedError(type(o))

        os.makedirs(os.path.dirname(filename), exist_ok=True)
        with open(filename, "w") as f:
            json.dump(self, f, indent=4, default=json_handler)
