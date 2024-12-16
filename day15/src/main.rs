use std::{fmt::Display, fmt::Debug, fs::File, io::{BufReader, Read}, ops::{Deref, DerefMut}};

#[derive(PartialEq, Eq, Clone, Copy)]
enum BoardCell {
    Empty,
    Box,
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
            BoardCell::Box => 'O',
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
            if *cell == BoardCell::Box {
                sum += 100 * row_num + col_num;
            }
        }
    }
    sum as u64
}

fn make_move(board: &mut Board, move_: char) {
    let (robot_y, robot_x) = find_robot(board);
    let direction = match move_ {
        '<' => (0, -1),
        '>' => (0, 1),
        '^' => (-1, 0),
        'v' => (1, 0),
        _ => panic!("Invalid direction"),
    };

    let (in_front_of_robot_y, in_front_of_robot_x) = ((robot_y as isize + direction.0) as usize, (robot_x as isize + direction.1) as usize);
    let (mut current_y, mut current_x) = (in_front_of_robot_y, in_front_of_robot_x);
    let mut current_cell =  board[current_y][current_x];
    
    if current_cell == BoardCell::Empty {
        // Remove the robot from the board:
        board[robot_y][robot_x] = BoardCell::Empty;
        // Just move one.
        board[current_y][current_x] = BoardCell::Robot;
        return;
    }

    while current_cell == BoardCell::Box {
        current_y = (current_y as isize + direction.0) as usize;
        current_x = (current_x as isize + direction.1) as usize;
        current_cell = board[current_y][current_x];
    }

    match current_cell {
        BoardCell::Empty => {
            board[robot_y][robot_x] = BoardCell::Empty;
            // Move the boxes.
            board[current_y][current_x] = BoardCell::Box;
            board[in_front_of_robot_y][in_front_of_robot_x] = BoardCell::Robot;
        }
        BoardCell::Box => {
            panic!("We should only get here if there were no more boxes in the way.");
        }
        BoardCell::Wall => {
            // Don't do anything, the robot couldn't move.
            return;
        }
        BoardCell::Robot => {
            panic!("Another robot?!");
        }
    }

}

fn main() {
    let mut input_file = BufReader::new(File::open("input.txt").unwrap());
    let mut input_string = String::new();
    input_file.read_to_string(&mut input_string).unwrap();

    let (initial_board_string, moves_string) = input_string.split_at(input_string.find("\n\n").unwrap());
    let moves_string = &moves_string[2..]; // Chop off initial two newlines
    dbg!(initial_board_string, moves_string);

    let mut board = Vec::new();
    for line in initial_board_string.lines() {
        let mut new_board_line = Vec::new();
        for board_cell_byte in line.as_bytes() {
            let board_cell = match board_cell_byte {
                b'#' => BoardCell::Wall,
                b'@' => BoardCell::Robot,
                b'O' => BoardCell::Box,
                b'.' => BoardCell::Empty,
                _ => panic!("Unknown board square!")
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
