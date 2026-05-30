"""
tests.py - unit tests for combinatorics.py and backtracking.py
Run: python -m pytest tests.py -v OR python tests.py
"""

import unittest
from combinatorics import (
    permutations, combinations, combinations_with_replacements,
    integer_partitions, set_partitions, power_set, cartesian_product
)
from backtracking import (
    n_queens, format_board, count_solutions,
    solve_sudoku, format_sudoku, parse_sudoku,
    find_paths, backtrack
)


# ----------Combinatorics Tests-------------------------------------------------
class TestPermutations(unittest.TestCase):
    def test_full(self):
        result = list(permutations([1, 2, 3]))
        self.assertEqual(len(result), 6)
        self.assertIn((1, 2, 3), result)
        self.assertIn((3, 2, 1), result)

    def test_partial(self):
        result = list(permutations([1, 2, 3], 2))
        self.assertEqual(len(result), 6)
        self.assertNotIn((1, 1), result)

    def test_empty(self):
        self.assertEqual(list(permutations([], 0)), [()])

    def test_r_gt_n(self):
        self.assertEqual(list(permutations([1, 2], 5)), [])

    def test_single(self):
        self.assertEqual(list(permutations([42])), [(42,)])


class TestCombinations(unittest.TestCase):
    def test_basic(self):
        result = list(combinations([1, 2, 3, 4], 2))
        self.assertEqual(len(result), 6)
        self.assertIn((1, 2), result)
        self.assertNotIn((2, 1), result)  # combinations are unordered

    def test_r_equals_n(self):
        result = list(combinations([1, 2, 3], 3))
        self.assertEqual(result, [(1, 2, 3)])

    def test_r_zero(self):
        result = list(combinations([1, 2, 3], 0))
        self.assertEqual(result, [()])

    def test_r_gt_n(self):
        self.assertEqual(list(combinations([1, 2], 5)), [])

    def test_with_replacement(self):
        result = list(combinations_with_replacements([1, 2], 2))
        self.assertIn((1, 1), result)
        self.assertIn((1, 2), result)
        self.assertIn((2, 2), result)
        self.assertEqual(len(result), 3)


class TestPartitions(unittest.TestCase):
    def test_integer_partitions_4(self):
        result = list(integer_partitions(4))
        self.assertEqual(len(result), 5)
        self.assertIn((4,), result)
        self.assertIn((1, 1, 1, 1), result)
        # All parts sum to 4
        for p in result:
            self.assertEqual(sum(p), 4)

    def test_integer_partitions_1(self):
        self.assertEqual(list(integer_partitions(1)), [(1,)])

    def test_set_partitions_count(self):
        # Bell numbers: B(1)=1, B(2)=2, B(3)=5, B(4)=15
        for n, bell in [(1, 1), (2, 2), (3, 5), (4, 15)]:
            result = list(set_partitions(list(range(n))))
            self.assertEqual(len(result), bell)

    def test_set_partitions_coverage(self):
        items = [1, 2, 3]
        for partition in set_partitions(items):
            flat = sorted(x for block in partition for x in block)
            self.assertEqual(flat, items)


class TestPowerSet(unittest.TestCase):
    def test_size(self):
        result = list(power_set([1, 2, 3]))
        self.assertEqual(len(result), 8)  # 2^3

    def test_empty_included(self):
        result = list(power_set([1, 2]))
        self.assertIn((), result)

    def test_full_set_included(self):
        result = list(power_set([1, 2, 3]))
        self.assertIn((1, 2, 3), result)


class TestCartesianProduct(unittest.TestCase):
    def test_binary_repeat(self):
        result = list(cartesian_product([0, 1], repeat=3))
        self.assertEqual(len(result), 8)
        self.assertIn((0, 0, 0), result)
        self.assertIn((1, 1, 1), result)

    def test_two_sequences(self):
        result = list(cartesian_product([1, 2], ['a', 'b']))
        self.assertEqual(len(result), 4)
        self.assertIn((1, 'a'), result)


# ----------Backtracking Tests-----------------------------------------------
class TestNQueens(unittest.TestCase):
    known_counts = {1: 1, 2: 0, 3: 0, 4: 2, 5: 10, 6: 4, 7: 40, 8: 92}

    def test_counts(self):
        for n, expected in self.known_counts.items():
            self.assertEqual(count_solutions(n), expected, f"n={n}")

    def test_solution_validity(self):
        for solution in n_queens(8):
            n = len(solution)
            self.assertEqual(len(set(solution)), n)  # unique cols
            diag1 = set(r - solution[r] for r in range(n))
            diag2 = set(r + solution[r] for r in range(n))
            self.assertEqual(len(diag1), n)
            self.assertEqual(len(diag2), n)

    def test_first_solution_n4(self):
        sol = next(n_queens(4))
        self.assertEqual(len(sol), 4)

    def test_format_board(self):
        sol = next(n_queens(4))
        board = format_board(sol)
        self.assertEqual(board.count("Q"), 4)


class TestSudoku(unittest.TestCase):
    EASY = (
        "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79"
    )

    def test_solve(self):
        puzzle = parse_sudoku(self.EASY)
        solution = next(solve_sudoku(puzzle))
        # Check all rows, cols contain 1-9
        for row in solution:
            self.assertEqual(set(row), set(range(1, 10)))
        for c in range(9):
            col = [solution[r][c] for r in range(9)]
            self.assertEqual(set(col), set(range(1, 10)))

    def test_unique_solution(self):
        puzzle = parse_sudoku(self.EASY)
        solutions = list(solve_sudoku(puzzle))
        self.assertEqual(len(solutions), 1)

    def test_format(self):
        puzzle = parse_sudoku(self.EASY)
        s = format_sudoku(puzzle)
        self.assertIn("+", s)
        self.assertIn("|", s)

    def test_parse_invalid(self):
        with self.assertRaises(ValueError):
            parse_sudoku("tooshort")

    def test_empty_board_has_solutions(self):
        empty = [[0] * 9 for _ in range(9)]
        first = next(solve_sudoku(empty))
        self.assertIsNotNone(first)


class TestGenericBacktrack(unittest.TestCase):
    """Verify the generic engine using subset-sum as a test case."""

    def test_subset_sum(self):
        nums = [1, 2, 3, 4, 5]
        target = 6
        state = {"index": 0, "chosen": []}

        def is_solution(s):
            return s["index"] == len(nums) and sum(s["chosen"]) == target

        def candidates(s):
            if s["index"] >= len(nums):
                return []
            return [True, False]  # include or not

        def apply_move(s, include):
            if include:
                s["chosen"].append(nums[s["index"]])
            s["index"] += 1

        def undo_move(s, include):
            s["index"] -= 1
            if include:
                s["chosen"].pop()

        results = list(backtrack(state, is_solution, candidates, apply_move, undo_move))
        self.assertEqual(len(results), 3)  # [1,2,3], [1,5], [2,4]


class TestFindPaths(unittest.TestCase):
    def test_simple_grid(self):
        grid = [
            [1, 1, 1],
            [1, 0, 1],
            [1, 1, 1],
        ]
        paths = list(find_paths(grid, (0, 0), (2, 2)))
        self.assertTrue(len(paths) > 0)
        for path in paths:
            self.assertEqual(path[0], (0, 0))
            self.assertEqual(path[-1], (2, 2))

    def test_no_path(self):
        grid = [
            [1, 0],
            [0, 1],
        ]
        paths = list(find_paths(grid, (0, 0), (1, 1)))
        self.assertEqual(paths, [])


# ----------Run Tests---------------------------------------------------
if __name__ == "__main__":
    unittest.main(verbosity=2)
