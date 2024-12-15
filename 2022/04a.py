def contains(e1, e2):
    return int(e1[0]) >= int(e2[0]) and int(e1[1]) <= int(e2[1])


with open("input.txt", "r") as f:

    data = f.readlines()
    data = [item.strip("\n").split(",") for item in data]

    s = 0
    offset = 0

    print(data)

    for pair in data:
        elf1 = pair[0].split("-")
        elf2 = pair[1].split("-")

        print(elf1)

        if contains(elf1, elf2) or contains(elf2, elf1):
            s += 1

        offset += 2

    print(s)

    with open("output.txt", "w") as o:
        o.close()
    f.close()
