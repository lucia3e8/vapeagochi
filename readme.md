# tamagotchi that dies if you stop vaping
...has been rewritten in Rust.

## dirs
```
.
├── assets
├── ee # kicad, components, assembly
├── readme.txt
├── scripts # one offs
├── vg2 # old C++ stm32ide build
└── vgrs # new rust build <- USE THIS ONE
```

## Pinout

<img width="864" alt="Screenshot 2025-05-05 at 12 30 01" src="https://github.com/user-attachments/assets/916bf9c5-f07b-4014-be63-f133dea236b5" />


```
| Pin  | Function        |
|------|-----------------|
| PA5  | SCREEN_CLK      |
| PA6  | SCREEN_RES      |
| PA7  | SCREEN_DIN      |
| PA8  | SCREEN_DC       |
| PB10 | SCREEN_CS       |
| PB3  | Left Button     |
| PA0  | Middle Button   |
| PB2  | Right Button    |
| PA4  | Coil Input      |
| PB11 | Coil Enable     |
| PA1  | Programmable LED|
```
## Building
```
cd vgrs
nix-shell
cargo build --release --target thumbv6m-none-eabi # build
sudo probe-rs run --chip STM32L072CBTx target/thumbv6m-none-eabi/release/vgrs # flash
```

