# auxn!
[![justforfunnoreally.dev badge](https://img.shields.io/badge/justforfunnoreally-dev-9ff)](https://justforfunnoreally.dev)

uxn virtual machine running inside of a audio plugin (standalone mode included)

🚧🚧🚧 beware of dragons and messy code 🚧🚧🚧
![screenshot of in progress development, showing a memory view and the actual emulator](./pics/progress.png)

*whats inside?*

- a reimplementation of uxn in rust (functional but could really use some structuring)
- custom-varvara implementation runnning as a vst
	- console - only supports output
	- screen - mostly implemented, graphical glitches
	- audio - none


## tests passed
- [x] arithmetic.rom
- [x] literals.rom
- [x] jumps.rom
- [x] memory.rom
- [x] stack.rom

### special thanks to:
- rekka and devine for creating uxn
- compudanzas and their amazing uxn tutorial
- the c2rust team which helped a lot in this port