import json
import matplotlib.pyplot as plt
import numpy as np

with open('position.txt') as f:
    data = f.read()

data = '[' + data.strip().replace('\n', ',') + ']'
# get rid of the last trailing newline so that theres no trailing comma after replace

data_np: np.ndarray = np.array(json.loads(data))

point_count, particle_count, dim_count = data_np.shape
assert dim_count == 3
assert particle_count == 2

data_np = data_np.transpose((1, 0, 2))
#data_np = data_np.reshape(particle_count, point_count, dim_count)
print(data_np.shape, data_np.dtype)

t = np.arange(0, point_count, 1)
p: np.ndarray
for p in data_np:
    x, y, _ = p.T
    # y = [(px, py) for px, py, pz in p] # remove z because of plotting in 2D

    plt.scatter(x, y)

plt.xlim((-2, 2))
plt.ylim((-2, 2))
# plt.axis('square')
plt.gca().set_aspect('equal', adjustable='box')
#plt.draw()
plt.show()
