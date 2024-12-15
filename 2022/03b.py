with open("input.txt", "r") as f:

    sum = 0
    letters = "abcdefghijklmnopqrstuvwxyz"
    letters = letters + letters.upper()

    data = f.readlines()
    rucksacks = [item.strip("\n") for item in data]

    elf1 = ""
    elf2 = ""
    badge = ""

    for i, rucksack in enumerate(rucksacks):

        match i % 3:
            case 2:
                badge = [item for item in rucksack if (item in elf1 and item in elf2)]
                sum += letters.find(badge[0]) + 1
            case 1:
                elf2 = rucksack
            case 0:
                elf1 = rucksack

    print(sum)

    with open("output.txt", "w") as o:

        o.close()
    f.close()
