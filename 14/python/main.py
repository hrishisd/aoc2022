import itertools
from collections import defaultdict
import math

AIR = "."
ROCK = "#"
SAND = "o"

Point = tuple[int, int]

with open("../input") as f:

    def print_grid():
        rows = [r for r, _ in grid]
        cols = [c for _, c in grid]
        min_col = min(cols)
        arr = [
            [AIR for _ in range(min_col, max(cols) + 1)] for _ in range(max(rows) + 1)
        ]
        for (r, c), p in grid.items():
            arr[r][c - min_col] = p
        for i, row in enumerate(arr):
            if i < 10:
                print(str(i) + "  " + "".join(row))
            else:
                print(str(i) + " " + "".join(row))

    def initialize_grid():
        grid = defaultdict(lambda: AIR)
        for start, end in segments:
            while not start == end:
                grid[start] = "#"
                start = step(start, end)
            grid[end] = "#"
        return grid

    def step(start: Point, end: Point) -> Point:
        def step_int(start, end):
            if start == end:
                return start
            return start + (end - start) // abs(end - start)

        r, c = start
        end_r, end_c = end
        return step_int(r, end_r), step_int(c, end_c)

    def parse_paths(line):
        segments = line.strip().split(" -> ")
        splits = (segment.split(",") for segment in segments)
        return [(int(r), int(c)) for c, r in splits]

    paths = [parse_paths(line) for line in f]
    max_row = max(r for path in paths for r, c in path)
    print(f"{max_row=}")
    segments = list(
        itertools.chain.from_iterable(zip(path, path[1:]) for path in paths)
    )
    grid = initialize_grid()

    def get_next_sand_pos(sand_pos, floor_row=math.inf):
        r, c = sand_pos
        if r + 1 == floor_row:
            return (r, c)
        if grid[(r + 1, c)] == AIR:
            return r + 1, c
        elif grid[(r + 1, c - 1)] == AIR:
            return r + 1, c - 1
        elif grid[(r + 1, c + 1)] == AIR:
            return r + 1, c + 1
        return r, c

    def in_bounds(r, c):
        return r >= 0 and r < len(grid) and c >= 0 and c < len(grid[0])

    sand_start_pos = (0, 500)

    def part1():
        deposited_sand = 0
        while True:
            # print_grid()
            sand_pos = sand_start_pos
            # while sand in range and sand can fall
            while True:
                next_sand_pos = get_next_sand_pos(sand_pos)
                # print(f"{(r,c)=}\n{(next_r, next_c)=}")
                if sand_pos == next_sand_pos:
                    # deposit sand
                    grid[sand_pos] = SAND
                    deposited_sand += 1
                    break
                elif next_sand_pos[0] > max_row:
                    return deposited_sand
                else:
                    sand_pos = next_sand_pos

    def part2():
        floor_row = max_row + 2
        print(f"{floor_row=}")
        deposited_sand = 0
        while grid[sand_start_pos] == AIR:
            sand_pos = sand_start_pos
            # deposit one piece of sand
            while True:
                next_sand_pos = get_next_sand_pos(sand_pos, floor_row)
                if sand_pos == next_sand_pos:
                    # deposit sand
                    grid[sand_pos] = SAND
                    deposited_sand += 1
                    break
                else:
                    sand_pos = next_sand_pos
        return deposited_sand

    print("part 1", part1())
    grid = initialize_grid()
    print("part 2", part2())
