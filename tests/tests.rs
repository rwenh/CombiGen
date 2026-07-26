//! Integration tests for combigen::combinatorics and combigen::backtracking
//! Run: cargo test

use combigen::backtracking::{
    backtrack, count_solutions, find_paths, format_board, format_sudoku, n_queens, parse_sudoku,
    solve_sudoku, Grid,
};
use combigen::combinatorics::{
    cartesian_product, combinations, combinations_with_replacement, integer_partitions,
    permutations, power_set, set_partitions,
};
use std::collections::HashSet;

// ---------------------------------------------------------------------------
// Combinatorics tests
// ---------------------------------------------------------------------------

mod test_permutations {
    use super::*;

    #[test]
    fn test_full() {
        let result: Vec<Vec<i32>> = permutations(&[1, 2, 3], None).collect();
        assert_eq!(result.len(), 6);
        assert!(result.contains(&vec![1, 2, 3]));
        assert!(result.contains(&vec![3, 2, 1]));
    }

    #[test]
    fn test_partial() {
        let result: Vec<Vec<i32>> = permutations(&[1, 2, 3], Some(2)).collect();
        assert_eq!(result.len(), 6);
        assert!(!result.contains(&vec![1, 1]));
    }

    #[test]
    fn test_empty() {
        let result: Vec<Vec<i32>> = permutations::<i32>(&[], Some(0)).collect();
        assert_eq!(result, vec![vec![]]);
    }

    #[test]
    fn test_r_gt_n() {
        let result: Vec<Vec<i32>> = permutations(&[1, 2], Some(5)).collect();
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_single() {
        let result: Vec<Vec<i32>> = permutations(&[42], None).collect();
        assert_eq!(result, vec![vec![42]]);
    }
}

mod test_combinations {
    use super::*;

    #[test]
    fn test_basic() {
        let result: Vec<Vec<i32>> = combinations(&[1, 2, 3, 4], 2).collect();
        assert_eq!(result.len(), 6);
        assert!(result.contains(&vec![1, 2]));
        assert!(!result.contains(&vec![2, 1])); // combinations are ordered by index
    }

