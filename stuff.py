import random
def f():
	return round(random.random()*30, 2)

print([(f(), f()) for _ in range(50)])