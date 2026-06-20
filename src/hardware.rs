use defmt::info;
use embassy_stm32::{
    Config, adc::{self, Adc, AdcChannel, AnyAdcChannel, SampleTime}, bind_interrupts, gpio::{Flex, Input, Level, Output, Pin, Pull, Speed}, pac, peripherals::ADC1, rcc::{AHBPrescaler, APBPrescaler, Pll, PllMul, PllPreDiv, PllSource, Sysclk}
};

bind_interrupts!(struct Irqs {
    ADC1_COMP => adc::InterruptHandler<ADC1>;
});

pub struct Leds {
    pub(crate) read_led: Output<'static>,
    pub(crate) write_led: Output<'static>,
}

pub struct Hardware {
    pub leds: Leds,
    pub sensor: Sensor,
}

impl Sensor {
    pub async fn key_level_read(&mut self) -> u16 {
        self.adc.read(&mut self.key_adc_channel, SampleTime::CYCLES1_5).await
    }

    pub fn enable_key(&mut self) {
        self.key_enable_pin.set_as_output(Speed::Low);
        self.key_enable_pin.set_low();
    }

    pub fn disable_key(&mut self) {
        // Return to tri-state input
        self.key_enable_pin.set_as_input(Pull::None);
    }
}

pub struct Sensor {
    key_adc_channel: AnyAdcChannel<'static, ADC1>,
    adc: Adc<'static, ADC1>,
    key_enable_pin: Flex<'static>,
}

pub fn init() -> Hardware {
    let mut config = Config::default();

    
    // 1. Enable the internal HSI oscillator (8 MHz)
    config.rcc.hsi = true;
    config.rcc.hse = None;

    // 2. Define the PLL configuration: (8 MHz / 2) * 12 = 48 MHz
    config.rcc.pll = Some(Pll {
        src: PllSource::HSI,
        prediv: PllPreDiv::DIV2,
        mul: PllMul::MUL12,
    });

    // 3. Set System Clock to use the configured PLL
    config.rcc.sys = Sysclk::PLL1_P;

    // 4. Ensure AHB and APB buses run at 48 MHz (no division)
    config.rcc.ahb_pre = AHBPrescaler::DIV1;
    config.rcc.apb1_pre = APBPrescaler::DIV1;

    let p = embassy_stm32::init(config);
    info!("Hardware initialized! Clocked at 48 MHz (HSI + PLL)");

    let leds = Leds {
        read_led: Output::new(p.PC9, Level::Low, Speed::Low),
        write_led: Output::new(p.PC8, Level::Low, Speed::Low),
    };

    let sensor = Sensor {
        key_adc_channel: p.PA1.degrade_adc(),
        adc: Adc::new(p.ADC1, Irqs),
        key_enable_pin: Flex::new(p.PA2), // Default: input, no pull = tri-state
    };

    Hardware { 
        leds,
        sensor,
    }
}