def build_data(f):
    crates = []

    while True:
        line = f.readline().strip("\n")

        if line.replace(" ", "").isdigit():
            for stack in crates:
                stack.reverse()
            return crates

        for j, letter in enumerate(line):
            if letter == "[":
                if len(crates) > (j + 1) // 4:
                    crates[(j + 1) // 4].append(line[j + 1])
                else:
                    for k in range(len(crates), ((j + 1) // 4) + 1):
                        crates.append([])
                    crates[(j + 1) // 4].append(line[j + 1])


def move_data(crates, amount, start, end):
    move = []
    for i in range(amount):
        move.append(crates[start - 1].pop())
    move.reverse()
    crates[end - 1].extend(move)


def read_data(crates, data):
    data = [item.strip("\n") for item in data]

    for line in data:
        if line == "":
            continue

        info = line.split(" ")[1::2]
        move_data(crates, int(info[0]), int(info[1]), int(info[2]))


with open("input.txt", "r") as f:

    crates = build_data(f)

    read_data(crates, f.readlines())

    s = ""

    for stack in crates:
        s += stack[-1]

    print(s)

    with open("output.txt", "w") as o:
        o.close()
    f.close()
