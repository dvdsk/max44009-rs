//! Public types

/// All possible errors in this crate
#[derive(Debug)]
#[cfg_attr(feature = "thiserror", derive(thiserror::Error))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(defmt::Format)]
pub enum Error<E: defmt::Format> {
    /// I²C bus error
    #[cfg_attr(feature = "thiserror", error("I²C bus error: {0}"))]
    I2C(E),
    #[cfg_attr(
        feature = "thiserror",
        error(
            "A manual-configuration-mode-only was attempted while in automatic configuration mode."
        )
    )]
    /// A manual-configuration-mode-only was attempted while in automatic configuration mode.
    OperationNotAvailable,
}

impl<E> Clone for Error<E>
where
    E: defmt::Format + core::fmt::Debug + Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::I2C(e) => Self::I2C(e.clone()),
            Self::OperationNotAvailable => Self::OperationNotAvailable,
        }
    }
}

#[cfg(feature = "postcard")]
impl<E> postcard::experimental::max_size::MaxSize for Error<E>
where
    E: postcard::experimental::max_size::MaxSize + core::fmt::Debug + defmt::Format,
{
    const POSTCARD_MAX_SIZE: usize = 1 + E::POSTCARD_MAX_SIZE;
}

/// Measurement mode
#[derive(defmt::Format, Clone, Copy, PartialEq)]
pub enum MeasurementMode {
    /// Once every 800ms mode (default).
    ///
    /// Measures lux intensity every 800ms regardless of the integration time.
    /// Sensor operates on lowest possible supply current.
    OnceEvery800ms,
    /// Continuous mode.
    ///
    /// Continuously measures lux intensity. As soon as a reading finishes,
    /// the next one begins. The actual cadence depends on the integration
    /// time selected.
    Continuous,
}

/// Configuration mode
#[derive(defmt::Format, Clone, Copy, PartialEq)]
pub enum ConfigurationMode {
    /// Automatic mode (default).
    ///
    /// On-chip algorithm selects the integration time (100ms - 800ms) and
    /// the current division ratio
    Automatic,
    /// Manual mode.
    ///
    /// The user can select the integration time and the current division
    /// ratio manually.
    Manual,
}

/// Integration time
#[derive(defmt::Format, Clone, Copy, PartialEq)]
pub enum IntegrationTime {
    /// 6.25ms. (Only in manual mode)
    _6_25ms,
    /// 12.5ms. (Only in manual mode)
    _12_5ms,
    /// 25ms. (Only in manual mode)
    _25ms,
    /// 50ms. (Only in manual mode)
    _50ms,
    /// 100ms. (Preferred mode for high-brightness applications)
    _100ms,
    /// 200ms
    _200ms,
    /// 400ms
    _400ms,
    /// 800ms. (Preferred mode for boosting low-light sensitivity)
    _800ms,
}

/// Current division ratio
#[derive(defmt::Format, Clone, Copy, PartialEq)]
pub enum CurrentDivisionRatio {
    /// No current division (default).
    ///
    /// All the photodiode current goes to the ADC.
    One,
    /// 1/8 current division ratio.
    ///
    /// Only 1/8 of the photodiode current goes to the ADC. This mode is used in
    /// high-brightness situations.
    OneEighth,
}

/// Possible slave addresses
#[derive(defmt::Format, Clone, Copy, PartialEq)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit value for A0
    Alternative(bool),
}

impl Default for SlaveAddr {
    /// Default slave address
    fn default() -> Self {
        SlaveAddr::Default
    }
}

impl SlaveAddr {
    pub(crate) fn addr(self, default: u8) -> u8 {
        match self {
            SlaveAddr::Default => default,
            SlaveAddr::Alternative(a0) => default | a0 as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DEVICE_BASE_ADDRESS;

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(DEVICE_BASE_ADDRESS, addr.addr(DEVICE_BASE_ADDRESS));
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(
            0b100_1010,
            SlaveAddr::Alternative(false).addr(DEVICE_BASE_ADDRESS)
        );
        assert_eq!(
            0b100_1011,
            SlaveAddr::Alternative(true).addr(DEVICE_BASE_ADDRESS)
        );
    }
}
