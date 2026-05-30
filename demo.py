"""
demo.py - Runnable showcase of all combinatorial generators and solvers.
"""

from combinatorics import (
    permutations, combinations, combinations_with_replacements,
    integer_partitions, set_partitions, power_set, cartesian_product
)
from backtracking import (
    n_queens, format_board, count_solutions,
    solve_sudoku, format_sudoku, parse_sudoku
)


def section(title: str):
    """Print a section header."""
    width = 56
    print(f"\n{'-' * width}")
    print(f"  {title}")
    print(f"{'-' * width}")


def demo_permutations():
    section("PERMUTATIONS")
    items = ["A", "B", "C", "D"]
    for r in [2, 3, len(items)]:
        perms = list(permutations(items, r))
        print(f"  P({len(items)},{r}) = {len(perms):>4} e.g. {perms[:3]} ...")


def demo_combinations():
    section("COMBINATIONS")
    items = list(range(1, 7))
    for r in [2, 3, 4]:
        combs = list(combinations(items, r))
        print(f" C({len(items)}, {r}) = {len(combs):>4} e.g. {combs[:3]} ...")

    print("\n With replacement:")
    for r in [2, 3]:
        combs = list(combinations_with_replacements([1, 2, 3], r))
        print(f" CR(3, {r}) = {len(combs):>4} e.g. {combs[:4]} ...")

    section("SET PARTITIONS")
    for items in [[1, 2, 3], [1, 2, 3, 4]]:
        parts = list(set_partitions(items))
        print(f"  Bell({len(items)}) = {len(parts):>3} e.g. {parts[0]} ...")


def demo_power_set():
    section("POWER SET")
    items = [1, 2, 3, 4]
    ps = list(power_set(items))
    print(f" 2^{len(items)} = {len(ps)} subsets")
    print(f" First 5: {ps[:5]}")
    print(f" Last 3: {ps[-3:]}")


def demo_cartesian():
    section("CARTESIAN PRODUCT")
    bits = list(cartesian_product([0, 1], repeat=3))
    print(f" {{0,1}}^3 = {len(bits)} tuples: {bits}")

    suits = list(cartesian_product(['♠', '♥', '♦', '♣'], list(range(2, 5))))
    print(f" 4 suits * ranks 2-4 = {len(suits)} cards: {suits[:6]} ...")


def demo_n_queens():
    section("N-QUEENS")
    for n in [4, 5, 6, 7, 8]:
        total = count_solutions(n)
        print(f"  {n}-QUEENS: {total:>4} solutions")

    print("\n A random 8-Queens solution:")
    sol = next(n_queens(8))
    board = format_board(sol)
    print(board)
    print(f"\n Placement (col per row): {sol}")


def demo_sudoku():
    section("SUDOKU SOLVER")
    # Easier puzzle for demo speed
    easy = "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79"

    print(f"\n Easy puzzle:")
    puzzle = parse_sudoku(easy)
    print(format_sudoku(puzzle))

    solution = next(solve_sudoku(puzzle))
    print(f"\n -> Solved:")
    print(format_sudoku(solution))
    print(f"  Row 1: {solution[0]}")


def demo_generator_laziness():
    section("GENERATOR LAZINESS (first-only extraction)")
    print("  Permutations of 10 items - only fetching the first 3:")
    gen = permutations(range(10))
    for _ in range(3):
        print(f"   {next(gen)}")
    print("  (rest of the ~3.6M perms never computed)")

    print("\n N-Queens(12) - only the first solution:")
    sol = next(n_queens(12))
    print(f"   {sol}")
    print("     (remaining solutions never explored)")


if __name__ == "__main__":
    print("=" * 56)
    print("  COMBINATORIAL GENERATOR - Demo")
    print("=" * 56)

    demo_permutations()
    demo_combinations()
    demo_power_set()
    demo_cartesian()
    demo_n_queens()
    demo_sudoku()
    demo_generator_laziness()

    print("\n" + "-" * 56)
    print("  Done.")
    print("=" * 56 + "\n")
