pub fn cells(location: tbp::PieceLocation) -> [(i32, i32); 4] {
    let mut cells = piece_cells(location.kind);
    for cell in &mut cells {
        *cell = rotate(location.orientation, *cell);
        cell.0 += location.x;
        cell.1 += location.y;
    }
    cells
}

pub fn piece_cells(piece: tbp::Piece) -> [(i32, i32); 4] {
    match piece {
        tbp::Piece::I => [(-1, 0), (0, 0), (1, 0), (2, 0)],
        tbp::Piece::O => [(0, 0), (0, 1), (1, 0), (1, 1)],
        tbp::Piece::T => [(-1, 0), (0, 0), (1, 0), (0, 1)],
        tbp::Piece::L => [(-1, 0), (0, 0), (1, 0), (1, 1)],
        tbp::Piece::J => [(-1, 0), (0, 0), (1, 0), (-1, 1)],
        tbp::Piece::S => [(-1, 0), (0, 0), (0, 1), (1, 1)],
        tbp::Piece::Z => [(-1, 1), (0, 1), (0, 0), (1, 0)],
    }
}

pub fn rotate(orientation: tbp::Orientation, cell: (i32, i32)) -> (i32, i32) {
    match orientation {
        tbp::Orientation::North => cell,
        tbp::Orientation::West => (-cell.1, cell.0),
        tbp::Orientation::South => (-cell.0, -cell.1),
        tbp::Orientation::East => (cell.1, -cell.0),
    }
}

pub fn blocked(board: &[[bool; 10]; 40], piece: tbp::PieceLocation) -> bool {
    for &(x, y) in &cells(piece) {
        if x < 0 || y < 0 || x >= 10 || y >= 40 {
            return true;
        }
        if board[y as usize][x as usize] {
            return true;
        }
    }
    false
}

pub fn place_piece(board: &mut [[bool; 10]; 40], piece: tbp::PieceLocation) {
    for &(x, y) in &cells(piece) {
        board[y as usize][x as usize] = true;
    }
}

pub fn collapse_lines(board: &mut [[bool; 10]; 40]) -> usize {
    let mut lines_cleared = 0;
    for y in (0..40).rev() {
        if board[y] == [true; 10] {
            lines_cleared += 1;
            for i in y + 1..40 {
                board[i - 1] = board[i];
            }
            board[39] = [false; 10];
        }
    }
    lines_cleared
}
