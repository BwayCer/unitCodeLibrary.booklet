use std::thread::sleep;
use std::time::Duration;

use rand::prelude::*;

use resource_management_test::{
    check_mem_usage, run_round, run_round_by_plan, PLAN_RUN_COUNT_LIST_LENGTH,
};

fn main() {
    for plan_idx in 0..PLAN_RUN_COUNT_LIST_LENGTH {
        run_round_by_plan(plan_idx);
    }

    let mut rng = rand::rng();
    let max_run_count = (2 as u32).pow(17) + 1;
    let loop_times = rng.random_range(36..=99);
    for loop_idx in 0..loop_times {
        println!("loop_times: {loop_idx:>2} / {loop_times:>2}");
        let action_mod = rng.random_range(0..9);
        match action_mod {
            1 => check_mem_usage(),
            _ => run_round(rng.random_range(1..max_run_count)),
        }
        sleep(Duration::from_millis(rng.random_range(700..=5000)));
    }
}
