#![deny(warnings)]
extern crate mpi;

use mpi::collective::SystemOperation;
use mpi::topology::Rank;
use mpi::traits::*;

fn fac(n: Rank) -> Rank {
<<<<<<< HEAD
    (1..=n).product()
=======
    (1..n + 1).product()
>>>>>>> a4c89e7a90608833ca21f9aa30e3af57e4bb5fdd
}

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();

    let mut x = 0;
    world.scan_into(&rank, &mut x, &SystemOperation::sum());
    assert_eq!(x, (rank * (rank + 1)) / 2);

    let y = rank + 1;
    let mut z = 0;
    world.exclusive_scan_into(&y, &mut z, &SystemOperation::product());
    if rank > 0 {
        assert_eq!(z, fac(y - 1));
    }
}
