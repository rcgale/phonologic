import os

from phonologic._exceptions import PhonologicalSystemNotFoundError
from phonologic._file_parsing import parse_file
from phonologic._systems import PhonologicalFeatureSystem
from phonologic.systems import PREDEFINED


def load(name) -> PhonologicalFeatureSystem:
    if os.path.isfile(name):
        filename = name
    elif name in PREDEFINED:
        filename = PREDEFINED[name]
    else:
        raise PhonologicalSystemNotFoundError(name)
    with open(filename) as f:
        return parse_file(f)

