use std::fmt;
use std::str::FromStr;

/// The supported physical units. Construction does not convert between units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Unit {
    Dimensionless,
    Celsius,
    MetresPerSecondSquared,
    Volt,
}

impl Unit {
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Dimensionless => "1",
            Self::Celsius => "Cel",
            Self::MetresPerSecondSquared => "m/s2",
            Self::Volt => "V",
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitParseError;

impl fmt::Display for UnitParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unknown unit symbol")
    }
}

impl std::error::Error for UnitParseError {}

impl FromStr for Unit {
    type Err = UnitParseError;

    fn from_str(symbol: &str) -> Result<Self, Self::Err> {
        match symbol {
            "1" => Ok(Self::Dimensionless),
            "Cel" => Ok(Self::Celsius),
            "m/s2" => Ok(Self::MetresPerSecondSquared),
            "V" => Ok(Self::Volt),
            _ => Err(UnitParseError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservationError {
    InvalidChannel,
    NonFiniteValue,
}

impl fmt::Display for ObservationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidChannel => f.write_str("channel must contain 1-128 ASCII letters, digits, dots, underscores, colons, or hyphens"),
            Self::NonFiniteValue => f.write_str("observation value must be finite"),
        }
    }
}

impl std::error::Error for ObservationError {}

/// One validated value at a source-relative microsecond timestamp.
#[derive(Debug, Clone, PartialEq)]
pub struct Observation {
    channel: String,
    timestamp_us: i64,
    value: f64,
    unit: Unit,
}

impl Observation {
    pub fn new(
        channel: impl Into<String>,
        timestamp_us: i64,
        value: f64,
        unit: Unit,
    ) -> Result<Self, ObservationError> {
        let channel = channel.into();
        if channel.is_empty()
            || channel.len() > 128
            || !channel
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"._:-".contains(&byte))
        {
            return Err(ObservationError::InvalidChannel);
        }
        if !value.is_finite() {
            return Err(ObservationError::NonFiniteValue);
        }
        Ok(Self {
            channel,
            timestamp_us,
            value,
            unit,
        })
    }

    pub fn channel(&self) -> &str {
        &self.channel
    }

    pub const fn timestamp_us(&self) -> i64 {
        self.timestamp_us
    }

    pub const fn value(&self) -> f64 {
        self.value
    }

    pub const fn unit(&self) -> Unit {
        self.unit
    }
}
