with open("input.txt", "r") as f:

    sum = 0

    letters = "abcdefghijklmnopqrstuvwxyz"
    letters = letters + letters.upper()

    data = f.readlines()
    rucksacks = [item.strip("\n") for item in data]

    for rucksack in rucksacks:

        comp1 = rucksack[: len(rucksack) // 2]
        comp2 = rucksack[len(rucksack) // 2 :]

        same = ""

        for i in comp2:
            if i in comp1:
                same = i

        sum += letters.find(same) + 1

    with open("output.txt", "w") as o:

        o.close()
    f.close()
