with open("input.txt", "r") as f:

    data = f.readlines()
    data = [item.strip("\n") for item in data]

    is_visible = [[False for c in data[0]] for r in data]
    r_max = len(data)
    c_max = len(data[0])

    # time for a shitshow !DO NOT COPY WARNING BAD CODE

    for r in range(len(data)):
        for c in range(len(data[0])):
            if r == 0 or c == 0 or r == r_max - 1 or c == c_max - 1:
                is_visible[r][c] = True
                continue

            if (r, c) == (1, 3):
                print("false")

            # -x
            height = data[r][c]
            for n_c in range(0, c):
                if data[r][n_c] >= height:
                    break
            else:
                is_visible[r][c] = True
                continue

            # +x
            height = data[r][c]
            for n_c in range(c + 1, c_max):
                if data[r][n_c] >= height:
                    break
            else:
                is_visible[r][c] = True
                continue

            # +y
            height = data[r][c]
            for n_r in range(0, r):
                if data[n_r][c] >= height:
                    break
            else:
                is_visible[r][c] = True
                continue

            # -y
            height = data[r][c]
            for n_r in range(r + 1, r_max):
                if data[n_r][c] >= height:
                    break
            else:
                is_visible[r][c] = True
                continue

    s = 0
    for row in is_visible:
        s += sum(row)

    print(s)

    with open("output.txt", "w") as o:
        o.close()
    f.close()
