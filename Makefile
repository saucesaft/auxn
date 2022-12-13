run: build
	../uxn/bin/uxnemu output.rom

build:
	../uxn/bin/uxnasm learning.tal output.rom