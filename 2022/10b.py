import math

width = 40
height = 6

cycle = 0
reg_x = 1


def in_sprite(cycle, x):
    return abs(x - ((cycle - 1) % 40)) <= 1


def is_interest_point(cycle):
    return cycle in [20, 60, 100, 140, 180, 220]
    # return cycle % 40 == 20


with open("input.txt", "r") as f:
    with open("output.txt", "w") as o:

        data = f.readlines()
        data = [item.strip("\n").split(" ") for item in data]

        busy = 0
        instruction = ""

        strength = []

        while data:
            cycle += 1
            busy -= 1

            if busy == 0:
                reg_x += int(instruction[1])

            if in_sprite(cycle, reg_x):
                o.write("#")
            else:
                o.write(".")

            if cycle % 40 == 0:
                o.write("\n")

            if is_interest_point(cycle):
                strength.append(cycle * reg_x)
                print(f"C: {cycle} , X: {reg_x}")

            if busy > 0:
                continue

            instruction = data.pop(0)

            if instruction[0] == "noop":
                continue

            if instruction[0] == "addx":
                busy = 2

        print(strength)
        print(sum(strength))

        o.close()
    f.close()
