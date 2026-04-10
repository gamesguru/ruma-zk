use jolt_sdk::host::Program;
use ruma_zk_guest::*;
fn main() {
    let mut cp = Program::new("src/guest");
    let (output, summary) = analyze_verify_topology(&mut cp, vec![], [0u8; 32], 0);
    println!("cycles: {}", summary.trace_len);
}
