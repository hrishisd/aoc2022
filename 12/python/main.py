import math
import collections

with open("../input") as f:

    def find(value: str):
        row_idx, row = next((idx, row) for idx, row in enumerate(grid) if value in row)
        col_idx = row.index(value)
        return row_idx, col_idx

    def can_reach(curr: str, dest: str) -> bool:
        def height(cell: str):
            if cell == "S":
                return height("a")
            if cell == "E":
                return height("z")
            return ord(cell) - ord("a") + 1

        return height(dest) - height(curr) <= 1

    def neighbors_in_range(row, col):
        return (
            (r, c)
            for r, c in ((row + 1, col), (row - 1, col), (row, col + 1), (row, col - 1))
            if r >= 0 and c >= 0 and r < len(grid) and c < len(grid[0])
        )

    def reachable_neighbors(row, col):
        return (
            (r, c)
            for r, c in neighbors_in_range(row, col)
            if can_reach(grid[row][col], grid[r][c])
        )

    def neighbors_that_can_reach(row, col):
        return (
            (r, c)
            for r, c in neighbors_in_range(row, col)
            if can_reach(grid[r][c], grid[row][col])
        )

    grid = [line.strip() for line in f]
    start_row, start_col = find("S")
    end_row, end_col = find("E")

    dist_from_start = [[math.inf for _ in row] for row in grid]

    # queue elements are (dist, (row, col))
    queue = collections.deque([(0, (start_row, start_col))])
    while queue:
        dist, (row, col) = queue.popleft()
        if dist < dist_from_start[row][col]:
            dist_from_start[row][col] = dist
            queue.extend(
                ((dist + 1, neighbor) for neighbor in reachable_neighbors(row, col))
            )

    print("part 1:", dist_from_start[end_row][end_col])

    dist_from_end = [[math.inf for _ in row] for row in grid]
    queue = collections.deque([(0, (end_row, end_col))])
    while queue:
        dist, (row, col) = queue.popleft()
        if dist < dist_from_end[row][col]:
            dist_from_end[row][col] = dist
            queue.extend(
                (
                    (dist + 1, neighbor)
                    for neighbor in neighbors_that_can_reach(row, col)
                )
            )
    print(
        "part 2:",
        min(
            dist_from_end[row_idx][col_idx]
            for row_idx in range(len(grid))
            for col_idx in range(len(grid[0]))
            if grid[row_idx][col_idx] == "a"
        ),
    )
