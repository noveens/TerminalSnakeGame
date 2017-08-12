extern crate rand;
extern crate rustbox;

use rand::Rng;
use std::error::Error;
use std::default::Default;
use std::time::Instant;

use rustbox::{Color, RustBox};
use rustbox::Key;

fn main() {
    let tb = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let mut board = [
        [0; 10]; 10
     ];

     board[3][1] = 1;
     let mut cell_i: u32 = 3;
     let mut cell_j: u32 = 1;

     board[5][5] = 2;
     let mut gift_i: u32 = 5;
     let mut gift_j: u32 = 5;

     let mut score = 0;
     let now = Instant::now();

     let mut time_up = 0;

    loop {
      tb.clear();

      tb.print(55, 8, rustbox::RB_BOLD, Color::Red, Color::Black, "X---------------------X");
      tb.print(55, 19, rustbox::RB_BOLD, Color::Red, Color::Black, "X---------------------X");

      for temp in 9..19 {
        tb.print(55, temp, rustbox::RB_BOLD, Color::Red, Color::Black, "| ");
      }

      for temp in 9..19 {
        tb.print(77, temp, rustbox::RB_BOLD, Color::Red, Color::Black, "|");
      }

      let mut j: usize = 8;
      let mut flag = 0;

      for row in board.as_ref() {
          let mut i: usize = 55;
          j = j + 1;
          for cell in row.as_ref() {
            i = i + 2;
            if  *cell == 0 {
              tb.print(i, j, rustbox::RB_BOLD, Color::White, Color::Black, "  ");
            } else if *cell == 1 {
              tb.print(i, j, rustbox::RB_BOLD, Color::White, Color::Black, "☻ ");
            } else if *cell == 2 {
              flag = 1;
              tb.print(i, j, rustbox::RB_BOLD, Color::Green, Color::Black, "■ ");
            }
          }
      }

      if flag == 0 {
          let rand_i = rand::thread_rng().gen_range(0, 9);
          let rand_j = rand::thread_rng().gen_range(0, 9);
          board[rand_i as usize][rand_j as usize] = 2;
          gift_i = rand_i;
          gift_j = rand_j;
      }

      let time = now.elapsed().as_secs();

      let sss: &str = &(score).to_string();
      tb.print(61, 25, rustbox::RB_BOLD, Color::White, Color::Black, "Your Score: ");
      tb.print(73, 25, rustbox::RB_BOLD, Color::White, Color::Black, sss);

      let ss: &str = &(20 - time).to_string();
      tb.print(57, 27, rustbox::RB_BOLD, Color::White, Color::Black, "Time Remaining: ");
      if 20-time > 5 {
        tb.print(73, 27, rustbox::RB_BOLD, Color::White, Color::Black, ss);
      } else {
        tb.print(73, 27, rustbox::RB_BOLD, Color::Red, Color::Black, ss);
      }

      if 20-time >= 10 {
        tb.print(75, 27, rustbox::RB_BOLD, Color::White, Color::Black, " sec.");
      } else if 20-time > 5 {
        tb.print(74, 27, rustbox::RB_BOLD, Color::White, Color::Black, " sec.");
      } else {
        tb.print(74, 27, rustbox::RB_BOLD, Color::Red, Color::Black, " sec.");
      }

      if time >= 20 {
        time_up = 1;
        break;
      }

      tb.present();

      match tb.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { break; }

                    Key::Char('w') => {
                        if cell_i == 0 {
                            time_up = 2;
                            break;
                        }
                        if cell_i - 1 == gift_i && cell_j == gift_j {
                            score = score + 1;
                            let rand_i = rand::thread_rng().gen_range(0, 9);
                            let rand_j = rand::thread_rng().gen_range(0, 9);
                            board[rand_i as usize][rand_j as usize] = 2;
                            gift_i = rand_i;
                            gift_j = rand_j;
                        }
                        board[cell_i as usize][cell_j as usize] = 0;
                        cell_i = cell_i - 1;
                        board[cell_i as usize][cell_j as usize] = 1;
                    }

                    Key::Char('a') => {
                        if cell_j == 0 {
                            time_up = 2;
                            break;
                        }
                        if cell_j - 1 == gift_j && cell_i == gift_i {
                            score = score + 1;
                            let rand_i = rand::thread_rng().gen_range(0, 9);
                            let rand_j = rand::thread_rng().gen_range(0, 9);
                            board[rand_i as usize][rand_j as usize] = 2;
                            gift_i = rand_i;
                            gift_j = rand_j;
                        }
                        board[cell_i as usize][cell_j as usize] = 0;
                        cell_j = cell_j - 1;
                        board[cell_i as usize][cell_j as usize] = 1;
                    }

                    Key::Char('s') => {
                        if cell_i == 9 {
                            time_up = 2;
                            break;
                        }
                        if cell_i + 1 == gift_i && cell_j == gift_j {
                            score = score + 1;
                            let rand_i = rand::thread_rng().gen_range(0, 9);
                            let rand_j = rand::thread_rng().gen_range(0, 9);
                            board[rand_i as usize][rand_j as usize] = 2;
                            gift_i = rand_i;
                            gift_j = rand_j;
                        }
                        board[cell_i as usize][cell_j as usize] = 0;
                        cell_i = cell_i + 1;
                        board[cell_i as usize][cell_j as usize] = 1;
                    }

                    Key::Char('d') => {
                        if cell_j == 9 {
                            time_up = 2;
                            break;
                        }
                        if cell_j + 1 == gift_j && cell_i == gift_i {
                            score = score + 1;
                            let rand_i = rand::thread_rng().gen_range(0, 9);
                            let rand_j = rand::thread_rng().gen_range(0, 9);
                            board[rand_i as usize][rand_j as usize] = 2;
                            gift_i = rand_i;
                            gift_j = rand_j;
                        }
                        board[cell_i as usize][cell_j as usize] = 0;
                        cell_j = cell_j + 1;
                        board[cell_i as usize][cell_j as usize] = 1;
                    }
                    _ => { }
                }
            },
            Err(e) => panic!("{}", e.description()),
            _ => { }
        }
    }

    let now2 = Instant::now();

    loop {
        let time = now2.elapsed().as_secs();
        if time >= 3 {
            break;
         }

        let ss: &str = &(score).to_string();

        tb.clear();

        if time_up == 1 {
            tb.print(58, 16, rustbox::RB_BOLD, Color::White, Color::Black, "Tera time khatam be :)");
        } else if time_up == 2 {
            tb.print(56, 16, rustbox::RB_BOLD, Color::White, Color::Black, "Chiii.. You hit the wall!");
        } else {
            tb.print(58, 16, rustbox::RB_BOLD, Color::White, Color::Black, "Why did You Quit? :(");
        }

        tb.print(63, 18, rustbox::RB_BOLD, Color::White, Color::Black, "Well Tried!");

        tb.print(62, 20, rustbox::RB_BOLD, Color::White, Color::Black, "Your Score: ");
        tb.print(74, 20, rustbox::RB_BOLD, Color::Green, Color::Black, ss);
        tb.present();
    }
}
