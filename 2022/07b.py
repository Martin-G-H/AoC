WANTED = 30000000
TOTAL = 70000000


with open("input.txt", "r") as f:

    data = f.readlines()
    data = [item.strip("\n").split(" ") for item in data]

    dir = {"_/": 0}
    dir_stack = []

    for i, command in enumerate(data):
        if command[0] == "$":
            if command[1] == "cd":
                if command[2] == "..":
                    dir_stack.pop()
                else:
                    dir_stack.append(command[2])

        elif command[0] == "dir":
            dir_key = ""
            for key in dir_stack:
                dir_key += "_" + key
            dir[dir_key + "_" + command[1]] = 0

        else:
            for i, d in enumerate(dir_stack):
                if i != 0:
                    dir_key = ""
                    for key in dir_stack[: i + 1]:
                        dir_key += "_" + key

                    dir[dir_key] += int(command[0])

                else:
                    dir["_" + d] += int(command[0])

    print(dir)

    s = dir["_/"]
    print(s)

    free = TOTAL - s
    print(free)

    free_up = WANTED - free
    print(free_up)

    _min = TOTAL

    for v in dir.values():
        if v >= free_up and v < _min:
            _min = v

    print(_min)

    with open("output.txt", "w") as o:
        o.close()
    f.close()
