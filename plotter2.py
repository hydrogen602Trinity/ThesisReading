from typing import List, Tuple
import matplotlib.pyplot as plt
import csv
from dataclasses import dataclass


@dataclass
class Vector:
    x: float
    y: float
    z: float


@dataclass
class Particle:
    pos: Vector
    vel: Vector


@dataclass
class Energy:
    e: float
    global_u: float
    pair_u: float
    ke: float


with open('full_data.csv') as f:
    data: List[Tuple[Particle, Particle, Energy]] = []
    for rest in csv.reader(f):
        x1, y1, z1, x2, y2, z2, vx1, vy1, vz1, vx2, vy2, vz2, e, global_u, pair_u, ke = map(
            float, rest)

        p1 = Particle(Vector(x1, y1, z1), Vector(vx1, vy1, vz1))
        p2 = Particle(Vector(x2, y2, z2), Vector(vx2, vy2, vz2))

        data.append((p1, p2, Energy(e, global_u, pair_u, ke)))

r = 1e-7

fig, (plot1, plot2, plot3) = plt.subplots(3, 1, sharex=True)

# rescale to radii
data1 = [p1.pos.x / r for p1, _, _ in data]
data2 = [p2.pos.x / r for _, p2, _ in data]


plot1.plot(data1, 'bx')
plot1.plot([x + 1 for x in data1], 'b--')
plot1.plot([x - 1 for x in data1], 'b--')

plot1.plot(data2, 'rx')
plot1.plot([x + 1 for x in data2], 'r--')
plot1.plot([x - 1 for x in data2], 'r--')

plot1.set_ylabel(f'x-coord in radii ({r})')
plot1.set_ylim(-10, 10)

plot2.plot([e.e for _, _, e in data], 'bx-', label='Energy')
plot2.plot([e.global_u for _, _, e in data], 'rx-', label='Global Force U')
plot2.plot([e.pair_u for _, _, e in data], 'gx-', label='Spring U')
plot2.plot([e.ke for _, _, e in data], 'cx-', label='KE')
plot2.legend(loc='right')
plot2.set_ylabel('Energy')

plot3.plot([abs(p.vel.x) for p, _, _ in data], 'bx-')
plot3.plot([abs(p.vel.x) for _, p, _ in data], 'rx-')
plot3.set_ylabel('Magnitude of Velocity')

plt.show()
