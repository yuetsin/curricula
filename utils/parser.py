#!/usr/bin/env python

c_id = input("Input Course ID >>> ")
c_id = c_id[:2] + '-' + c_id[2:]

credit = "%.2f" % float(input("Input Credit >>> "))

badge_colors = [
    "brightgreen",
    "green",
    "yellowgreen",
    "yellow",
    "orange",
    "red",
    "blue",
    "lightgrey",
    "blueviolet"]

color = badge_colors[int(input("Input badge color [0 - 8] >>> "))]

print("![%s](https://img.shields.io/badge/%s-%s?.svg?style=flat-square)" %
      (c_id, c_id, color))
print("![credit](https://img.shields.io/badge/credit-%s-%s?.svg?style=flat-square)" %
      (credit, color))
