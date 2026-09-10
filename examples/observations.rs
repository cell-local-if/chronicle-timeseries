use chronicle_timeseries::{Batch, Observation, Unit};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let batch = Batch::new(vec![
        Observation::new("motor.temperature", 1_250_000, 23.5, Unit::Celsius)?,
        Observation::new(
            "motor.vibration",
            1_250_050,
            -0.012,
            Unit::MetresPerSecondSquared,
        )?,
    ]);
    for observation in batch {
        println!(
            "{}\t{}\t{}\t{}",
            observation.channel(),
            observation.timestamp_us(),
            observation.value(),
            observation.unit()
        );
    }
    Ok(())
}
