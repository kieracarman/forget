use std::io::{stdin, stdout, Read, Write};
use std::time::Duration;
use std::thread::sleep;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    stdout().flush().unwrap();
}

//fn blink_cursor() {
//    let t = 250;
//    
//    loop {
//        // cursor off
//        sleep(Duration::from_millis(t));
//        print!("\x1B[?25l");
//        stdout().flush().unwrap();
//
//        // cursor on
//        sleep(Duration::from_millis(t));
//        print!("\x1B[?25h");
//        stdout().flush().unwrap();
//    }
//
//    // make sure the cursor is on
//    print!("\x1B[?25h");
//    stdout().flush().unwrap();
//}

fn wait() {
    print!(" (press enter) ");
    stdout().flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn pause(n: u64) {
    sleep(Duration::from_millis(n*1000));
}

fn type_string(string: &str) {
   let t = 20;
   let mut prev_char = ' ';

   for c in string.chars() {
       if prev_char == '.' || prev_char == '?' {
           sleep(Duration::from_millis(2500));
       }
       print!("{}", c);
       stdout().flush().unwrap();
       sleep(Duration::from_millis(t));
       prev_char = c;
   }
}

fn skip_line(n: i32) {
    let t = 50;

    for _ in 0..n {
        println!("");
        sleep(Duration::from_millis(t));
    }
}

fn main() {
    clear_screen();
    pause(3);
    type_string("You need to dance. You need to shake your hips");
    pause(2);
    skip_line(5);
    pause(2);
    type_string("It felt good when you did that");
    pause(2);
    type_string("\nYou used to love it");
    pause(1);
    type_string("\nWhat happened?");
    pause(4);
    skip_line(8);
    type_string("Remember dancing in the club? That was the closest you\'ve felt to that feeling since you turned 8.");
    pause(5);
    clear_screen();
    pause(4);
    type_string("Remember her taking you by the hand and leading you to her house?");
    pause(4);
    skip_line(2);
    pause(1);
    type_string("Upstairs? To the guest room?");
    pause(4);
    skip_line(12);
    pause(3);
    type_string("Remember when her hips moved against you");
    pause(4);
    type_string("\nLike she meant it");
    pause(4);
    skip_line(8);
    type_string("What\'s hap-");
    clear_screen();
    type_string("Murky.");
    pause(1);
    type_string("\nSo dark and so deep. Inky haze");
    pause(3);
    type_string("\nObscured");
    pause(1);
    type_string("\nUnderwater");
    pause(1);
    type_string("\nUnder alcohol");
    pause(4);
    skip_line(4);
    pause(4);
    type_string("...");
    pause(5);
    type_string("\nPoof.");
    pause(2);
    skip_line(2);
    pause(1);
    type_string("Goodnight");
    pause(3);
    skip_line(8);
    pause(4);
    type_string("Never happened");
    pause(4);
    skip_line(2);
    pause(2);
    type_string("Paper shredder");
    pause(5);
    skip_line(30);
    pause(3);
    type_string("Forget.");
    pause(4);
    wait();
    clear_screen();
}
