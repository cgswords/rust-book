extern crate lmc;

use lmc::machine::datatypes::Machine;
use lmc::machine::printer::print_machine;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io;
use std::path::Path;

fn main() {
  println!("Please input an instruction file name.");

  let mut fname = String::new();
  io::stdin().read_line(&mut fname)
             .expect("Failed to read line");
  fname = fname.trim().to_string();
  let path = Path::new(&fname);
  
  let f = File::open(path).expect("No instructions found");
  let file = BufReader::new(&f);
  let mut instrs: Vec<u32> = file.lines()
                                 .into_iter()
                                 .map(|n| { n.expect("Invalid").trim().parse().expect("Invalid instruction") })
                                 .collect(); 
  
  instrs.resize(100,0); 
  
  println!("");

  let mut mach = Machine::new(&instrs, &vec![010,020]);
  println!("Starting Configuration");
  print_machine(&mut mach);

  mach.run();
  
  println!("");
  println!("Ending Configuration");
  print_machine(&mut mach);
}
