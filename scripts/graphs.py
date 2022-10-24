import matplotlib.pyplot as plt
from pathlib import Path
from csv import reader
import numpy as np

base = Path("..").resolve()
data_dir = base / "data/output"
assert data_dir.exists()

# Categorize the output of different program configurations by filename

# Champion & Challenger - All Approaches
for pattern, label, errorevery in [("output_b_r?.csv", "Random Search", 10)]:
    data = {}
    for csv_path in data_dir.glob(pattern):
        with open(str(csv_path)) as f:
            reading = reader(f)
            for line in reading:
                if not line:
                    break
                tag = int(line[1])
                if tag not in data:
                    data[tag] = [[float(line[2]), float(line[3])]]
                    continue
                data[tag] += [[float(line[2]), float(line[3])]]

    x = [*data.keys()]
    ys = np.array([*data.values()])[:, :, 0] # 0 is running best, 1 is challenger from each epoch
    y = np.mean(ys, axis = 1)
    errs = np.std(ys, axis = 1)

    plt.errorbar(x, y, yerr = errs, errorevery = errorevery, label = label)

plt.suptitle("Learning Curves")
plt.legend(loc = "lower right")
plt.xlabel("Evaluations")
plt.ylabel("Fitness")
plt.savefig(base / "graph_s.png",
            dpi = 650, bbox_inches = "tight")
plt.close()

# Champion & Challenger - Detailed View
for pattern, label in [("output_b_r?.csv", "Random Search")]:
    data = {}
    for csv_path in data_dir.glob(pattern):
        with open(str(csv_path)) as f:
            reading = reader(f)
            for line in reading:
                if not line:
                    break
                tag = int(line[1])
                if tag not in data:
                    data[tag] = [[float(line[2]), float(line[3])]]
                    continue
                data[tag] += [[float(line[2]), float(line[3])]]

    x = [*data.keys()]
    y = np.array([*data.values()])[:, :, 0] # 0 is running best, 1 is challenger from each epoch

    plt.plot(x, y)

plt.xlabel("Evaluations")
plt.ylabel("Fitness")
plt.suptitle("Random Search")
plt.savefig(base / "graph_s_dot.png",
            dpi = 650, bbox_inches = "tight")
plt.close()
