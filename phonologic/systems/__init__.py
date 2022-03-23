import os

_EXT = ".phl"

_THIS_DIR = os.path.dirname(__file__)

PREDEFINED = {
    filename[:-4]: os.path.join(_THIS_DIR, filename)
    for filename in os.listdir(_THIS_DIR)
    if filename.endswith(_EXT)
}