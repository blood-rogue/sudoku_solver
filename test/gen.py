import json


def generate(difficulty: str):
    with open(f"test/boards-{difficulty}.json") as f:
        boards = json.load(f)

    solutions = []
    for board in boards:
        f = open(f"test/boards/{difficulty}/board-{board['id']}.txt", "w")
        solutions.append(board["solution"])

        problem = board["problem"]
        parts = [problem[pos * 9 : pos * 9 + 9] for pos in range(9)]

        f.write("\n".join(parts))
        f.close

    with open(f"test/boards/{difficulty}/solutions.txt", "w") as f:
        f.write("\n".join(solutions))


if __name__ == "__main__":
    generate("easy")
    generate("medium")
    generate("hard")
    generate("expert")
    generate("evil")
