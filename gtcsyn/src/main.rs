mod synth;

use gtcsyn::{HemminkiCore, HEMMINKI_CONSTANT};
use crate::synth::GtcSynthesizer;

fn main() {
    let mut core = HemminkiCore::init();
    let synth = GtcSynthesizer::new(100.0);
    let data = vec![1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0, 1];

    println!("--- gtcsyn v1.0.0 start ---");
    match core.execute_transduction(&data) {
        Ok(drift) => {
            println!("status: stable_determinism");
            println!("final_e_drift: {:.4} / {}", drift, 
HEMMINKI_CONSTANT);
            let schedule = synth.generate_schedule(&data);
            println!("synth_schedule_size: {}", schedule.len());
        }
        Err(e) => {
            println!("status: failure");
            println!("reason: {}", e);
        }
    }
    println!("--- gtcsyn end ---");
}

