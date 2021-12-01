import logging
import argparse

WINDOW_SIZE = 3
WINDOW_LAST_INDEX= WINDOW_SIZE - 1

parser = argparse.ArgumentParser()
parser.add_argument("-v", "--verbose", dest="verbose", action="store_true", help="verbose")
parsed_args = parser.parse_args()

if parsed_args.verbose:
    logging.basicConfig(format='[%(levelname)s]: %(message)s', level=logging.DEBUG)

with open('input.txt') as f:
    measurements = [int(x) for x in f.readlines()]
increments = 0

window_value = sum(measurements[0:WINDOW_SIZE])

for i in range(0, len(measurements) - WINDOW_SIZE):
    new_window_value = window_value - measurements[i] + measurements[i + WINDOW_SIZE]
    logging.debug(f"Window value: {window_value}, new window value: {new_window_value}")
    if new_window_value > window_value:
        increments += 1
    window_value = new_window_value
print(increments)