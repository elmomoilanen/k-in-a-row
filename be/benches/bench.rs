#[macro_use]
extern crate criterion;
use criterion::Criterion;

use be::{Board, Bot, Game, Level};

fn init_game(cells: &[i8]) -> Game {
    let board = Board {
        cells: cells.to_vec(),
        p1_mark: -1,
        bot_mark: 1,
        empty_mark: 0,
    };
    match Game::new(board, Level::Normal) {
        Ok(game) => game,
        Err(error_kind) => panic!("Game::new(): {:?}", error_kind),
    }
}

fn bench_bot_player(c: &mut Criterion) {
    let mut group = c.benchmark_group("bot::next_move");
    group.sample_size(10);

    group.bench_function("3x3 3rd move", |b| {
        b.iter(|| Bot::next_move(init_game(&[-1, 0, 0, 0, 1, 0, 0, 0, 0])))
    });

    group.bench_function("3x3 5th move to win", |b| {
        b.iter(|| Bot::next_move(init_game(&[-1, -1, 1, 0, 1, 0, 0, 0, 0])))
    });

    group.bench_function("5x5 8th move", |b| {
        b.iter(|| {
            Bot::next_move(init_game(&[
                0, 0, 0, 0, 0, 0, 1, 0, -1, 0, 0, -1, -1, 1, 0, 0, 1, -1, 0, 0, 0, 0, 0, 0, 0,
            ]))
        })
    });

    group.bench_function("5x5 10th move", |b| {
        b.iter(|| {
            Bot::next_move(init_game(&[
                0, 0, 0, 0, -1, 0, 1, -1, 0, 0, 0, -1, 1, 1, 0, 0, -1, 1, -1, 0, 0, 0, 0, 0, 0,
            ]))
        })
    });

    group.bench_function("15x15 40th move", |b| {
        b.iter(|| {
            Bot::next_move(init_game(&[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0,
                0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                1, 0, -1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 1, 1, -1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, -1, -1, -1, -1, 1, 0, -1, 0, 0, 0, 0,
                0, 0, 0, 0, 0, -1, 1, -1, -1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, -1, 0, 1, 1, -1, -1,
                0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
                0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
            ]))
        })
    });

    group.bench_function("15x15 90th move", |b| {
        b.iter(|| {
            Bot::next_move(init_game(&[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, -1, 0, 0,
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, 1, -1, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 1, -1,
                0, 1, 0, 1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, 1, 0, -1, 0, -1, 1, 0, 0, 0, -1, 0,
                0, 1, 1, -1, -1, 1, 1, 0, -1, 0, 0, 0, 0, 0, 1, 0, -1, -1, 1, 1, -1, 0, -1, 1, -1,
                0, 0, 0, 0, 0, 1, 0, -1, 1, -1, 1, -1, 0, 0, 1, 0, 0, 0, 0, 0, -1, 1, -1, 1, 1, -1,
                -1, 1, 0, 1, 0, 0, 0, 1, 1, -1, 0, 1, -1, 1, 0, 1, 0, 0, 0, 0, 1, -1, -1, -1, -1,
                1, 0, -1, 1, -1, 0, 0, 0, 1, 0, 0, 0, -1, 1, -1, -1, 1, 1, -1, 0, 0, 0, 0, 0, 0, 0,
                1, -1, 0, 1, 1, -1, -1, 1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, -1, 0, -1, 0, 0,
                0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
            ]))
        })
    });

    group.finish();
}

criterion_group!(benches, bench_bot_player);
criterion_main!(benches);
