use chronicle_timeseries::{Batch, Observation, ObservationError, Unit};

#[test]
fn retains_identity_source_time_value_and_unit() {
    let observation = Observation::new("Motor-1.temp:a", -50, -0.0, Unit::Celsius).unwrap();
    assert_eq!(observation.channel(), "Motor-1.temp:a");
    assert_eq!(observation.timestamp_us(), -50);
    assert_eq!(observation.value().to_bits(), (-0.0f64).to_bits());
    assert_eq!(observation.unit(), Unit::Celsius);
}

#[test]
fn validates_channel_bytes_without_normalizing_them() {
    for channel in [
        "",
        " temperature",
        "temperature ",
        "a/b",
        "温度",
        "a\tb",
        "a\nb",
    ] {
        assert_eq!(
            Observation::new(channel, 0, 1.0, Unit::Dimensionless),
            Err(ObservationError::InvalidChannel)
        );
    }
    assert!(Observation::new("a".repeat(128), 0, 1.0, Unit::Volt).is_ok());
    assert_eq!(
        Observation::new("a".repeat(129), 0, 1.0, Unit::Volt),
        Err(ObservationError::InvalidChannel)
    );
}

#[test]
fn accepts_the_full_timestamp_range() {
    for timestamp in [i64::MIN, -1, 0, i64::MAX] {
        let observation = Observation::new("signal", timestamp, 1.0, Unit::Volt).unwrap();
        assert_eq!(observation.timestamp_us(), timestamp);
    }
}

#[test]
fn rejects_all_non_finite_values() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(
            Observation::new("signal", 0, value, Unit::Volt),
            Err(ObservationError::NonFiniteValue)
        );
    }
    for value in [f64::MIN, f64::MAX, f64::MIN_POSITIVE, -0.0] {
        assert!(Observation::new("signal", 0, value, Unit::Volt).is_ok());
    }
}

#[test]
fn unit_symbols_round_trip_without_case_folding() {
    for unit in [
        Unit::Dimensionless,
        Unit::Celsius,
        Unit::MetresPerSecondSquared,
        Unit::Volt,
    ] {
        assert_eq!(unit.to_string().parse::<Unit>(), Ok(unit));
    }
    for symbol in ["C", "cel", "v", " V", "V ", "m/s²", ""] {
        assert!(symbol.parse::<Unit>().is_err());
    }
}

#[test]
fn empty_batches_have_no_implicit_observations() {
    let batch = Batch::default();
    assert!(batch.is_empty());
    assert_eq!(batch.len(), 0);
    assert!(batch.into_vec().is_empty());
}

#[test]
fn batches_preserve_mixed_channels_repetition_and_input_order() {
    let first = Observation::new("temperature", 100, 22.0, Unit::Celsius).unwrap();
    let earlier = Observation::new("voltage", 50, 3.3, Unit::Volt).unwrap();
    let supplied = vec![first.clone(), earlier, first];
    let batch = Batch::new(supplied.clone());
    assert_eq!(batch.len(), 3);
    assert!(!batch.is_empty());
    assert_eq!(batch.as_slice(), supplied.as_slice());
    assert_eq!(batch.into_iter().collect::<Vec<_>>(), supplied);
}
