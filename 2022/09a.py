import math

DIST_CAP = 1.42


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


with open("input.txt", "r") as f:

    data = f.readlines()
    data = [item.strip("\n").split(" ") for item in data]

    #                X, Y
    tail_visited = {(0, 0)}

    #       X, Y
    tail = [0, 0]
    head = [0, 0]
    head_shadow = [0, 0]

    for move in data:
        o, m = direction(move[0])
        for i in range(int(move[1])):
            head[o] += m

            if dist(head, tail) > DIST_CAP:
                tail = head_shadow.copy()
                tail_visited.add(tuple(tail))

            head_shadow = head.copy()
            # print(f"head: {head}, tail: {tail}")

    print(len(tail_visited))

    with open("output.txt", "w") as o:
        o.close()
    f.close()
