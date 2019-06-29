#![deny(warnings)]
extern crate mpi;

use mpi::traits::*;

<<<<<<< HEAD
const CNAME: &str = "__rsmpi__test";
=======
const CNAME: &'static str = "__rsmpi__test";
>>>>>>> a4c89e7a90608833ca21f9aa30e3af57e4bb5fdd

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    assert_eq!("MPI_COMM_WORLD", world.get_name());
    world.set_name(CNAME);
    assert_eq!(CNAME, world.get_name());
}
