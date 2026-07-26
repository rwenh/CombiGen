//! demo.rs — Runnable showcase of all combinatorial generators and solvers
//! Mirrors `demo.py`

use combigen::backtracking::{
    count_solutions, format_board, format_sudoku, n_queens, parse_sudoku, solve_sudoku,
};
use combigen::combinatorics::{
    cartesian_product, combinations, combinations_with_replacement, permutations, power_set,
    set_partitions,
};

fn section(title: &str) {
    let width = 56;
    println!();
    println!("{}", "-".repeat(width));
    println!("  {}", title);
    println!("{}", "-".repeat(width));
}

fn demo_permutations() {
    section("PERMUTATIONS");
    let items = vec!["A", "B", "C", "D"];
    for r in [2, 3, items.len()] {
        let perms: Vec<Vec<&str>> = permutations(&items, Some(r)).collect();
        let take = perms.len().min(3);
        println!(
            " P({}, {}) = {:>4} e.g. {:?} ...",
            items.len(),
            r,
            perms.len(),
            &perms[..take]
        );
    }
}

fn demo_combinations() {
    section("COMBINATIONS");
    let items: Vec<i32> = (1..=6).collect();
    for r in [2, 3, 4] {
        let combs: Vec<Vec<i32>> = combinations(&items, r).collect();
        let take = combs.len().min(3);
        println!(
            " C({}, {}) = {:>4} e.g. {:?} ...",
            items.len(),
            r,
            combs.len(),
            &combs[..take]
        );
    }
    println!("\n With replacement:");
    for r in [2, 3] {
        let combs: Vec<Vec<i32>> =
            combinations_with_replacement(&[1, 2, 3], r).collect();
        let take = combs.len().min(4);
        println!(
            " CR(3, {}) = {:>4} e.g. {:?} ...",
            r,
            combs.len(),
            &combs[..take]
        );
    }
}

fn demo_set_partitions() {
    section("SET PARTITIONS");
    for items in [vec![1, 2, 3], vec![1, 2, 3, 4]] {
        let n = items.len();
        let parts: Vec<Vec<Vec<i32>>> = set_partitions(items).collect();
        println!(
            " Bell({}) = {:>3} e.g. {:?} ...",
            n,
            parts.len(),
            &parts[0]
        );
    }
}

fn demo_power_set() {
    section("POWER SET");
    let items = vec![1, 2, 3, 4];
    let ps: Vec<Vec<i32>> = power_set(&items).collect();
    println!("  2^{} = {} subsets", items.len(), ps.len());
    println!(" First 5: {:?}", &ps[..5]);
    println!(" Last 3: {:?}", &ps[ps.len() - 3..]);
}

fn demo_cartesian() {
    section("CARTESIAN PRODUCT");
    let bits: Vec<Vec<i32>> = cartesian_product(vec![vec![0, 1]], 3).collect();
    println!("   {{0,1}}^3 = {} tuples: {:?}", bits.len(), bits);

    // Rust needs one concrete element type per call, so suits and ranks are
    // both coerced to String.
    let suits: Vec<String> = ["♠", "♥", "♦", "♣"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let ranks: Vec<String> = (2..=4).map(|n: i32| n.to_string()).collect();
    let cards: Vec<Vec<String>> = cartesian_product(vec![suits, ranks], 1).collect();
    let take = cards.len().min(6);
    println!(
        " 4 suits × ranks 2-4 = {} cards: {:?} ...",
        cards.len(),
        &cards[..take]
    );
}

fn demo_n_queens() {
    section("N-QUEENS");
    for n in [4, 5, 6, 7, 8] {
        let total = count_solutions(n);
        println!("   {}-QUEENS: {:>4} solutions", n, total);
    }
    println!("\n A random 8-Queens solution:");
    let sol = n_queens(8).next().unwrap();
    println!("{}", format_board(&sol));
    println!("\n Placement (col per row): {:?}", sol);
}

fn demo_sudoku() {
    section("SUDOKU SOLVER");
    let easy =
        "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";

    println!("\n Easy puzzle:");
    let puzzle = parse_sudoku(easy).expect("valid puzzle string");
    println!("{}", format_sudoku(&puzzle));

    let solution = solve_sudoku(&puzzle).next().unwrap();
    println!("\n → Solved:");
    println!("{}", format_sudoku(&solution));
    println!(" Row 1: {:?}", solution[0]);
}

fn demo_generator_laziness() {
    section("GENERATOR LAZINESS (first-only extraction)");
    println!("   Permutations of 10 items — only fetching the first 3:");
    let items: Vec<i32> = (0..10).collect();
    let mut gen = permutations(&items, None);
    for _ in 0..3 {
        println!("   {:?}", gen.next().unwrap());
    }
    println!("  (rest of the ~3.6M perms never computed)");

    println!("\n N-Queens(12) — only the first solution:");
    let sol = n_queens(12).next().unwrap();
    println!("      {:?}", sol);
    println!("      (remaining solutions never explored)");
}

fn main() {
    println!("{}", "=".repeat(56));
    println!("  COMBINATORIAL GENERATOR — DEMO");
    println!("{}", "=".repeat(56));

    demo_permutations();
    demo_combinations();
    demo_set_partitions();
    demo_power_set();
    demo_cartesian();
    demo_n_queens();
    demo_sudoku();
    demo_generator_laziness();

    println!("\n{}", "-".repeat(56));
    println!("  Done.");
    println!("{}", "=".repeat(56));
}
