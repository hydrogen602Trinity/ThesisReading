
from typing import Any, List, Tuple
import matplotlib.pyplot as plt
import matplotlib.colors as mcolors

# def tree_builder(lines: List[str]) -> Any:
#     if len(lines) == 1:
#         line = lines[0]
#         assert line.startswith('points')
#         print(line)
#     else:
#         line = lines[0]
#         rest = [l[2:] for l in lines[1:]]
#         part1 = []
#         while 
#         pass

def parse_group(g: str) -> list:
    g = [s.strip() for s in g.split(',')]
    return [tuple(map(float, s[1:-1].split())) for s in g if s]

def main(text: str):
    lines = text.split('\n')
    # tree_builder(lines)
    groups: List[tuple] = [parse_group(l.strip()[len('points = '):]) for l in lines if l.strip().startswith('points = ')]
    # for l in groups:
    #     print(l)

    def helper(ol):
        l = ol.strip()
        return (len(ol) - len(l)) / 2, l[0], float(l[4:])

    axis = [helper(l) for l in lines if 'points' not in l and l.strip()]
    print(axis)

    

    # for dir, value in axis:
    #     if dir == 'Y':
    #         plt.axhline(value)
    #     elif dir == 'X':
    #         plt.axvline(value)

    def setValueInTup(tup, index, val):
        l = list(tup)
        l[index] = val
        return tuple(l)

    def bounds_recurser(splitters, min = (0, 0), max = (30, 30), depth_lvl = 0) -> list:
        if len(splitters) == 0:
            return []
        
        pre = []
        post = []
        mid = None
        for s in splitters:
            if s[0] == depth_lvl:
                assert mid is None, 'multiple hits'
                mid = s
            elif mid is None:
                pre.append(s)
            else:
                post.append(s)

        if mid is None:
            print(splitters, depth_lvl)
        (depth, dir, value) = mid

        index = dir != 'X'

        return [(depth, dir, value, min[not index], max[not index])] +\
            bounds_recurser(pre, min, setValueInTup(max, index, value), depth_lvl+1) +\
            bounds_recurser(post, setValueInTup(min, index, value), max, depth_lvl+1)


        
    to_graph = bounds_recurser(axis)

    for _, dir, val, min, max in to_graph:
        if dir == 'X':
            plt.vlines(val, min, max)
        else:
            plt.hlines(val, min, max)


    # for i in range(len(axis)):
    #     depth, dir, value = axis[i]
        
        
    #     bounds = [0, 30]
    #     for j in range(i+1, len(axis)):
    #         if axis[j][0] < depth and axis[j][1] != dir:
    #             # lesser depth, so higher up the tree
    #             bounds[1] = axis[j][2]
    #             break
        
    #     for j in reversed(range(0, i)):
    #         if axis[j][0] < depth and axis[j][1] != dir:
    #             # lesser depth, so higher up the tree
    #             bounds[0] = axis[j][2]
    #             break
        
    #     # print(dir, value, bounds)
        
    #     if dir == 'X':
    #         plt.vlines(value, bounds[0], bounds[1])
    #     else:
    #         plt.hlines(value, bounds[0], bounds[1]) 
            




    for g, c in zip(groups, mcolors.TABLEAU_COLORS):
        x, y = zip(*g)
        #print(c)
        plt.scatter(x, y, c=c)
    plt.show()


if __name__ == '__main__':
    with open('data.out') as f:
        main(f.read())
