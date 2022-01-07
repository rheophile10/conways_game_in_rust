use std::{
    io::{stdout},
    thread::sleep,
    time::Duration,
};
use crossterm::{
    style::{Color, PrintStyledContent, style, 
        ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result, cursor
};

//const HEIGHT:usize = 20;
//const WIDTH:usize = 20;
const TURNS:u8= 20;

#[derive(Clone, Copy)]
struct Environment {
    data: [[bool; 50]; 50],
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            data: [[false; 50]; 50]
        }
    }

}

fn load_live_cells(live_cells: [(usize, usize); 5], environment: &mut Environment) {
    for cell in live_cells {
        environment.data[cell.0][cell.1] = true;
    }
}

fn get_live_neigbour_count(coords: (usize, usize), environment: &Environment) -> u8 {
    let x = coords.0;
    let y = coords.1;
    let mut neighbour_coords = Vec::new();
    if x == 0 {
        neighbour_coords.push((x+1, y));
        if y != 0 {
            neighbour_coords.push((x+1, y-1));
            neighbour_coords.push((x, y-1));
        }
        if y != 49 {
            neighbour_coords.push((x+1, y+1));
            neighbour_coords.push((x, y+1));
        }             
    } else if x == 49 {
        neighbour_coords.push((x-1, y));
        if y != 0 {
            neighbour_coords.push((x-1, y-1));
            neighbour_coords.push((x, y-1));
        }
        if y != 49 {
            neighbour_coords.push((x-1, y+1));
            neighbour_coords.push((x, y+1));
        }       
    } else if y == 0 {
        neighbour_coords.push((x, y+1));
        if x != 0 {
            neighbour_coords.push((x-1, y+1));
            neighbour_coords.push((x-1, y));
        }
        if x != 49 {
            neighbour_coords.push((x+1, y+1));
            neighbour_coords.push((x+1, y));
        }    
    } else if y == 49 {
        neighbour_coords.push((x, y-1));
        if x != 0 {
            neighbour_coords.push((x-1, y-1));
            neighbour_coords.push((x-1, y));
        }
        if x != 49 {
            neighbour_coords.push((x+1, y-1));
            neighbour_coords.push((x+1, y));
        }  
    } else {
        neighbour_coords.push((x, y-1));
        neighbour_coords.push((x-1, y-1));
        neighbour_coords.push((x+1, y-1));
        neighbour_coords.push((x+1, y));
        neighbour_coords.push((x-1, y));
        neighbour_coords.push((x, y+1));
        neighbour_coords.push((x-1, y+1));
        neighbour_coords.push((x+1, y+1));
    }
    let mut count = 0;
    for coord in neighbour_coords {
        if environment.data[coord.0][coord.1] {
            count = count + 1;
        }
    }
    count
}

fn row_to_string(row: usize, environment: &Environment) -> String {
    let mut row_string = String::from("");
    for col in 0..50 {
        match environment.data[row][col] {
            true => {
                row_string.push_str("█");
            }
            false => {
                row_string.push_str(" ");
            }

        }
    }
    row_string
}

fn main() -> Result<()> {
    let mut stdout = stdout();
    stdout.execute(cursor::RestorePosition)?;

    let mut environment = Environment::new();
    let mut new_environment = Environment::new();
    load_live_cells([(0,0),(1,0),(0,1),(1,2),(2,1)], &mut environment);
    
    for _ in 0..TURNS {
        for row in 0..50 {
            for col in 0..50 {
                let neighbours = get_live_neigbour_count((row,col), &environment);
                if neighbours > 2 {
                    new_environment.data[row][col] = true;
                } else if neighbours > 1 && environment.data[row][col] {
                    new_environment.data[row][col] = true;
                } else {
                    new_environment.data[row][col] = false;
                }
            }


            let mut row_state = row_to_string(row, &new_environment);
            row_state.push_str("\n");
            let stylized_row = style(row_state);
            stdout
                .execute(SetForegroundColor(Color::Red))?
                .execute(SetBackgroundColor(Color::Blue))?
                .execute(PrintStyledContent(stylized_row))?
                .execute(ResetColor)?;
        }
        sleep(Duration::from_millis(100));
        stdout.execute(cursor::RestorePosition)?;
        environment = new_environment;

    }
    Ok(())
}