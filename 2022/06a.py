def find_start(data):
    q = []
    for i, word in enumerate(data):

        q.append(word)
        if len(q) > 4:
            q.pop(0)

        print(f"Matching : {word} with q: {q}")

        if len(set(q)) == 4:
            return i


with open("input.txt", "r") as f:

    data = f.readline()
    data = [item.strip("\n") for item in data]

    print(find_start(data=data) + 1)

    with open("output.txt", "w") as o:
        o.close()
    f.close()
