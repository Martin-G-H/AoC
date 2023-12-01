def is_edge(r, c):
    return r == 0 or c == 0 or r == r_max - 1 or c == c_max - 1


with open("input.txt", "r") as f:

    data = f.readlines()
    data = [item.strip("\n") for item in data]

    is_visible = [[1 for c in data[0]] for r in data]
    r_max = len(data)
    c_max = len(data[0])

    # time for a shitshow !DO NOT COPY WARNING BAD CODE

    for r in range(len(data)):
        for c in range(len(data[0])):
            if is_edge(r, c):
                is_visible[r][c] = 0
                continue

            if (r, c) == (3, 2):
                print("false")

            # Up
            max_height = data[r][c]
            l = 0
            for offset in range(1, r + 1):
                l += 1
                if data[r - offset][c] >= max_height or is_edge(r - offset, c):
                    is_visible[r][c] *= l
                    break

            # Left
            max_height = data[r][c]
            l = 0
            for offset in range(1, c + 1):
                l += 1
                if data[r][c - offset] >= max_height or is_edge(r, c - offset):
                    is_visible[r][c] *= l
                    break

            # Down
            max_height = data[r][c]
            l = 0
            for n_r in range(r + 1, r_max):
                l += 1
                if data[n_r][c] >= max_height or is_edge(n_r, c):
                    is_visible[r][c] *= l
                    break

            # Right
            max_height = data[r][c]
            l = 0
            for n_c in range(c + 1, c_max):
                l += 1
                if data[r][n_c] >= max_height or is_edge(r, n_c):
                    is_visible[r][c] *= l
                    break

    m = []
    for row in is_visible:
        m.append(max(row))

    print(max(m))

    with open("output.txt", "w") as o:
        o.close()
    f.close()
