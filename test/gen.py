import json
import os


def generate(difficulty: str):
    with open(f"test/puzzles-{difficulty}.json") as f:
        puzzles = json.load(f)

    os.mkdir(f"test/puzzles/{difficulty}")

    solutions = []
    for puzzle in puzzles:
        f = open(f"test/puzzles/{difficulty}/puzzle-{puzzle['id']}.txt", "w")
        solutions.append(puzzle["solution"])

        problem = puzzle["problem"]
        parts = [problem[pos * 9 : pos * 9 + 9] for pos in range(9)]

        f.write("\n".join(parts))
        f.close

    with open(f"test/puzzles/{difficulty}/solutions.txt", "w") as f:
        f.write("\n".join(solutions))


if __name__ == "__main__":
    generate("easy")
    generate("medium")
    generate("hard")
    generate("expert")
    generate("evil")
