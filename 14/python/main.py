import itertools

AIR = "."
ROCK = "#"
SAND = "o"


with open("../sample") as f:

    def initialize_grid():
        grid = [[AIR for _ in range(max_col + 1)] for _ in range(max_row + 1)]
        for start, end in segments:
            while not start == end:
                grid[start[0]][start[1]] = "#"
                start = step(start, end)
            grid[end[0]][end[1]] = "#"
        return grid

    def step(start, end) -> tuple[int, int]:
        def step_int(start, end):
            if start == end:
                return start
            return start + (end - start) // abs(end - start)

        r, c = start
        end_r, end_c = end
        return step_int(r, end_r), step_int(c, end_c)

    def print_grid():
        print("--" * len(grid[0]))
        for row in grid:
            print(" ".join(row))

    def parse_paths(line):
        segments = line.strip().split(" -> ")
        splits = (segment.split(",") for segment in segments)
        return [(int(r), int(c)) for c, r in splits]

    paths = [parse_paths(line) for line in f]
    min_col = min(c for line in paths for r, c in line)
    paths_normalized = [[(r, c - min_col) for r, c in line] for line in paths]
    max_col = max(c for line in paths_normalized for r, c in line)
    max_row = max(r for line in paths_normalized for r, c in line)
    segments = list(
        itertools.chain.from_iterable(zip(path, path[1:]) for path in paths_normalized)
    )
    grid = initialize_grid()
    print_grid()

    def next_sand_pos(r, c):
        if r + 1 >= len(grid):
            return r + 1, c
        if grid[r + 1][c] == AIR:
            return r + 1, c
        elif grid[r + 1][c - 1] == AIR:
            return r + 1, c - 1
        elif grid[r + 1][c + 1] == AIR:
            return r + 1, c + 1
        return r, c

    def in_bounds(r, c):
        return r >= 0 and r < len(grid) and c >= 0 and c < len(grid[0])

    # part 1
    def part1():
        deposited_sand = 0
        sand_start_col = 500 - min_col
        while True:
            # print_grid()
            r, c = 0, sand_start_col
            # while sand in range and sand can fall
            while True:
                next_r, next_c = next_sand_pos(r, c)
                # print(f"{(r,c)=}\n{(next_r, next_c)=}")
                if (r, c) == (next_r, next_c):
                    # deposit sand
                    grid[r][c] = SAND
                    deposited_sand += 1
                    break
                elif not in_bounds(next_r, next_c):
                    print_grid()
                    return deposited_sand
                else:
                    r, c = next_r, next_c

    print(part1())
