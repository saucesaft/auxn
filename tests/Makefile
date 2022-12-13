custom_uxnemu = ../target/debug/uxncli
uxnemu = ../bin/uxncli
uxnasm = ../bin/uxnasm

all: literals arithmetic stack jumps memory

literals:
	$(uxnasm) literals.tal literals.rom

arithmetic:
	$(uxnasm) arithmetic.tal arithmetic.rom

stack:
	$(uxnasm) stack.tal stack.rom

jumps:
	$(uxnasm) jumps.tal jumps.rom

memory:
	$(uxnasm) memory.tal memory.rom

run: all
	$(foreach file, $(wildcard ./*.rom), echo "---$(file)--"; echo "--> metric:";$(uxnemu) $(file) < /dev/null | xxd; echo "--> custom:";$(custom_uxnemu) $(file) < /dev/null | xxd; )
