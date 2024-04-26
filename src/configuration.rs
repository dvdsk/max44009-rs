use crate::{
    BitFlags, ConfigurationMode, CurrentDivisionRatio, Error, IntegrationTime, Max44009,
    MeasurementMode, Register,
};
use embedded_hal_async;
use embedded_hal_async::i2c::{I2c, SevenBitAddress};

impl<I2C> Max44009<I2C>
where
    I2C: I2c<SevenBitAddress>,
    I2C::Error: defmt::Format,
{
    /// Enable interrupt.
    ///
    /// The INT pin will be pulled low if the interrupt condition is triggered.
    pub async fn enable_interrupt(&mut self) -> Result<(), Error<I2C::Error>> {
        self.i2c
            .write(self.address, &[Register::INT_ENABLE, 1])
            .await
            .map_err(Error::I2C)
    }

    /// Disable interrupt.
    pub async fn disable_interrupt(&mut self) -> Result<(), Error<I2C::Error>> {
        self.i2c
            .write(self.address, &[Register::INT_ENABLE, 0])
            .await
            .map_err(Error::I2C)
    }

    /// Set the measurement mode.
    pub async fn set_measurement_mode(
        &mut self,
        mode: MeasurementMode,
    ) -> Result<(), Error<I2C::Error>> {
        let config = self.config;
        match mode {
            MeasurementMode::OnceEvery800ms => self.write_config(config & !BitFlags::CONTINUOUS),
            MeasurementMode::Continuous => self.write_config(config | BitFlags::CONTINUOUS),
        }
        .await
    }

    /// Set configuration mode.
    pub async fn set_configuration_mode(
        &mut self,
        mode: ConfigurationMode,
    ) -> Result<(), Error<I2C::Error>> {
        let config = self.config;
        match mode {
            ConfigurationMode::Automatic => self.write_config(config & !BitFlags::MANUAL),
            ConfigurationMode::Manual => self.write_config(config | BitFlags::MANUAL),
        }
        .await
    }

    /// Set integration time. (Only in manual configuration mode).
    pub async fn set_integration_time(
        &mut self,
        it: IntegrationTime,
    ) -> Result<(), Error<I2C::Error>> {
        self.assert_is_in_manual_mode()?;
        let config = self.config & 0b1111_1000;
        match it {
            IntegrationTime::_800ms => self.write_config(config),
            IntegrationTime::_400ms => self.write_config(config | 0x01),
            IntegrationTime::_200ms => self.write_config(config | 0x02),
            IntegrationTime::_100ms => self.write_config(config | 0x03),
            IntegrationTime::_50ms => self.write_config(config | 0x04),
            IntegrationTime::_25ms => self.write_config(config | 0x05),
            IntegrationTime::_12_5ms => self.write_config(config | 0x06),
            IntegrationTime::_6_25ms => self.write_config(config | 0x07),
        }
        .await
    }

    /// Set current division ratio. (Only in manual configuration mode).
    pub async fn set_current_division_ratio(
        &mut self,
        cdr: CurrentDivisionRatio,
    ) -> Result<(), Error<I2C::Error>> {
        self.assert_is_in_manual_mode()?;
        let config = self.config;
        match cdr {
            CurrentDivisionRatio::One => self.write_config(config & !BitFlags::CDR),
            CurrentDivisionRatio::OneEighth => self.write_config(config | BitFlags::CDR),
        }
        .await
    }

    async fn write_config(&mut self, config: u8) -> Result<(), Error<I2C::Error>> {
        self.i2c
            .write(self.address, &[Register::CONFIGURATION, config])
            .await
            .map_err(Error::I2C)?;
        self.config = config;
        Ok(())
    }

    fn assert_is_in_manual_mode(&self) -> Result<(), Error<I2C::Error>> {
        if (self.config & BitFlags::MANUAL) == 0 {
            return Err(Error::OperationNotAvailable);
        }
        Ok(())
    }
}
