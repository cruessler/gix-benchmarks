#!/usr/bin/env python
# /// script
# requires-python = ">=3.13"
# dependencies = [
#     "seaborn",
# ]
# ///

import numpy as np
import pandas as pd

import matplotlib as mpl

import seaborn as sns

import json

filenames = [f"benchmark-{i}.json" for i in range(1, 10)]


def load_results(filename):
    with open(filename) as f:
        return json.load(f)["results"]


results = [load_results(filename) for filename in filenames]


def extract_data_points(result):
    command = result["command"]
    path = result["parameters"]["path"]

    return {"command": command, "path": path, "time": result["times"]}


results = [extract_data_points(result) for result in np.concatenate(results)]

data = pd.concat(
    [pd.DataFrame(result) for result in results],
    ignore_index=True,
)

FIGURE_WIDTH = 10
FIGURE_ASPECT = 1.6

sns.set_theme(
    palette="Paired",
    rc={"figure.figsize": (FIGURE_WIDTH, FIGURE_WIDTH / FIGURE_ASPECT)},
)

boxplot = sns.boxplot(x="time", y="path", hue="command", data=data)
sns.despine(offset=10, trim=True)
mpl.pyplot.tight_layout()

boxplot.get_figure().savefig("boxplot.webp")

catplot = sns.catplot(
    x="time",
    y="path",
    hue="command",
    legend_out=False,
    height=FIGURE_WIDTH / FIGURE_ASPECT,
    aspect=FIGURE_ASPECT,
    data=data,
)

mpl.pyplot.savefig("catplot.webp")

mpl.pyplot.show()
