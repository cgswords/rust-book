
use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

struct Table {
      forks: Vec<Mutex<()>>,
}

struct Philosopher {
  name:  String,
  left:  usize, // usize is the vector index type
  right: usize,
}

impl Philosopher {
  fn new(name: &str, left: usize, right: usize) -> Philosopher 
  {
    Philosopher {
      name:  name.to_string(),
      left:  left,
      right: right,
    }
  }

 fn eat(&self, table: &Table) {
   let _left = table.forks[self.left].lock().unwrap();
   thread::sleep(Duration::from_millis(150));
   let _right = table.forks[self.right].lock().unwrap();

   // By using the underscore, we tell Rust that we don't plan to
   // use the value. That way, it won't raise a warning about the
   // unused value.
   // Also, unlocking happens automagically when _left and _right
   // go out of scope.

   println!("{} is eating.", self.name);
   thread::sleep(Duration::from_millis(1000));
   println!("{} is done eating.", self.name);
 }
}

fn main() {
 
  // Arc : Atomic Reference Count; makes it so we can
  // share the table across multiple threads.
  let table = Arc::new(Table { forks: vec![
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
    Mutex::new(()),
  ]});

  let philosophers = vec![
    Philosopher::new("Judith Butler"  , 0, 1),
    Philosopher::new("Gilles Deleuze" , 1, 2),
    Philosopher::new("Karl Marx"      , 2, 3),
    Philosopher::new("Emma Goldman"   , 3, 4),
    // Philosopher::new("Michel Foucault", 4, 0),
    Philosopher::new("Michel Foucault", 0, 4), 
    // A left-handed philosopher lets the whole world eat. 
  ];

  let handles: Vec<_> = 
        philosophers.into_iter()
                    .map( |p| 
                           { let table = table.clone(); // bumps ref count;
                                                        // it drops at end of scope
                             thread::spawn(move || { p.eat(&table); })
                     })
                    .collect();

  for h in handles {
    h.join().unwrap();
  }
}

