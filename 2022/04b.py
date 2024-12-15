def overlap(e1, e2):
    return (
        int(e1[0]) <= int(e2[0])
        and int(e1[1]) >= int(e2[0])
        or int(e1[0]) <= int(e2[1])
        and int(e1[1]) >= int(e2[1])
        or int(e1[0]) <= int(e2[0])
        and int(e1[1]) >= int(e2[1])
        or int(e1[0]) >= int(e2[0])
        and int(e1[1]) <= int(e2[1])
    )


with open("input.txt", "r") as f:

    data = f.readlines()
    data = [item.strip("\n").split(",") for item in data]

    s = 0
    offset = 0

    for pair in data:
        elf1 = pair[0].split("-")
        elf2 = pair[1].split("-")

        if overlap(elf1, elf2) or overlap(elf1, elf2):
            s += 1

        offset += 2

    print(s)

    with open("output.txt", "w") as o:
        o.close()
    f.close()
