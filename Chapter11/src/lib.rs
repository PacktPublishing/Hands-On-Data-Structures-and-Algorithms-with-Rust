#![feature(test, bind_by_move_pattern_guards)]
mod knapsack;

mod nqueens;
mod prng;
use knapsack::{Item, Knapsack, Backtracking, DynamicProgramming};
use nqueens::ChessBoard;

#[cfg(test)]
mod tests {
    extern crate test;
    use crate::*;
    use rand::{thread_rng, Rng};
    use test::Bencher;

    #[bench]
    fn bench_5_queens(b: &mut Bencher) {
        b.iter(|| ChessBoard::new(5).place_queens());
    }

    #[bench]
    fn bench_10_queens(b: &mut Bencher) {
        b.iter(|| ChessBoard::new(10).place_queens());
    }

    #[bench]
    fn bench_15_queens(b: &mut Bencher) {
        b.iter(|| ChessBoard::new(15).place_queens());
    }

    #[bench]
    fn bench_20_queens(b: &mut Bencher) {
        b.iter(|| ChessBoard::new(20).place_queens());
    }

    #[bench]
    fn bench_20_knapsack(b: &mut Bencher) {
        let knapsack = Knapsack::new(5);
        let items = vec![
            Item {
                value: 10,
                weight: 3,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 10,
                weight: 3,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 10,
                weight: 3,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 10,
                weight: 3,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
        ];

        b.iter(|| Backtracking::fill(&knapsack, items.iter().collect()));
    }

    #[bench]
    fn bench_15_knapsack(b: &mut Bencher) {
        let knapsack = Knapsack::new(5);
        let items = vec![
            Item {
                value: 10,
                weight: 3,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 10,
                weight: 3,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 10,
                weight: 3,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
        ];

        b.iter(|| Backtracking::fill(&knapsack, items.iter().collect()));
    }

    #[bench]
    fn bench_10_knapsack(b: &mut Bencher) {
        let knapsack = Knapsack::new(5);
        let items = vec![
            Item {
                value: 10,
                weight: 3,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 10,
                weight: 3,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
        ];

        b.iter(|| Backtracking::fill(&knapsack, items.iter().collect()));
    }

    #[bench]
    fn bench_5_knapsack(b: &mut Bencher) {
        let knapsack = Knapsack::new(5);
        let items = vec![
            Item {
                value: 10,
                weight: 3,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
        ];

        b.iter(|| Backtracking::fill(&knapsack, items.iter().collect()));
    }

    #[bench]
    fn bench_20_knapsackdp(b: &mut Bencher) {
        let knapsack = Knapsack::new(5);
        let items = vec![
            Item {
                value: 10,
                weight: 3,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 10,
                weight: 3,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 10,
                weight: 3,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 10,
                weight: 3,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
        ];

        b.iter(|| DynamicProgramming::fill(&knapsack, items.iter().collect()));
    }

    #[bench]
    fn bench_15_knapsackdp(b: &mut Bencher) {
        let knapsack = Knapsack::new(5);
        let items = vec![
            Item {
                value: 10,
                weight: 3,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 10,
                weight: 3,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 10,
                weight: 3,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
        ];

        b.iter(|| DynamicProgramming::fill(&knapsack, items.iter().collect()));

    }

    #[bench]
    fn bench_10_knapsackdp(b: &mut Bencher) {
        let knapsack = Knapsack::new(5);
        let items = vec![
            Item {
                value: 10,
                weight: 3,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 10,
                weight: 3,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
        ];

        b.iter(|| DynamicProgramming::fill(&knapsack, items.iter().collect()));
    }

    #[bench]
    fn bench_5_knapsackdp(b: &mut Bencher) {
        let knapsack = Knapsack::new(5);
        let items = vec![
            Item {
                value: 10,
                weight: 3,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
        ];

        b.iter(|| DynamicProgramming::fill(&knapsack, items.iter().collect()));
    }
    #[test]
    fn test_knapsack_5() {
        let knapsack = Knapsack::new(5);
        let items = vec![
            Item {
                value: 10,
                weight: 3,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
        ];

        assert_eq!(Backtracking::fill(&knapsack, items.iter().collect()), 20)
    }

    #[test]
    fn test_knapsackdp_5() {
        let knapsack = Knapsack::new(5);
        let items = vec![
            Item {
                value: 10,
                weight: 3,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
            Item {
                value: 5,
                weight: 1,
            },
        ];

        assert_eq!(DynamicProgramming::fill(&knapsack, items.iter().collect()), 20)
    }

    #[test]
    fn test_4_queens() {
        let mut board = ChessBoard::new(4);
        board.place_queens();

        assert_eq!(
            board.queen_coordinates(),
            vec![(0, 2), (1, 0), (2, 3), (3, 1)]
        )
    }

    #[test]
    fn test_8_queens() {
        let mut board = ChessBoard::new(8);
        board.place_queens();

        assert_eq!(
            board.queen_coordinates(),
            vec![
                (0, 0),
                (1, 6),
                (2, 4),
                (3, 7),
                (4, 1),
                (5, 3),
                (6, 5),
                (7, 2)
            ]
        )
    }

}
