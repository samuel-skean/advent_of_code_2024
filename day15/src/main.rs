use std::{fmt::Display, fs::File, io::{BufReader, Read}};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum BoardCell {
    Empty,
    Box,
    Wall,
    Robot,
}
impl Display for BoardCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("Make this print its input symbol for debugging.");
    }
}

// All locations are in y, x form.

fn find_robot(board: &Vec<Vec<BoardCell>>) -> (usize, usize) {
    for (row_num, row) in board.iter().enumerate() {
        for (col_num, board_cell) in row.iter().enumerate() {
            if *board_cell == BoardCell::Robot {
                return (row_num, col_num);
            }
        }
    }
    unreachable!()
}

fn make_move(board: &mut Vec<Vec<BoardCell>>, move_: u8) {
    let (robot_x, robot_y) = find_robot(board);
    let direction = match move_ {
        b'<' => (0, -1),
        b'>' => (0, 1),
        b'^' => (-1, 0),
        b'v' => (1, 0),
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
        current_x += (current_x as isize + direction.1) as usize;
        current_cell = board[current_y][current_x];
    }

    match current_cell {
        BoardCell::Empty => {
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
    let mut input_file = BufReader::new(File::open("simple_test_input.txt").unwrap());
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

    dbg!(&board);
    dbg!(find_robot(&mut board));

    for &curr_move in moves_string.as_bytes() {
        make_move(&mut board, curr_move);
        println!("Board: {:?}", &board);
    }


}
