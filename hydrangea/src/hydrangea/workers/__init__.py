import time
import logging
from functools import wraps


logger = logging.getLogger(__name__)


def time_it(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        start = time.perf_counter()
        result = func(*args, **kwargs)
        elapsed = time.perf_counter() - start
        logger.debug("%s took %s seconds to run",
                     func.__name__, f"{elapsed:.6f}")
        return (result, elapsed)
    return wrapper
