import sys

rows = []
for line in sys.stdin:
    line = line.strip()
    if line:
        rows.append(list(line))

H = len(rows)
W = len(rows[0])
r = 0
while True:
    next_grid = [["." for _ in row] for row in rows]
    move = False
    for i in range(H):
        for j in range(W):
            if rows[i][j] == ">":
                if rows[i][(j+1) % W] == ".":
                    next_grid[i][(j+1) % W] = ">"
                    move = True
                else:
                    next_grid[i][j] = ">"
            elif rows[i][j] == "v":
                next_grid[i][j] = "v"

    rows = next_grid
    next_grid = [["." for _ in row] for row in rows]
    for i in range(H):
        for j in range(W):
            if rows[i][j] == "v":
                if rows[(i+1)%H][j] == ".":
                    next_grid[(i+1)%H][j] = "v"
                    move = True
                else:
                    next_grid[i][j] = "v"
            elif rows[i][j] == ">":
                next_grid[i][j] = ">"
    r += 1
    if not move:
        break
    rows = next_grid
print(r)
