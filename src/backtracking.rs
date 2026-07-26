//! backtracking.rs — Backtracking solvers for classic constraint problems
//! Returns `generator::Generator<T>`.

use crate::generator::Generator;
use std::collections::HashSet;
use std::sync::mpsc::SyncSender;

// ---------------------------------------------------------------------------
// Generic backtracking
// ---------------------------------------------------------------------------

/// Generic depth-first, constraint-driven search.
///
/// - `state`: mutable problem state, cloned at each solution.
/// - `is_solution(&state) -> bool`: true when state is a complete solution.
/// - `candidates(&state) -> Vec<Move>`: valid next moves from this state.
/// - `apply_move(&mut state, &Move)`: extend state with a move.
/// - `undo_move(&mut state, &Move)`: retract a move.
///
/// Yields a clone of `state` for every complete solution found.
pub fn backtrack<S, M, IsSolution, Candidates, ApplyMove, UndoMove>(
    state: S,
    is_solution: IsSolution,
    candidates: Candidates,
    apply_move: ApplyMove,
    undo_move: UndoMove,
) -> Generator<S>
where
    S: Clone + Send + 'static,
    M: Send + 'static,
    IsSolution: Fn(&S) -> bool + Send + 'static,
    Candidates: Fn(&S) -> Vec<M> + Send + 'static,
    ApplyMove: Fn(&mut S, &M) + Send + 'static,
    UndoMove: Fn(&mut S, &M) + Send + 'static,
{
    Generator::new(move |tx| {
        fn go<S: Clone, M>(
            state: &mut S,
            is_solution: &dyn Fn(&S) -> bool,
            candidates: &dyn Fn(&S) -> Vec<M>,
            apply_move: &dyn Fn(&mut S, &M),
            undo_move: &dyn Fn(&mut S, &M),
            tx: &SyncSender<S>,
        ) -> bool {
            if is_solution(state) {
                return tx.send(state.clone()).is_ok();
            }
            for m in candidates(state) {
                apply_move(state, &m);
                let keep_going = go(state, is_solution, candidates, apply_move, undo_move, tx);
                undo_move(state, &m);
                if !keep_going {
                    return false;
                }
            }
            true
        }
        let mut state = state;
        go(
            &mut state,
            &is_solution,
            &candidates,
            &apply_move,
            &undo_move,
            &tx,
        );
    })
}

// ---------------------------------------------------------------------------
// N-Queens
// ---------------------------------------------------------------------------

/// Solution is `Vec<usize>` of length `n` where `solution[row] = col`.
///
/// Uses `Vec<bool>` lookup tables instead of hashing once the index domain is
/// small and fixed, avoiding overhead on the hottest loop.
pub fn n_queens(n: usize) -> Generator<Vec<usize>> {
    Generator::new(move |tx| {
        let mut queens: Vec<usize> = Vec::with_capacity(n);
        let mut cols_used = vec![false; n];
        // row - col, shifted by +n to stay non-negative
        let mut diag1_used = vec![false; 2 * n];
        // row + col
        let mut diag2_used = vec![false; 2 * n];

        fn solve(
            row: usize,
            n: usize,
            queens: &mut Vec<usize>,
            cols_used: &mut [bool],
            diag1_used: &mut [bool],
            diag2_used: &mut [bool],
            tx: &SyncSender<Vec<usize>>,
        ) -> bool {
            if row == n {
                return tx.send(queens.clone()).is_ok();
            }
            for col in 0..n {
                let d1 = row + n - col;
                let d2 = row + col;
                if cols_used[col] || diag1_used[d1] || diag2_used[d2] {
                    continue;
                }
                queens.push(col);
                cols_used[col] = true;
                diag1_used[d1] = true;
                diag2_used[d2] = true;

                let keep_going =
                    solve(row + 1, n, queens, cols_used, diag1_used, diag2_used, tx);

                queens.pop();
                cols_used[col] = false;
                diag1_used[d1] = false;
                diag2_used[d2] = false;

                if !keep_going {
                    return false;
                }
            }
            true
        }
        solve(
            0,
            n,
            &mut queens,
            &mut cols_used,
            &mut diag1_used,
            &mut diag2_used,
            &tx,
        );
    })
}

/// Pretty-print an N-Queens solution as a grid.
pub fn format_board(solution: &[usize]) -> String {
    let n = solution.len();
    let mut rows = Vec::with_capacity(n);
    for row in 0..n {
        let mut line = String::new();
        for col in 0..n {
            line.push_str(if solution[row] == col { "Q " } else { ". " });
        }
        rows.push(line.trim_end().to_string());
    }
    rows.join("\n")
}

/// Count the total number of solutions for n-queens.
pub fn count_solutions(n: usize) -> usize {
    n_queens(n).count()
}

// ---------------------------------------------------------------------------
// Sudoku
// ---------------------------------------------------------------------------

/// 9×9 grid, 0 = empty.
pub type Grid = Vec<Vec<u8>>;

/// Return valid digits for (row, col) given the current board.
fn sudoku_candidates(grid: &Grid, row: usize, col: usize) -> HashSet<u8> {
    let mut used: HashSet<u8> = HashSet::new();

    // Row
    used.extend(grid[row].iter().copied());
    // Column
    used.extend((0..9).map(|r| grid[r][col]));
    // 3×3 box
    let (br, bc) = ((row / 3) * 3, (col / 3) * 3);
    for r in br..br + 3 {
        for c in bc..bc + 3 {
            used.insert(grid[r][c]);
        }
    }
    (1..=9u8).filter(|d| !used.contains(d)).collect()
}

