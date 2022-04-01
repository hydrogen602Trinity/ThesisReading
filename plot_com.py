import csv

r = 1e-7

with open('angular.csv') as f:
    reader = csv.reader(f)
    x, y, z = [], [], []
    for row in reader:
        px1, px2, px3 = row
        x.append(float(px1))
        y.append(float(px2))
        z.append(float(px3))

import matplotlib.pyplot as plt

plt.plot(z, 'rx')

plt.show()
