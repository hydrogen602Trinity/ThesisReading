import csv

r = 1e-7

with open('data.out') as f:
    reader = csv.reader(f)
    data1 = []
    data2 = []
    for row in reader:
        px1, px2 = row[0], row[3]
        data1.append(float(px1))
        data2.append(float(px2))

import matplotlib.pyplot as plt

# rescale to radii
data1 = [d / r for d in data1]
data2 = [d / r for d in data2]

plt.plot(data1, 'bx')
plt.plot([x + 1 for x in data1], 'b--')
plt.plot([x - 1 for x in data1], 'b--')

plt.plot(data2, 'rx')
plt.plot([x + 1 for x in data2], 'r--')
plt.plot([x - 1 for x in data2], 'r--')
plt.show()
