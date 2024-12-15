import time

# input
with open("input.txt", "r") as f:

    cals = f.readlines()

    res = [item.strip("\n") for item in cals]

    sum = 0
    lead = 0

    for num in res:
        if num == "":
            if sum > lead:
                lead = sum
            sum = 0
            continue
        sum += int(num)

    with open("output.txt", "w") as o:
        o.write(str(lead))
        o.close()
    f.close()
