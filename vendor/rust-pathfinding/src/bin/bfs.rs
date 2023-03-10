use std::path::Path;

use pathfinding::prelude::bfs;
use rust_pathfinding::{Board, Pos};

fn main() {
    let board = Board::new(vec!["1111X", "11111", "X1111", "X1111", "11111"], false);
    let start = Pos(0, 1);
    let goal = Pos(4, 1);
    let result = bfs(
        &start,
        |p| {
            board
                .get_successors(p)
                .iter()
                .map(|successor| successor.pos)
                .collect::<Vec<_>>()
        },
        |p| *p == goal,
    );
    let result = result.expect("No path found");
    board.draw_to_image(Path::new("bfs.png"), Some(&result));
}
