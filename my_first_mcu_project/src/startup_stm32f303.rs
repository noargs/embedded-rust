// 1. define the vector table for the mcu
// 2. define the reset handler for the mcu
// 3. define the panic handler i.e. exception handler for the mcu

#[unsafe(no_mangle)]
extern "C" fn HardFault_Handler() { loop {} }
#[unsafe(no_mangle)]
extern "C" fn NMI_Handler() { loop {} }
#[unsafe(no_mangle)]
extern "C" fn Reset_Handler() { loop {} }
#[unsafe(no_mangle)]
extern "C" fn Default_Handler() { loop {} }

unsafe extern "C" {
    unsafe fn BusFault_Handler();
    unsafe fn MemManage_Handler();
    unsafe fn PendSV_Handler();
    unsafe fn SVCall_Handler();
    unsafe fn SysTick_Handler();
    unsafe fn UsageFault_Handler();
    unsafe fn ADC1_2_Handler();
    unsafe fn ADC3_Handler();
    unsafe fn ADC4_Handler();
    unsafe fn CAN_RX1_Handler();
    unsafe fn CAN_SCE_Handler();
    unsafe fn COMP123_Handler();
    unsafe fn COMP456_Handler();
    unsafe fn COMP7_Handler();
    unsafe fn DMA1_CH1_Handler();
    unsafe fn DMA1_CH2_Handler();
    unsafe fn DMA1_CH3_Handler();
    unsafe fn DMA1_CH4_Handler();
    unsafe fn DMA1_CH5_Handler();
    unsafe fn DMA1_CH6_Handler();
    unsafe fn DMA1_CH7_Handler();
    unsafe fn DMA2_CH1_Handler();
    unsafe fn DMA2_CH2_Handler();
    unsafe fn DMA2_CH3_Handler();
    unsafe fn DMA2_CH4_Handler();
    unsafe fn DMA2_CH5_Handler();
    unsafe fn EXTI0_Handler();
    unsafe fn EXTI15_10_Handler();
    unsafe fn EXTI1_Handler();
    unsafe fn EXTI2_TSC_Handler();
    unsafe fn EXTI3_Handler();
    unsafe fn EXTI4_Handler();
    unsafe fn EXTI9_5_Handler();
    unsafe fn FLASH_Handler();
    unsafe fn FMC_Handler();
    unsafe fn I2C1_ER_Handler();
    unsafe fn I2C1_EV_EXTI23_Handler();
    unsafe fn I2C2_ER_Handler();
    unsafe fn I2C2_EV_EXTI24_Handler();
    unsafe fn I2C3_ER_Handler();
    unsafe fn I2C3_EV_Handler();
    unsafe fn PVD_Handler();
    unsafe fn RCC_Handler();
    unsafe fn RTCAlarm_Handler();
    unsafe fn RTC_WKUP_Handler();
    unsafe fn SPI1_Handler();
    unsafe fn SPI2_Handler();
    unsafe fn SPI3_Handler();
    unsafe fn SPI4_Handler();
    unsafe fn TAMP_STAMP_Handler();
    unsafe fn TIM1_BRK_TIM15_Handler();
    unsafe fn TIM1_CC_Handler();
    unsafe fn TIM1_TRG_COM_TIM17_Handler();
    unsafe fn TIM1_UP_TIM16_Handler();
    unsafe fn TIM20_BRK_Handler();
    unsafe fn TIM20_CC_Handler();
    unsafe fn TIM20_TRG_COM_Handler();
    unsafe fn TIM20_UP_Handler();
    unsafe fn TIM2_Handler();
    unsafe fn TIM3_Handler();
    unsafe fn TIM4_Handler();
    unsafe fn TIM6_DACUNDER_Handler();
    unsafe fn TIM7_Handler();
    unsafe fn TIM8_BRK_Handler();
    unsafe fn TIM8_CC_Handler();
    unsafe fn TIM8_TRG_COM_Handler();
    unsafe fn TIM8_UP_Handler();
    unsafe fn UART4_EXTI34_Handler();
    unsafe fn UART5_EXTI35_Handler();
    unsafe fn USART1_EXTI25_Handler();
    unsafe fn USART2_EXTI26_Handler();
    unsafe fn USART3_EXTI28_Handler();
    unsafe fn USB_HP_CAN_TX_Handler();
    unsafe fn USB_HP_Handler();
    unsafe fn USB_LP_CAN_RX0_Handler();
    unsafe fn USB_LP_Handler();
    unsafe fn USB_WKUP_EXTI_Handler();
    unsafe fn USB_WKUP_Handler();
    unsafe fn WWDG_Handler();
}

