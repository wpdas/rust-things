// The use declaration can be used so manual scoping isn't needed:
#![allow(dead_code)]

#[derive(Debug)]
enum Status {
    Pending,
    InProgress,
    Done,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // Explicitly `use` each name so they are available without
    // manual scoping.
    use crate::Status::{Pending, InProgress, Done}; // crate = caixote
    // use crate::Status::Pending // Tambem funciona
    use crate::Work::*; // usa todos os items do enum Work

    let mut current_status = Pending;
    let work = Civilian;

    println!("current status: {:?}", current_status);

    // current_status = InProgress;
    current_status = Done;

    match current_status {
        Pending => print!("pendente!\n"),
        InProgress => print!("carregando...\n"),
        Done => print!("operacao concluida com sucesso!\n")
    }

    match work {
        Civilian => print!("Civilians work!"),
        Soldier => print!("Soldiers fight!")
    }
}