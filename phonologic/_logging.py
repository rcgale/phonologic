import logging

import phonologic

logger = logging.getLogger(phonologic.__name__)

formatter = logging.Formatter('%(name)s %(levelname)s: %(message)s')
handler = logging.StreamHandler()
handler.setFormatter(formatter)
logger.addHandler(handler)
