import os.path


def path(subpath):
    this_dir = os.path.dirname(__file__)
    return os.path.join(this_dir, subpath)