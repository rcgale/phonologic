# Regular expressions:
#
# I didn't take lightly the decision to use such substantial regular expressions for parsing, since they can be hard
# to debug. However, they are also highly portable, and can be reasonably maintained with sufficient documentation and
# testing.

# \[[^\]]*\]                # entire segment between two brackets represents, namely for a feature specification. This
                            # is overly inclusive, but a more constrained definition is used for subparsing.

# =                         # Assignment operator

# <[\p{L}\p{M}_]+>          # A symbol name with angle brackets around it, e.g. <velar>. The symbol name may be any
                            # Unicode character belonging to the Letter or Mark categories, and underscores

# [\p{L}\p{M}]+             # A symbol name not enclosed in angle brackets.

# (\s*#.*$)                 # Comments, i.e. the `#` symbol through the end of a line

#

# Note the four groupings: `for token, comment, separator, illegal in regex.findall(TOKENIZER, statement)): # ...`
TOKENIZER = r"(\[[^\]]*\]|=|<[\p{L}\p{M}_\+˗]+>|[\p{L}\p{M}_\+˗]+)|(\s*#.*$)|(\s+)|(.+)"

SYMBOL = r"<[\p{L}\p{M}_\+˗]+>|[\p{L}\p{M}_\+˗]+"

FEATURE_TOKEN_SEP = r",\s*"

# Note the four groupings: `for value, feature, separator, bracket, illegal in regex.findall(TOKENIZER, statement)): # ...`
FEATURE_SET_TOKENIZER = r"([+\-0\?]{1,2})\s*([A-Za-z]\w*)|(\s*,\s+|\s+)|(^\[|\]$)|(.+)"

FEATURE = r"\s*([+\-0\?]{1,2}|)\s*([A-Za-z]\w*|)\s*"


FEATURE_VALUES = {"0": 0.0, "-": -1.0, "+": +1.0, "+-": +0.5, "-+": -0.5, "0-": -0.5, "?": float("NaN")}

IGNORE_SYMBOLS = (" ", "ˌ", "ˈ", "/", "[", "]")

ZERO_COST_SYMBOLS = ("<sil>", "<unk>", "<spn>")

DEFAULT_SYMBOL = "<default>"
