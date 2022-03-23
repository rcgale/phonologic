import phonologic.systems


class PhonologicalSystemNotFoundError(FileNotFoundError):
    def __init__(self, name):
        import phonologic._load
        super().__init__(f"Couldn't find system `{name}`. Either specify a valid filename or one of the predefined "
                         f"systems: {tuple(phonologic.systems.PREDEFINED)}")


class PhonoError(Exception):
    pass


class InvalidTokenError(PhonoError, ValueError):
    pass


class InvalidFeatureValueError(PhonoError, ValueError):
    pass


class InvalidFeatureNameError(PhonoError, ValueError):
    pass


class MustHaveDefaultError(PhonoError):
    def __init__(self):
        super().__init__(
            f"Must define <default> with the default values before all other features."
        )


class RedefinedSymbolError(PhonoError):
    def __init__(self, symbol):
        super().__init__(f"Symbol defined more than once: {symbol}")


class InvalidFeatureVectorError(PhonoError):
    def __init__(self, vector, default_vector):
        message = ""

        additional_fields = set(vector) - set(default_vector)
        if additional_fields:
            message += f" Field which were not in <default>: {tuple(additional_fields)}."

        missing_fields = set(default_vector) - set(vector)
        if missing_fields:
            # Should be unreachable if building from default.
            message += f" Missing fields: {tuple(missing_fields)}."

        super().__init__(message)

class IncompleteFeatureVectorDefinitionError(PhonoError):
    pass


class SymbolNotDefinedError(PhonoError):
    def __init__(self, symbol: "Symbol"):
        super().__init__(f"Undefined symbol {symbol}")


class FileParseError(Exception):
    def __init__(self, inner_error: Exception, line_number):
        super().__init__(f"While parsing line number {line_number} ({type(inner_error).__name__}: {inner_error})")
        self.inner_error = inner_error

