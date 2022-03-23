from typing import Iterable, Union, Optional

import regex

from phonologic._systems import Definition, Symbol, Feature, FeatureCollection, DefinitionItems, \
    PhonologicalFeatureSystem, FeatureValue, build_system
from phonologic._exceptions import FileParseError, PhonoError
from phonologic._file_parsing._spec import SYMBOL, FEATURE_VALUES
from phonologic._file_parsing._tokenizer import tokenize, tokenize_feature_set


def parse_file(f: Union[str, Iterable[str]]) -> "PhonologicalFeatureSystem":
    last_line_number = [None]

    def line_number_iterator(lines):
        for line_number, line in enumerate(lines, start=1):
            last_line_number[-1] = line_number
            parsed = parse_statement(line)
            if parsed is not None:
                yield parsed
    try:
        return build_system(line_number_iterator(f))
    except PhonoError as e:
        if last_line_number[-1] is None:
            raise
        raise FileParseError(e, last_line_number[-1]) from e


def parse_statement(s) -> Optional[Definition]:
    definition = tuple()
    tokens = tokenize(s)
    if not len(tokens):
        return None
    if len(tokens) < 3 or tokens[1] != "=":
        raise SyntaxError(s)
    symbol = Symbol(tokens[0])
    for token in tokens[2:]:
        if not len(token.strip()):
            continue
        elif regex.match(SYMBOL, token):
            definition = (*definition, Symbol(token))
        elif token[0] == "[" and token[-1] == "]":
            features = parse_features(token)
            definition = (*definition, features)
        else:
            raise SyntaxError(f"Invalid token: {token}")
    return Definition(symbol, DefinitionItems(definition))


def parse_features(feature_set_string):
    features = tuple(
        Feature(FeatureValue(FEATURE_VALUES[value]), name)
        for value, name in tokenize_feature_set(feature_set_string)
    )
    return FeatureCollection(features)
