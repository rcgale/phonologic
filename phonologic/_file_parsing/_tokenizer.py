import regex

from phonologic._exceptions import InvalidTokenError, InvalidFeatureValueError, InvalidFeatureNameError
from phonologic._file_parsing._spec import TOKENIZER, FEATURE_SET_TOKENIZER


def tokenize(statement):
    tokens = []
    for token, comment, separator, illegal in regex.findall(TOKENIZER, statement):
        if len(separator) or len(comment):
            continue
        if len(illegal):
            raise InvalidTokenError(f"Invalid token: {illegal}")
        if not len(token):
            raise ValueError(f"The tokenizer regular expression returned an empty token.")
        tokens.append(token)
    return tuple(tokens)


def tokenize_feature_set(feature_set_string):
    feature_set_string = feature_set_string.strip("[ ]")
    if not feature_set_string:
        return tuple()
    tokens = []
    # for value, feature_name in regex.split(FEATURE_TOKEN_SEP, feature_set_string):
    for value, feature, separator, bracket, illegal in regex.findall(FEATURE_SET_TOKENIZER, feature_set_string):
        if len(separator):
            continue
        if len(illegal):
            raise InvalidTokenError(illegal)
        if not len(value):
            raise InvalidFeatureValueError(feature_set_string)
        if not len(feature):
            raise InvalidFeatureNameError(feature_set_string)
        tokens.append((value, feature))
    return tuple(tokens)