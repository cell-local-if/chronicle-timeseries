# Observation contract

## Channel identity

A channel identifier contains between 1 and 128 ASCII bytes. Accepted characters are letters, digits, dot, underscore, colon, and hyphen. Identifiers are case-sensitive. Leading or trailing whitespace is not removed, and whitespace is not accepted. An identifier denotes one measurement channel rather than a display label.

## Timestamp

`timestamp_us` is a signed 64-bit count of microseconds relative to the origin chosen for the data source. Negative timestamps are valid. The type does not imply a civil timezone or a Unix epoch. Combining sources requires the caller to establish a common origin explicitly.

## Values and units

An observation has one finite `f64` value. NaN and positive or negative infinity are rejected. Zero, negative zero, and negative finite values remain valid. Construction performs no rounding, calibration, or unit conversion.

The initial unit set is dimensionless (`1`), degrees Celsius (`Cel`), acceleration in metres per second squared (`m/s2`), and volts (`V`). Symbols are case-sensitive. Unknown symbols are rejected by `Unit::from_str`.

## Batches

A batch owns an ordered sequence of observations. An empty batch is valid. A batch can contain multiple channels, units, and timestamps, including repeated observations and timestamps that move backwards. Construction does not infer ordering, remove duplicates, resample, or combine values. Consumers that need those operations must apply an explicit policy.

Fields of validated observations are read-only through the public interface. The batch can be inspected through a slice or consumed to recover the owned vector.