    #[test]
    fn test_r_equal_n() {
        let result: Vec<Vec<i32>> = combinations(&[1, 2, 3], 3).collect();
        assert_eq!(result, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn test_r_zero() {
        let result: Vec<Vec<i32>> = combinations(&[1, 2, 3], 0).collect();
        assert_eq!(result, vec![vec![]]);
    }

    #[test]
    fn test_r_gt_n() {
        let result: Vec<Vec<i32>> = combinations(&[1, 2], 5).collect();
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_with_replacement() {
        let result: Vec<Vec<i32>> =
            combinations_with_replacement(&[1, 2], 2).collect();
        assert!(result.contains(&vec![1, 1]));
        assert!(result.contains(&vec![1, 2]));
        assert!(result.contains(&vec![2, 2]));
        assert_eq!(result.len(), 3);
    }
}

mod test_partitions {
    use super::*;

    #[test]
    fn test_integer_partitions_4() {
        let result: Vec<Vec<u32>> = integer_partitions(4, None).collect();
        assert_eq!(result.len(), 5);
        assert!(result.contains(&vec![4]));
        assert!(result.contains(&vec![1, 1, 1, 1]));
        for p in &result {
            assert_eq!(p.iter().sum::<u32>(), 4);
        }
    }

    #[test]
    fn test_integer_partitions_1() {
        let result: Vec<Vec<u32>> = integer_partitions(1, None).collect();
        assert_eq!(result, vec![vec![1]]);
    }

    #[test]
    fn test_set_partitions_count() {
        // Bell numbers: B(1)=1, B(2)=2, B(3)=5, B(4)=15
        for (n, bell) in [(1usize, 1usize), (2, 2), (3, 5), (4, 15)] {
            let items: Vec<i32> = (0..n as i32).collect();
            let result: Vec<Vec<Vec<i32>>> = set_partitions(items).collect();
            assert_eq!(result.len(), bell, "n={}", n);
        }
    }

    #[test]
    fn test_set_partitions_coverage() {
        let items = vec![1, 2, 3];
        for partition in set_partitions(items.clone()) {
            let mut flat: Vec<i32> = partition.into_iter().flatten().collect();
            flat.sort();
            assert_eq!(flat, items);
        }
    }
}

mod test_power_set {
    use super::*;

    #[test]
    fn test_size() {
        let result: Vec<Vec<i32>> = power_set(&[1, 2, 3]).collect();
        assert_eq!(result.len(), 8); // 2^3
    }

    #[test]
    fn test_empty_included() {
        let result: Vec<Vec<i32>> = power_set(&[1, 2]).collect();
        assert!(result.contains(&vec![]));
    }

    #[test]
    fn test_full_set_included() {
        let result: Vec<Vec<i32>> = power_set(&[1, 2, 3]).collect();
        assert!(result.contains(&vec![1, 2, 3]));
    }
}

mod test_cartesian_product {
    use super::*;

    #[test]
    fn test_binary_repeat() {
        let result: Vec<Vec<i32>> = cartesian_product(vec![vec![0, 1]], 3).collect();
        assert_eq!(result.len(), 8);
        assert!(result.contains(&vec![0, 0, 0]));
        assert!(result.contains(&vec![1, 1, 1]));
    }

    #[test]
    fn test_two_sequences() {
        let result: Vec<Vec<i32>> =
            cartesian_product(vec![vec![1, 2], vec![10, 20]], 1).collect();
        assert_eq!(result.len(), 4);
        assert!(result.contains(&vec![1, 10]));
    }
}

// ---------------------------------------------------------------------------
// Backtracking tests
// ---------------------------------------------------------------------------

mod test_n_queens {
    use super::*;

    #[test]
    fn test_counts() {
        let known_counts: [(usize, usize); 8] = [
            (1, 1),
            (2, 0),
            (3, 0),
            (4, 2),
            (5, 10),
            (6, 4),
            (7, 40),
            (8, 92),
        ];
        for (n, expected) in known_counts {
            assert_eq!(count_solutions(n), expected, "n={}", n);
        }
    }

    #[test]
    fn test_solutions_validity() {
        for solution in n_queens(8) {
            let n = solution.len();
            let unique_cols: HashSet<usize> = solution.iter().copied().collect();
            assert_eq!(unique_cols.len(), n);

            let diag1: HashSet<isize> = (0..n)
                .map(|r| r as isize - solution[r] as isize)
                .collect();
            let diag2: HashSet<usize> = (0..n).map(|r| r + solution[r]).collect();
            assert_eq!(diag1.len(), n);
            assert_eq!(diag2.len(), n);
        }
    }

    #[test]
    fn test_first_solution_n4() {
        let sol = n_queens(4).next().unwrap();
        assert_eq!(sol.len(), 4);
    }

    #[test]
    fn test_format_board() {
        let sol = n_queens(4).next().unwrap();
        let board = format_board(&sol);
        assert_eq!(board.matches('Q').count(), 4);
    }
}

mod test_sudoku {
    use super::*;

    const EASY: &str =
        "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";

    #[test]
    fn test_solve() {
        let puzzle = parse_sudoku(EASY).unwrap();
        let solution = solve_sudoku(&puzzle).next().unwrap();
        let full: HashSet<u8> = (1..=9).collect();
        for row in &solution {
            let set: HashSet<u8> = row.iter().copied().collect();
            assert_eq!(set, full);
        }
        for c in 0..9 {
            let set: HashSet<u8> = (0..9).map(|r| solution[r][c]).collect();
            assert_eq!(set, full);
        }
    }

    #[test]
    fn test_unique_solution() {
        let puzzle = parse_sudoku(EASY).unwrap();
        let solutions: Vec<_> = solve_sudoku(&puzzle).collect();
        assert_eq!(solutions.len(), 1);
    }

    #[test]
    fn test_format() {
        let puzzle = parse_sudoku(EASY).unwrap();
        let s = format_sudoku(&puzzle);
        assert!(s.contains('+'));
        assert!(s.contains('|'));
    }

    #[test]
    fn test_parse_invalid() {
        assert!(parse_sudoku("tooshort").is_err());
    }

    #[test]
    fn test_empty_board_has_solution() {
        let empty: Grid = vec![vec![0; 9]; 9];
        let first = solve_sudoku(&empty).next();
        assert!(first.is_some());
    }
}

mod test_generic_backtrack {
    use super::*;

    #[derive(Clone)]
    struct SubsetSumState {
        index: usize,
        chosen: Vec<i32>,
    }

    #[test]
    fn test_subset_sum() {
        let nums = vec![1, 2, 3, 4, 5];
        let target = 6;
        let state = SubsetSumState {
            index: 0,
            chosen: vec![],
        };

        let nums_a = nums.clone();
        let is_solution = move |s: &SubsetSumState| {
            s.index == nums_a.len() && s.chosen.iter().sum::<i32>() == target
        };

        let nums_b = nums.clone();
        let candidates = move |s: &SubsetSumState| -> Vec<bool> {
            if s.index >= nums_b.len() {
                vec![]
            } else {
                vec![true, false] // include or not
            }
        };

        let nums_c = nums.clone();
        let apply_move = move |s: &mut SubsetSumState, include: &bool| {
            if *include {
                s.chosen.push(nums_c[s.index]);
            }
            s.index += 1;
        };

        let undo_move = move |s: &mut SubsetSumState, include: &bool| {
            s.index -= 1;
            if *include {
                s.chosen.pop();
            }
        };

        let results: Vec<SubsetSumState> =
            backtrack(state, is_solution, candidates, apply_move, undo_move).collect();
        assert_eq!(results.len(), 3); // [1,2,3], [1,5], [2,4]
    }
}

mod test_find_paths {
    use super::*;

    #[test]
    fn test_simple_grid() {
        let grid = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let paths: Vec<_> = find_paths(grid, (0, 0), (2, 2), None).collect();
        assert!(!paths.is_empty());
        for path in &paths {
            assert_eq!(path[0], (0, 0));
            assert_eq!(*path.last().unwrap(), (2, 2));
        }
    }

    #[test]
    fn test_no_path() {
        let grid = vec![vec![1, 0], vec![0, 1]];
        let paths: Vec<_> = find_paths(grid, (0, 0), (1, 1), None).collect();
        assert_eq!(paths, Vec::<Vec<(usize, usize)>>::new());
    }
}
