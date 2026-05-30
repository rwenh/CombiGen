"""
backtracking.py -- Backtracking solvers for classic constraint problems.

Each solver is a generator that yields solutions lazily, so you can
retrieve just the first solution or exhaust all of them.
"""

from typing import Generator
import copy

# --------Backtracking Engine-----------------------------------------------
def backtrack(
    state,
    is_solution,
    candidates,
    apply_move,
    undo_move,
) -> Generator:
    """
    Generic backtracking engine (depth-first, constraint-driven).
    Args:
        state:   Mutable problem state.
        is_solution: state -> bool - True when state is a complete solution.
        candidates: state -> iterable - valid next moves from this state.
        apply_move: (state, move) -> None - extend state with move.
        undo_move: (state, move) -> None - retract the move.
    Yields:
        A deep-copy snapshot of state for each complete solution found.
    """
    if is_solution(state):
        yield copy.deepcopy(state)
        return

    for move in candidates(state):
        apply_move(state, move)
        yield from backtrack(state, is_solution, candidates, apply_move, undo_move)
        undo_move(state, move)


# ------N-Queens-------------------------------------------------------------------
def n_queens(n: int) -> Generator[list[int], None, None]:
    """
    Yield all solutions to the N-Queens problem.
    Each solution is a list of length n where solution[row] = col
    gives the column of the queen in that row.
    """
    queens: list[int] = []          # queens[row] = col
    cols_used: set[int] = set()
    diag1_used: set[int] = set()    # row - col
    diag2_used: set[int] = set()    # row + col

    def _solve(row: int):
        if row == n:
            yield list(queens)
            return
        for col in range(n):
            d1, d2 = row - col, row + col
            if col in cols_used or d1 in diag1_used or d2 in diag2_used:
                continue
            queens.append(col)
            cols_used.add(col)
            diag1_used.add(d1)
            diag2_used.add(d2)
            yield from _solve(row + 1)
            queens.pop()
            cols_used.discard(col)
            diag1_used.discard(d1)
            diag2_used.discard(d2)

    yield from _solve(0)


def format_board(solution: list[int]) -> str:
    """Pretty-print an N-Queens solution as a grid."""
    n = len(solution)
    rows = []
    for row in range(n):
        line = ""
        for col in range(n):
            line += "Q " if solution[row] == col else ". "
        rows.append(line.rstrip())
    return "\n".join(rows)


def count_solutions(n: int) -> int:
    """Return the total number of solutions for n-queens (iterates fully)."""
    return sum(1 for _ in n_queens(n))


# ------Sudoku---------------------------------------------------------------
Grid = list[list[int]]  # 9x9, 0 = empty


def _sudoku_candidates(grid: Grid, row: int, col: int) -> set[int]:
    """Return valid digits for (row, col) given current board."""
    used: set[int] = set()

    # Row
    used.update(grid[row])
    # Column
    used.update(grid[r][col] for r in range(9))
    # 3x3 box
    br, bc = (row // 3) * 3, (col // 3) * 3
    for r in range(br, br + 3):
        for c in range(bc, bc + 3):
            used.add(grid[r][c])

    return set(range(1, 10)) - used


def solve_sudoku(grid: Grid) -> Generator[Grid, None, None]:
    """
    Yield all solutions for a Sudoku puzzle.
    Most puzzles have exactly one solution.
    """
    def _find_empty(g: Grid):
        """Return (row, col) of the empty cell with fewest candidates (MRV)."""
        best = None
        best_count = 10
        for r in range(9):
            for c in range(9):
                if g[r][c] == 0:
                    count = len(_sudoku_candidates(g, r, c))
                    if count < best_count:
                        best_count = count
                        best = (r, c)
                        if count == 0:
                            return best, 0
        return best, best_count

    def _solve(g: Grid):
        cell, count = _find_empty(g)
        if cell is None:             # No empty cells -> solved
            yield copy.deepcopy(g)
            return
        if count == 0:               # Dead end
            return

        row, col = cell
        for digit in _sudoku_candidates(g, row, col):
            g[row][col] = digit
            yield from _solve(g)
            g[row][col] = 0

    # Work on a copy
    working = [row[:] for row in grid]
    yield from _solve(working)


def format_sudoku(grid: Grid) -> str:
    """Pretty-print a Sudoku grid."""
    sep = "+-------+-------+-------+"
    lines = [sep]
    for r in range(9):
        row_str = "| "
        for c in range(9):
            val = grid[r][c]
            row_str += (str(val) if val else ".") + " "
            if c in (2, 5):
                row_str += "| "
        lines.append(row_str + "|")
        if r in (2, 5, 8):
            lines.append(sep)
    return "\n".join(lines)


def parse_sudoku(s: str) -> Grid:
    """
    Parse a compact 81-char string (. or 0 = empty) into a 9x9 grid.
    """
    digits = [int(ch) if ch.isdigit() and ch != "0" else 0
              for ch in s if ch in "0123456789."]
    if len(digits) != 81:
        raise ValueError(f"Expected 81 cells, got {len(digits)}")
    return [digits[r * 9:(r + 1) * 9] for r in range(9)]


# --------Word Search / Path Finder ------------------------------------------
def find_paths(grid: list[list], start, end, moves=None) -> Generator[list, None, None]:
    """
    Yield all simple paths from start to end in a 2-D grid.
    """
    if moves is None:
        moves = [(-1, 0), (1, 0), (0, -1), (0, 1)]  # Fixed: was (o, 1)

    rows, cols = len(grid), len(grid[0])
    visited: set = set()

    def _dfs(pos, path):
        if pos == end:
            yield list(path)
            return
        r, c = pos
        for dr, dc in moves:
            nr, nc = r + dr, c + dc
            if (0 <= nr < rows and 0 <= nc < cols and
                grid[nr][nc] and (nr, nc) not in visited):
                visited.add((nr, nc))           # Fixed: was add(nr, nc)
                path.append((nr, nc))
                yield from _dfs((nr, nc), path)
                path.pop()
                visited.discard((nr, nc))

    visited.add(start)
    yield from _dfs(start, [start])