/// Return (row, col) of the empty cell with fewest candidates (MRV).
/// `None` means no empty cells remain. Also returns the candidate count.
fn find_empty(g: &Grid) -> (Option<(usize, usize)>, usize) {
    let mut best = None;
    let mut best_count = 10;
    for r in 0..9 {
        for c in 0..9 {
            if g[r][c] == 0 {
                let count = sudoku_candidates(g, r, c).len();
                if count < best_count {
                    best_count = count;
                    best = Some((r, c));
                    if count == 0 {
                        return (best, 0);
                    }
                }
            }
        }
    }
    (best, best_count)
}

/// Most sudoku puzzles have exactly one solution.
pub fn solve_sudoku(grid: &Grid) -> Generator<Grid> {
    let working = grid.clone();
    Generator::new(move |tx| {
        fn solve(g: &mut Grid, tx: &SyncSender<Grid>) -> bool {
            let (cell, count) = find_empty(g);
            let (row, col) = match cell {
                None => return tx.send(g.clone()).is_ok(), // no empty cells → solved
                Some(rc) => rc,
            };
            if count == 0 {
                return true; // dead end, keep searching siblings
            }
            for digit in sudoku_candidates(g, row, col) {
                g[row][col] = digit;
                let keep_going = solve(g, tx);
                g[row][col] = 0;
                if !keep_going {
                    return false;
                }
            }
            true
        }
        let mut working = working;
        solve(&mut working, &tx);
    })
}

/// Pretty-print a Sudoku grid.
pub fn format_sudoku(grid: &Grid) -> String {
    let sep = "+-------+-------+-------+";
    let mut lines = vec![sep.to_string()];
    for r in 0..9 {
        let mut row_str = String::from("| ");
        for c in 0..9 {
            let val = grid[r][c];
            row_str.push_str(&if val != 0 {
                val.to_string()
            } else {
                ".".to_string()
            });
            row_str.push(' ');
            if c == 2 || c == 5 {
                row_str.push_str("| ");
            }
        }
        row_str.push('|');
        lines.push(row_str);
        if r == 2 || r == 5 || r == 8 {
            lines.push(sep.to_string());
        }
    }
    lines.join("\n")
}

/// Parse a compact 81-char string ('.' or '0' = empty) into a 9×9 grid.
pub fn parse_sudoku(s: &str) -> Result<Grid, String> {
    let digits: Vec<u8> = s
        .chars()
        .filter(|ch| ch.is_ascii_digit() || *ch == '.')
        .map(|ch| {
            if ch == '.' {
                0
            } else {
                ch.to_digit(10).unwrap() as u8
            }
        })
        .collect();
    if digits.len() != 81 {
        return Err(format!("Expected 81 cells, got {}", digits.len()));
    }
    Ok((0..9)
        .map(|r| digits[r * 9..(r + 1) * 9].to_vec())
        .collect())
}

// ---------------------------------------------------------------------------
// Word search / Path finder
// ---------------------------------------------------------------------------

/// Yield all simple paths from `start` to `end` in a 2-D grid (0 = blocked).
pub fn find_paths(
    grid: Vec<Vec<i32>>,
    start: (usize, usize),
    end: (usize, usize),
    moves: Option<Vec<(isize, isize)>>,
) -> Generator<Vec<(usize, usize)>> {
    let moves = moves.unwrap_or_else(|| vec![(-1, 0), (1, 0), (0, -1), (0, 1)]);
    Generator::new(move |tx| {
        let rows = grid.len() as isize;
        let cols = if rows > 0 {
            grid[0].len() as isize
        } else {
            0
        };
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        visited.insert(start);

        fn dfs(
            pos: (usize, usize),
            end: (usize, usize),
            grid: &[Vec<i32>],
            moves: &[(isize, isize)],
            rows: isize,
            cols: isize,
            visited: &mut HashSet<(usize, usize)>,
            path: &mut Vec<(usize, usize)>,
            tx: &SyncSender<Vec<(usize, usize)>>,
        ) -> bool {
            if pos == end {
                return tx.send(path.clone()).is_ok();
            }
            let (r, c) = (pos.0 as isize, pos.1 as isize);
            for &(dr, dc) in moves {
                let nr = r + dr;
                let nc = c + dc;
                if nr < 0 || nr >= rows || nc < 0 || nc >= cols {
                    continue;
                }
                let (nru, ncu) = (nr as usize, nc as usize);
                if grid[nru][ncu] == 0 || visited.contains(&(nru, ncu)) {
                    continue;
                }
                visited.insert((nru, ncu));
                path.push((nru, ncu));
                let keep_going =
                    dfs((nru, ncu), end, grid, moves, rows, cols, visited, path, tx);
                path.pop();
                visited.remove(&(nru, ncu));
                if !keep_going {
                    return false;
                }
            }
            true
        }

        let mut path = vec![start];
        dfs(
            start, end, &grid, &moves, rows, cols, &mut visited, &mut path, &tx,
        );
    })
}
