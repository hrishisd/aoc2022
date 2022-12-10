import math

with open("../input") as f:
    trees = [[int(c) for c in line.rstrip()] for line in f]
    num_rows = len(trees)
    num_cols = len(trees[0])

    def find_visible_trees() -> int:
        acc = set()

        def add_visible_in_line(start_row, start_col, row_step=0, col_step=0):
            row, col = start_row, start_col
            max_height_so_far = -1
            while 0 <= row and row < num_rows and 0 <= col and col < num_cols:
                height = trees[row][col]
                if height > max_height_so_far:
                    acc.add((row, col))
                    max_height_so_far = height
                row += row_step
                col += col_step

        for row in range(num_rows):
            add_visible_in_line(row, 0, col_step=1)
            add_visible_in_line(row, num_cols - 1, col_step=-1)
        for col in range(num_cols):
            add_visible_in_line(0, col, row_step=1)
            add_visible_in_line(num_rows - 1, col, row_step=-1)
        return acc

    visible_trees = find_visible_trees()
    print("part 1: ", len(visible_trees))

    def part2() -> int:
        def scenic_score(row, col):
            height = trees[row][col]

            def view_distance(row_step, col_step) -> int:
                r, c = row, col
                distance = 0
                while True:
                    r += row_step
                    c += col_step
                    if 0 > r or r >= num_rows or 0 > c or c >= num_cols:
                        break
                    distance += 1
                    if height <= trees[r][c]:
                        break
                return distance

            result = math.prod(
                view_distance(row_step, col_step)
                for row_step, col_step in ((0, 1), (0, -1), (1, 0), (-1, 0))
            )
            return result

        return max(scenic_score(row, col) for row, col in visible_trees)

    print("part 2: ", part2())
