use std::{
    fmt::{Debug, Display},
    fs::File,
    io::{BufReader, Read},
    isize,
    ops::{Deref, DerefMut},
};

#[derive(PartialEq, Eq, Clone, Copy)]
enum BoardCell {
    Empty,
    BoxLeft,
    BoxRight,
    Wall,
    Robot,
}

struct Board(Vec<Vec<BoardCell>>);

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            for cell in row {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Deref for Board {
    type Target = Vec<Vec<BoardCell>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for BoardCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            BoardCell::Empty => '.',
            BoardCell::BoxLeft => '[',
            BoardCell::BoxRight => ']',
            BoardCell::Wall => '#',
            BoardCell::Robot => '@',
        };
        write!(f, "{}", c)
    }
}

// All locations are in y, x form.

fn find_robot(board: &Board) -> (usize, usize) {
    for (row_num, row) in board.iter().enumerate() {
        for (col_num, board_cell) in row.iter().enumerate() {
            if *board_cell == BoardCell::Robot {
                return (row_num, col_num);
            }
        }
    }
    unreachable!()
}

fn gps_sum(board: &Board) -> u64 {
    let mut sum = 0;
    for (row_num, row) in board.iter().enumerate() {
        for (col_num, cell) in row.iter().enumerate() {
            if *cell == BoardCell::BoxLeft {
                // Position depends on position of left.
                sum += 100 * row_num + col_num;
            }
        }
    }
    sum as u64
}

fn add_vector((y_pos, x_pos): (usize, usize), (y_dir, x_dir): (isize, isize)) -> (usize, usize) {
    (
        (y_pos as isize + y_dir) as usize,
        (x_pos as isize + x_dir) as usize,
    )
}

fn find_box_half_positions(board: &Board, (y, x): (usize, usize)) -> [(usize, usize); 2] {
    match board[y][x] {
        BoardCell::BoxLeft if board[y][x + 1] == BoardCell::BoxRight => [(y, x), (y, x + 1)],
        BoardCell::BoxLeft => panic!("Invalid box"),
        BoardCell::BoxRight if board[y][x - 1] == BoardCell::BoxLeft => [(y, x - 1), (y, x)],
        BoardCell::BoxRight => panic!("Invalid box"),
        _ => panic!("Not a box"),
    }
}

fn find_push_targets(
    board: &Board,
    box_half_position: (usize, usize),
    direction: (isize, isize),
) -> Box<[(usize, usize)]> {
    let [left_box_position, right_box_position] = find_box_half_positions(board, box_half_position);
    match direction.1 {
        0 => Box::new([
            add_vector(left_box_position, direction),
            add_vector(right_box_position, direction),
        ]),
        1 => Box::new([add_vector(right_box_position, direction)]),
        -1 => Box::new([add_vector(left_box_position, direction)]),
        _ => panic!("Unsupported horizontal direction."),
    }
}

fn is_pushable(board: &Board, position: (usize, usize), direction: (isize, isize)) -> bool {
    match board[position.0][position.1] {
        BoardCell::Empty => true,
        BoardCell::BoxLeft | BoardCell::BoxRight => find_push_targets(board, position, direction)
            .iter()
            .all(|target_position| is_pushable(board, *target_position, direction)),
        BoardCell::Wall => false,
        BoardCell::Robot => panic!("Another robot?!"),
    }
}

// Only call after verifying this position is pushable with `is_pushable`
fn push(board: &mut Board, position: (usize, usize), direction: (isize, isize)) {
    match board[position.0][position.1] {
        BoardCell::Empty => {}
        BoardCell::BoxLeft | BoardCell::BoxRight => {
            for target_position in find_push_targets(board, position, direction) {
                push(board, target_position, direction);
            }
            let [box_left_position, box_right_position] = find_box_half_positions(board, position);
            // Remove the box from the old position:
            let (box_left_y, box_left_x) = box_left_position;
            board[box_left_y][box_left_x] = BoardCell::Empty;
            let (box_right_y, box_right_x) = box_right_position;
            board[box_right_y][box_right_x] = BoardCell::Empty;

            // Place the box in the new position:
            let (new_box_left_y, new_box_left_x) = add_vector(box_left_position, direction);
            board[new_box_left_y][new_box_left_x] = BoardCell::BoxLeft;
            let (new_box_right_y, new_box_right_x) = add_vector(box_right_position, direction);
            board[new_box_right_y][new_box_right_x] = BoardCell::BoxRight;
        }
        _ => panic!("Not pushable!"),
    }
}

fn make_move(board: &mut Board, move_: char) {
    let robot_position = find_robot(board);
    let direction = match move_ {
        '<' => (0, -1),
        '>' => (0, 1),
        '^' => (-1, 0),
        'v' => (1, 0),
        _ => panic!("Invalid direction"),
    };

    let position_in_front_of_robot = add_vector(robot_position, direction);

    if is_pushable(board, position_in_front_of_robot, direction) {
        push(board, position_in_front_of_robot, direction);
        let (in_front_of_robot_y, in_front_of_robot_x) = position_in_front_of_robot;
        let (robot_y, robot_x) = robot_position;
        board[in_front_of_robot_y][in_front_of_robot_x] = BoardCell::Robot;
        board[robot_y][robot_x] = BoardCell::Empty;
    }
}

fn main() {
    let mut input_file = BufReader::new(File::open("input-part_2.txt").unwrap());
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let (initial_board_string, moves_string) =
        input_string.split_at(input_string.find("\n\n").unwrap());
    let moves_string = &moves_string[2..]; // Chop off initial two newlines
    dbg!(initial_board_string, moves_string);

    let mut board = Vec::new();
    for line in initial_board_string.lines() {
        let mut new_board_line = Vec::new();
        for board_cell_byte in line.as_bytes() {
            let board_cell = match board_cell_byte {
                b'#' => BoardCell::Wall,
                b'@' => BoardCell::Robot,
                b'[' => BoardCell::BoxLeft,
                b']' => BoardCell::BoxRight,
                b'.' => BoardCell::Empty,
                _ => panic!("Unknown board square!"),
            };
            new_board_line.push(board_cell);
        }
        board.push(new_board_line);
    }

    let mut board = Board(board);

    dbg!(&board);
    dbg!(find_robot(&board));

    for curr_move in moves_string.chars() {
        if curr_move == '\n' {
            continue;
        }
        make_move(&mut board, curr_move);
        // println!("{:?}", &board);
    }

    println!("Final GPS Sum is: {}", gps_sum(&board));
}
