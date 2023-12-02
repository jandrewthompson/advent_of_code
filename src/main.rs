use std::fs;
use std::collections::HashMap;

fn main() {

    let file_path = "day1_input.txt";

    let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
    
    let _nums = HashMap::from([
                              ("zero",0),
                              ("one",1),
                              ("two",2),
                              ("three",3),
                              ("four",4),
                              ("five",5),
                              ("six",6),
                              ("seven",7),
                              ("eight",8),
                              ("nine",9),
    ]);

    let mut total :u32 = 0;
    for line in contents.lines() {
        let mut l :u32 = 0;
        let mut r :u32 = 0;
        'outer_l:for (_i, c) in line.chars().enumerate() {
           if c.is_numeric() {
               l = c.to_digit(10).unwrap();
               break;
           }

           //println!("{}", line[..=_i].to_string());
           for (k,v) in &_nums {
               if line[..=_i].contains(k) {
                   l = *v;
                   //println!("found l: {}", l);
                   break 'outer_l;
               }
           }

        }
        'outer_r:for (_i, c) in line.chars().rev().enumerate() {
           //println!("s: {}", line[(line.len()-1-_i)..].to_string());
           if c.is_numeric() {
               r = c.to_digit(10).unwrap();
               break;
           }
           
           for (k,v) in &_nums {
               if line[(line.len()-1-_i)..].contains(k) {
                   r = *v;
                   //println!("found r: {}", r);
                   break 'outer_r;
               }
           }
        }

        total += (l * 10) + r;
        println!("line: {}. l: {}, r: {}", (l * 10) + r, l, r);
    }
    println!("total: {}", total);
}
