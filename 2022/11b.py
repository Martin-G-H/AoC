import math
from decimal import *

ROUNDS = 10000


""" Monkey System
# {
#   n : {
#       items : [],
#       operation : {
#           operator : ?,
#           value : n
#       }
#       test: {
#           operator : ?,
#           value : n
#           target: {
#               False : (monkey) n,
#               True : (monkey) n
#           }
#       }
#    }
# }
"""
monkeys = {}

inspections = {}

prod_ = math.prod([7, 2, 19, 3, 13, 11, 5, 17])


def parse_data(data):
    for i in range((len(data) + 1) // 7):
        anchor = i * 7
        monkey = int(data[anchor][1].strip(":"))
        monkey_data = {
            "items": [int(item.strip(",")) for item in data[anchor + 1][4:]],
            "operation": {
                "operator": data[anchor + 2][6],
                "value": data[anchor + 2][7],
            },
            "test": {
                "operator": "/",
                "value": data[anchor + 3][5],
                "target": {True: data[anchor + 4][9], False: data[anchor + 5][9]},
            },
        }
        inspections[monkey] = 0
        monkeys[monkey] = monkey_data


def inspection(data, item):

    if data["value"] == "old":
        other = item
    else:
        other = int(data["value"])

    # print(f"set_old: {time.process_time() - start}")

    match data["operator"]:
        case "/":
            new = int(Decimal(item) / Decimal(other))
        case "*":
            new = int(Decimal(item) * Decimal(other))
        case "+":
            new = int(Decimal(item) + Decimal(other))
        case "-":
            new = int(Decimal(item) - Decimal(other))

    # print(f"old: {item} -> new: {new}")

    # print(f"handle_operation: {time.process_time() - start}")
    return new


def throw(data, item):
    # start = time.process_time()
    monkeys[int(data["target"][item % int(data["value"]) == 0])]["items"].append(item)
    # print(f"Throw: {time.process_time() - start}")


def item_handler(monkey, item):
    # start = time.process_time()
    inspections[n] += 1

    # One monkey inspect item [0]
    #   Worry go up
    new = inspection(monkey["operation"], item)

    # Worry go down by x / 3 rounded down (x // 3)
    new = new % prod_

    # Monkey test worry
    #   Decide where to throw
    throw(monkey["test"], new)
    # print(f"Handler: {time.process_time() - start}")


with open("input.txt", "r") as f:

    data = f.readlines()
    data = [item.strip("\n").split(" ") for item in data]

    parse_data(data)

    print("=== Start ===")

    # routine
    for i in range(ROUNDS):
        # start = time.process_time()
        # if i % 10 == 0:
        print(f" ============ round: {i} =============")

        for n, monkey in monkeys.items():
            # start2 = time.process_time()

            l = len(monkey["items"])
            # print(f"Throwing: {l} items")

            for item in monkey["items"]:
                item_handler(monkey, item)

            monkey["items"].clear()

            # print(
            #    f"Monkey {n}: {time.process_time() - start2}, from start: {time.process_time() - start}"
            # )

        # print(f"Round {i}: {time.process_time() - start}")

    # print(monkeys)
    print(inspections)

    # Answer Question:
    max1 = 0
    max2 = 0
    for k, v in inspections.items():
        if v > max1:
            max2 = max1
            max1 = v
        elif v > max2:
            max2 = v

    print(max1 * max2)

    with open("output.txt", "w") as o:
        o.close()
    f.close()
