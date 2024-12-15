import math


DIST_CAP = 1.42
ROPE_LENGTH = 10


def dist(a, b):
    return math.sqrt(((a[0] - b[0]) ** 2) + ((a[1] - b[1]) ** 2))


def direction(d):
    match d:
        case "U":
            return 1, 1
        case "L":
            return 0, -1
        case "R":
            return 0, 1
        case "D":
            return 1, -1


def reduce(d):

    if abs(d[0]) > 1:
        if d[0] > 0:
            d[0] -= 1
        elif d[0] < 0:
            d[0] += 1

    if abs(d[1]) > 1:
        if d[1] > 0:
            d[1] -= 1
        elif d[1] < 0:
            d[1] += 1


def move_knot(k1, k2):
    diff = [k1[0] - k2[0], k1[1] - k2[1]]

    reduce(diff)

    k2[0] += diff[0]
    k2[1] += diff[1]


with open("input.txt", "r") as f:

    data = f.readlines()
    data = [item.strip("\n").split(" ") for item in data]

    #                X, Y
    tail_visited = {(0, 0)}

    rope = [[0, 0] for i in range(ROPE_LENGTH)]

    for move in data:
        o, m = direction(move[0])
        for i in range(int(move[1])):
            # Head[x/y] += -1/1
            rope[0][o] += m

            for i, knot in enumerate(rope[1:]):
                if dist(rope[i], knot) > DIST_CAP:
                    move_knot(rope[i], knot)

            tail_visited.add(tuple(rope[-1]))

    print(len(tail_visited))

    with open("output.txt", "w") as o:
        for y in range(-30, 30):
            line = ""
            for x in range(-30, 30):
                if (x, -y) == (0, 0):
                    line += "s"
                    continue
                if (x, -y) in tail_visited:
                    line += "#"
                else:
                    line += "."

            o.write(line + "\n")
        o.close()

    f.close()
