use crate::mechanics::*;

pub fn suggest(board: &[[bool; 10]; 40], piece: tbp::Piece) -> Vec<tbp::Move> {
    let mut moves = vec![];
    for &orientation in sensible_orientations(piece) {
        for x in 0..10 {
            let mut piece = tbp::PieceLocation {
                kind: piece,
                orientation,
                x,
                y: 38,
            };
            if blocked(board, piece) {
                continue;
            }

            while !blocked(board, piece) {
                piece.y -= 1;
            }
            piece.y += 1;

            let mut board = *board;
            place_piece(&mut board, piece);

            let piece_cells_eliminated = cells(piece)
                .iter()
                .filter(|&&(_, y)| board[y as usize] == [true; 10])
                .count();
            let lines_cleared = collapse_lines(&mut board);

            let (low, high) = piece_span(piece);

            let landing_height = low + high;
            let eroded_piece_cells_metric = (lines_cleared * piece_cells_eliminated) as i32;
            let row_transitions = row_transitions(&board);
            let column_transitions = column_transitions(&board);
            let buried_holes = buried_holes(&board);
            let wells = wells(&board);

            let score = 2 * eroded_piece_cells_metric
                - landing_height
                - 2 * row_transitions
                - 2 * column_transitions
                - 8 * buried_holes
                - 2 * wells;

            moves.push((piece, score));
        }
    }

    moves.sort_by_key(|&(_, score)| std::cmp::Reverse(score));

    moves
        .into_iter()
        .map(|(piece, _)| tbp::Move {
            location: piece,
            spin: tbp::Spin::None,
        })
        .collect()
}

fn row_transitions(board: &[[bool; 10]; 40]) -> i32 {
    board
        .iter()
        .map(|row| {
            let mut previous = true;
            let mut count = 0;
            for &cell in row {
                if cell != previous {
                    count += 1;
                }
                previous = cell;
            }
            if !previous {
                count += 1;
            }
            count
        })
        .sum()
}

fn column_transitions(board: &[[bool; 10]; 40]) -> i32 {
    let mut count = 0;
    let mut previous = [true; 10];
    for row in board {
        count += (0..10).filter(|&x| row[x] != previous[x]).count();
        previous = *row;
    }
    count as i32
}

fn buried_holes(board: &[[bool; 10]; 40]) -> i32 {
    let mut count = 0;
    let mut is_column_covered = [false; 10];
    for row in board.iter().rev() {
        for x in 0..10 {
            if is_column_covered[x] && !row[x] {
                count += 1;
            }
            is_column_covered[x] |= row[x];
        }
    }
    count
}

fn wells(board: &[[bool; 10]; 40]) -> i32 {
    let mut score = 0;
    for y in 0..40 {
        for x in 0..10 {
            let left = x == 0 || board[y][x - 1];
            let right = x == 9 || board[y][x + 1];
            if left && right && !board[y][x] {
                // Count the number of empty cells below, including the well cell
                for y in (0..=y).rev() {
                    if board[y][x] {
                        break;
                    }
                    score += 1;
                }
            }
        }
    }
    score
}

fn sensible_orientations(piece: tbp::Piece) -> &'static [tbp::Orientation] {
    match piece {
        tbp::Piece::I | tbp::Piece::S | tbp::Piece::Z => {
            &[tbp::Orientation::North, tbp::Orientation::West]
        }
        tbp::Piece::O => &[tbp::Orientation::North],
        tbp::Piece::T | tbp::Piece::L | tbp::Piece::J => &[
            tbp::Orientation::North,
            tbp::Orientation::East,
            tbp::Orientation::South,
            tbp::Orientation::West,
        ],
    }
}

fn piece_span(piece: tbp::PieceLocation) -> (i32, i32) {
    let mut min = 40;
    let mut max = 0;
    for &(_, y) in &cells(piece) {
        min = min.min(y);
        max = max.max(y);
    }
    (min, max)
}
