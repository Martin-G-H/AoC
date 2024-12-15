letter_lookup = {
    "A": "Rock",
    "B": "Paper",
    "C": "Scissor",
    "X": "Rock",
    "Y": "Paper",
    "Z": "Scissor",
}

win_lookup = {
    ("Rock", "Rock"): "Draw",
    ("Rock", "Paper"): "Loss",
    ("Rock", "Scissor"): "Win",
    ("Paper", "Rock"): "Win",
    ("Paper", "Paper"): "Draw",
    ("Paper", "Scissor"): "Loss",
    ("Scissor", "Rock"): "Loss",
    ("Scissor", "Paper"): "Win",
    ("Scissor", "Scissor"): "Draw",
}

score_lookup = {"Rock": 1, "Paper": 2, "Scissor": 3, "Loss": 0, "Draw": 3, "Win": 6}


def calc_score(a, b):
    pick_a = letter_lookup[a]
    pick_b = letter_lookup[b]
    result_score = score_lookup[win_lookup[(pick_b, pick_a)]]
    pick_score = score_lookup[pick_b]

    return result_score + pick_score


with open("input.txt", "r") as f:

    data = f.readlines()
    data = [item.strip("\n").split(" ") for item in data]

    #print(data)

    sum = 0
    for round in data:
        sum += calc_score(round[0], round[1])

    with open("output.txt", "w") as o:
        o.write(str(sum))
        o.close()
    f.close()
