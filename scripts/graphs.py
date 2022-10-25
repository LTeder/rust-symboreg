import matplotlib.pyplot as plt
from pathlib import Path
from csv import reader
import numpy as np

base = Path("..").resolve()
data_dir = base / "data/output"
assert data_dir.exists()

# Categorize the output of different program configurations by filename
    
# Thanks https://stackoverflow.com/a/10685869/5379649
def shrink(data, cols):
    shaped = data.reshape(cols, int(data.shape[0] / cols))
    return shaped.sum(axis = 1)

# Champion & Challenger - Gold, All Approaches
for pattern, label in [("output_g_e?.csv", "Evolutionary Algorithm"),
                       ("output_g_r?.csv", "Hill Climber"),
                       ("output_g_g?.csv", "Random Search")]:
    data = {}
    for csv_path in data_dir.glob(pattern):
        with open(str(csv_path)) as f:
            reading = reader(f)
            for line in reading:
                if not line:
                    break
                tag = int(line[1])
                if tag not in data:
                    data[tag] = []
                data[tag] += [[float(line[2]), float(line[3])]]
    x = np.array([*data.keys()])
    ys = np.array([*data.values()])[:, :, 0] # 0 is running best, 1 is challenger from each epoch
    ys = [y for (_, y) in sorted(zip(x, ys), key = lambda pair: pair[0])]
    x = shrink(np.sort(x), 100)
    y = shrink(np.mean(ys, axis = 1), 100)
    plt.plot(x, y, label = label)
    #errs = shrink(np.std(ys, axis = 1), 100)
    #plt.errorbar(x, y, yerr = errs, errorevery = 10, label = label)
plt.suptitle("Learning Curves, Gold")
plt.legend(loc = "lower right")
plt.xlabel("Evaluations")
plt.ylabel("Fitness")
plt.yscale("log")
plt.xscale("log")
plt.savefig(base / "graph_g.png", dpi = 650, bbox_inches = "tight")
plt.close()

# Champion & Challenger - Bronze, All Approaches
for pattern, label in [("output_b_e?.csv", "Evolutionary Algorithm"),
                       ("output_b_r?.csv", "Hill Climber")]:
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
    x = np.array([*data.keys()])
    ys = np.array([*data.values()])[:, :, 0] # 0 is running best, 1 is challenger from each epoch
    ys = [y for (_, y) in sorted(zip(x, ys), key = lambda pair: pair[0])]
    x = shrink(np.sort(x), 100)
    y = shrink(np.mean(ys, axis = 1), 100)
    plt.plot(x, y, label = label)
plt.suptitle("Learning Curves, Bronze")
plt.legend(loc = "lower right")
plt.xlabel("Evaluations")
plt.ylabel("Fitness")
plt.yscale("log")
plt.xscale("log")
plt.savefig(base / "graph_b.png", dpi = 650, bbox_inches = "tight")
plt.close()

# Champion & Challenger - Gold, Detailed View
for pattern, label in [("output_g_e?.csv", "Evolutionary Algorithm")]:
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
    x = np.array([*data.keys()])
    plt.plot(x, np.array([*data.values()])[:, :, 0],
             marker = "o", ms = 3, linestyle = "None")
plt.suptitle("Evolutionary Algorithm Dot Plot, Gold")
plt.xlabel("Evaluations")
plt.ylabel("Fitness")
#plt.yscale("log")
plt.xscale("log")
plt.savefig(base / "graph_g_dot.png", dpi = 650, bbox_inches = "tight")
plt.close()
