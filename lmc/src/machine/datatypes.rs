pub struct Machine {
  memory : Vec<u32>,
  inbox  : Vec<u32>,
  outbox : u32,
  acc    : u32,
  pc     : usize,
}

// Design consideration: lookup doens't need a mutable
// machine. Is this good Rust style?
impl Machine {
  pub fn new(instrs: &Vec<u32>, in_stream : &Vec<u32>) -> Machine {
    Machine 
      { memory : instrs.clone()
      , inbox  : in_stream.clone()
      , outbox : 0
      , acc    : 0
      , pc     : 0
      }
  }

  // program counter stuff
  fn incr_pc(&mut self) {
    self.pc += 1;
  }

  fn set_pc(&mut self, new_pc : usize) {
    self.pc = new_pc;
  }

  // Main Loop
  pub fn run(&mut self) {
    loop {
      let cur_instr = self.memory[self.pc];
      self.incr_pc();
      if cur_instr == 0 {
        break; 
      } else if (cur_instr > 99) && (cur_instr < 200) {
        let cur_instr = cur_instr - 100;
        let instr_box = cur_instr as usize;
        self.add(instr_box);
      } else if (cur_instr > 199) && (cur_instr < 300) {
        let cur_instr = cur_instr - 200;
        let instr_box = cur_instr as usize;
        self.sub(instr_box);
      } else if (cur_instr > 299) && (cur_instr < 400) {
        let cur_instr = cur_instr - 300;
        let instr_box = cur_instr as usize;
        self.store(instr_box);
      } else if (cur_instr > 499) && (cur_instr < 600) {
        let new_pc = cur_instr - 500;
        let new_pc = new_pc as usize;
        self.load(new_pc);
      } else if (cur_instr > 599) && (cur_instr < 700) {
        let new_pc = cur_instr - 600;
        let new_pc = new_pc as usize;
        self.set_pc(new_pc);
      } else if (cur_instr > 699) && (cur_instr < 800) {
        let new_pc = cur_instr - 700;
        let new_pc = new_pc as usize;
        if self.acc == 0 { self.set_pc(new_pc); }
      } else if (cur_instr > 799) && (cur_instr < 900) {
        let new_pc = cur_instr - 800;
        let new_pc = new_pc as usize;
        if self.acc > 0 { self.set_pc(new_pc); }
      } else if cur_instr == 901 { 
        self.input();
      } else if cur_instr == 902 { 
        self.output();
      }
    }
  }
  
  // Getters
  pub fn get_memory(&self) -> Vec<u32> {
    self.memory.clone()  
  }

  // pub fn get_inbox(&self) -> Vec<u32> {
  //   self.inbox  
  // }

  pub fn get_outbox(&self) -> u32 {
    self.outbox  
  }

  pub fn get_acc(&self) -> u32 {
    self.acc  
  }

  pub fn get_pc(&self) -> usize {
    self.pc  
  }

  pub fn lookup(&self, addr : usize) -> u32 {
    self.memory[addr]
  }

  // Instructions
  pub fn store(&mut self, addr : usize,) {
    self.memory[addr] = self.acc;
  }

  pub fn load(&mut self, addr : usize) {
    self.acc = self.memory[addr];
  }

  pub fn input(&mut self) {
    self.acc = self.inbox.pop().expect("Empty input"); 
  }

  pub fn output(&mut self) {
    self.outbox = self.acc; 
  }

  pub fn sub(&mut self, addr : usize) {
    self.acc = self.acc - self.memory[addr]; 
  }

  pub fn add(&mut self, addr : usize) {
    self.acc = self.acc + self.memory[addr]; 
  }
}

