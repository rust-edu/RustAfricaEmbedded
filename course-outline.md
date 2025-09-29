# Discover Embedded Rust
Bart Massey 2025-09

## Week 1: Rust Systems Programming

1. Review of Rust
   * **Reading:** [*The Rust Programming Language*] chapters 1-4
   * Installing Rust, Cargo, etc
   * Rust basic datatypes
   * Rust variables
   * Rust expressions and statements
   * Rust functions
   * Rust references

2. More Rust
   * **Reading:** [*The Rust Programming Language*] chapters 5-6, 7.1, 9-10
   * Rust move semantics and ownership
   * Rust structs and enums
   * Rust arrays

## Week 2: Beginning Rust Embedded

1. Understanding Embedded
   * **Reading:** [*Rust Embedded Discovery Book*] chapters 1-5
   * Rust traits
   * Rust error handling
   * `no_std` Rust with `core`
   * Assembly / machine instructions / ARM
   * Rust cross-compilation
   * Loading ARM code on the MB2

2. Simple I/O
   * Blinkies at several levels
     * Basic electricity
     * Embedded dev boards / micro:bit v2 (MB2)
     * microcontrollers / nRF52833
     * **Reading:** [*Rust Embedded Discovery Book*] chapter 6
     * GPIO pins
     * Using Rust crates
     * Layers
       * Device registers
       * Peripheral Access Crate (PAC)
       * Hardware Abstraction Layer (HAL)
       * Board Support Package (BSP)
     * Lighting an LED
     * Blinking an LED
   * Simple Output
     * **Reading:** [*Rust Embedded Discovery Book*] chapter 7
     * Using the `microbit` Board Support Package (BSP)
     * LED Array / LED Roulette

## Week 3: Input and Output

1. More I/O
  * Simple Input
    * **Reading:** [*Rust Embedded Discovery Book*] chapter 8
    * Buttons / Turn Signal
  * Serial I/O and protocols
    * **Reading:** [*Rust Embedded Discovery Book*] chapters 10-12, 14
    * Asynchronous serial communication
    * I2C
    * Reading the I2C accelerometer

2. Interrupts
   * **Reading:** [*Rust Embedded Discovery Book*] chapter 15
   * Interrupts from the CPU point of view
   * Setting up interrupt handlers
   * Communicating between interrupt handlers and main code

## Week 4: Wrap-Up
   
1. Concluding
   * **Reading:** [*Rust Embedded Discovery Book*] chapter 16
   * Snake Game!
   * Topics not covered: DMA, ADC, PWM, speaker, microphone,
     external hardware, etc
   * Next steps

[*The Rust Programming Language*]: https://doc.rust-lang.org/book/
[*Rust Embedded Discovery Book*]: https://docs.rust-embedded.org/discovery-mb2/