#[used]
#[unsafe(link_section = ".isr_vector")]
static VECTOR_TABLE: [Option<unsafe extern "C" fn()>; 100] = [
    Some(Reset_Handler),
    Some(NMI_Handler),
    Some(HardFault_Handler),
    Some(MemManage_Handler),
    Some(BusFault_Handler),
    Some(UsageFault_Handler),
    None,
    None,
    None,
    None,
    Some(SVCall_Handler),
    None,
    None,
    Some(PendSV_Handler),
    Some(SysTick_Handler),
    Some(WWDG_Handler),
    Some(PVD_Handler),
    Some(TAMP_STAMP_Handler),
    Some(RTC_WKUP_Handler),
    Some(FLASH_Handler),
    Some(RCC_Handler),
    Some(EXTI0_Handler),
    Some(EXTI1_Handler),
    Some(EXTI2_TSC_Handler),
    Some(EXTI3_Handler),
    Some(EXTI4_Handler),
    Some(DMA1_CH1_Handler),
    Some(DMA1_CH2_Handler),
    Some(DMA1_CH3_Handler),
    Some(DMA1_CH4_Handler),
    Some(DMA1_CH5_Handler),
    Some(DMA1_CH6_Handler),
    Some(DMA1_CH7_Handler),
    Some(ADC1_2_Handler),
    Some(USB_HP_CAN_TX_Handler),
    Some(USB_LP_CAN_RX0_Handler),
    Some(CAN_RX1_Handler),
    Some(CAN_SCE_Handler),
    Some(EXTI9_5_Handler),
    Some(TIM1_BRK_TIM15_Handler),
    Some(TIM1_UP_TIM16_Handler),
    Some(TIM1_TRG_COM_TIM17_Handler),
    Some(TIM1_CC_Handler),
    Some(TIM2_Handler),
    Some(TIM3_Handler),
    Some(TIM4_Handler),
    Some(I2C1_EV_EXTI23_Handler),
    Some(I2C1_ER_Handler),
    Some(I2C2_EV_EXTI24_Handler),
    Some(I2C2_ER_Handler),
    Some(SPI1_Handler),
    Some(SPI2_Handler),
    Some(USART1_EXTI25_Handler),
    Some(USART2_EXTI26_Handler),
    Some(USART3_EXTI28_Handler),
    Some(EXTI15_10_Handler),
    Some(RTCAlarm_Handler),
    Some(USB_WKUP_Handler),
    Some(TIM8_BRK_Handler),
    Some(TIM8_UP_Handler),
    Some(TIM8_TRG_COM_Handler),
    Some(TIM8_CC_Handler),
    Some(ADC3_Handler),
    Some(FMC_Handler),
    None,
    None,
    Some(SPI3_Handler),
    Some(UART4_EXTI34_Handler),
    Some(UART5_EXTI35_Handler),
    Some(TIM6_DACUNDER_Handler),
    Some(TIM7_Handler),
    Some(DMA2_CH1_Handler),
    Some(DMA2_CH2_Handler),
    Some(DMA2_CH3_Handler),
    Some(DMA2_CH4_Handler),
    Some(DMA2_CH5_Handler),
    Some(ADC4_Handler),
    None,
    None,
    Some(COMP123_Handler),
    Some(COMP456_Handler),
    Some(COMP7_Handler),
    None,
    None,
    None,
    None,
    None,
    Some(I2C3_EV_Handler),
    Some(I2C3_ER_Handler),
    Some(USB_HP_Handler),
    Some(USB_LP_Handler),
    Some(USB_WKUP_EXTI_Handler),
    Some(TIM20_BRK_Handler),
    Some(TIM20_UP_Handler),
    Some(TIM20_TRG_COM_Handler),
    Some(TIM20_CC_Handler),
    None,
    None,
    None,
    Some(SPI4_Handler),
];

#[unsafe(no_mangle)]
extern "C" fn reset_handler() {
  // 1. copy the .data section from flash to ram

  // 2. zero out the .bss section in ram

  // 3. call the main function
  crate::main();
}