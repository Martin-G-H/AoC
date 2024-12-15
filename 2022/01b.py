import time

# input
with open("input.txt", "r") as f:

    cals = f.readlines()

    res = [item.strip("\n") for item in cals]

    sums = []
    s = 0

    for num in res:
        if num == "":
            sums.append(s)
            s = 0
            continue
        s += int(num)

    sums.sort(reverse=True)

    with open("output.txt", "w") as o:
        o.write(str(sum(sums[:3])))
        o.close()
    f.close()
