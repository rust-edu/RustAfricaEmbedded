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
   * Embedded dev boards / micro:bit v2 (MB2)
   * microcontrollers / nRF52833
   * Assembly / machine instructions / ARM
   * Rust cross-compilation
   * Loading ARM code on the MB2

2. Blinky at several levels
   * **Reading:** [*Rust Embedded Discovery Book*] chapter 6
   * Basic electricity
   * GPIO pins
   * Using Rust crates
   * Lighting an LED
   * Blinking an LED

## Week 3: Input and Output

1. Simple I/O
   * **Reading:** [*Rust Embedded Discovery Book*] chapters 7-8
   * Using the `microbit` Board Support Package (BSP)
   * LED Array / LED Roulette
   * Buttons / Turn Signal

2. Serial I/O and protocols
   * **Reading:** [*Rust Embedded Discovery Book*] chapters 10-12, 14
   * Asynchronous serial communication
   * I2C
   * Reading the I2C accelerometer

## Week 4: Fancy Stuff

1. Interrupts
   * **Reading:** [*Rust Embedded Discovery Book*] chapter 15
   * Interrupts from the CPU point of view
   * Setting up interrupt handlers
   * Communicating between interrupt handlers and main code
   
2. Concluding
   * **Reading:** [*Rust Embedded Discovery Book*] chapter 16
   * Snake Game!
   * Topics not covered: DMA, ADC, PWM, speaker, microphone,
     external hardware, etc
   * Next steps

[*The Rust Programming Language*]: https://doc.rust-lang.org/book/
[*Rust Embedded Discovery Book*]: https://docs.rust-embedded.org/discovery-mb2/
