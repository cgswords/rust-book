use machine::datatypes::Machine;
// Why does that work, but lmc::machine::datatypes::Machine not?

pub fn print_machine(mach : &mut Machine) {
    println!("---------------------------------------------------------------");
    println!("| Acc    | {:3}                                                |",
             mach.get_acc());
    println!("| PC     | {:3}                                                |",
             mach.get_pc());
    println!("---------------------------------------------------------------");
//     println!("| Inbox  | {:3}                                                |",
//              mach.get_inbox().into_iter());
    println!("| Outbox | {:3}                                                |",
             mach.get_outbox());
    println!("---------------------------------------------------------------");
    for x in 0..9 {
      let x = x * 10;
     println!("| {:3}   {:3}   {:3}   {:3}   {:3}   {:3}   {:3}   {:3}   {:3}   {:3}   |"
              , mach.lookup(x)  , mach.lookup(x+1)
              , mach.lookup(x+2), mach.lookup(x+3)
              , mach.lookup(x+4), mach.lookup(x+5)
              , mach.lookup(x+6), mach.lookup(x+7)
              , mach.lookup(x+8), mach.lookup(x+9)
              );
    }
    println!("---------------------------------------------------------------");
}
