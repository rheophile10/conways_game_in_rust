use std::{
    io::{stdout},
    thread::sleep,
    time::Duration,
    iter::FromIterator,
};
use crossterm::{
    style::{Color, PrintStyledContent, style, 
        ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result, cursor
};

const HEIGHT:usize = 20;
const WIDTH:usize = 20;
const TURNS:u8= 20;

fn main() -> Result<()> {
    let mut stdout = stdout();
    
    let mut environment = [[' '; HEIGHT]; WIDTH];
    environment[0][0] = '█';
    let mut new_environment = [[' '; HEIGHT]; WIDTH];
    stdout.execute(cursor::RestorePosition)?;

    for _ in 0..=TURNS-2 {
        for row in 0..=HEIGHT-1 {
            for col in 0..=WIDTH-1{
                if environment[row][col] == '█' {
                    new_environment[row][col] = ' ';
                    new_environment[row+1][col+1] = '█';
                }
            }
            let mut row_state = String::from_iter(new_environment[row]);
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