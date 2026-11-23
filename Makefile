SDK ?= ThirdParty/wasi-sdk
WAMRC ?= ThirdParty/wamr/wasm-micro-runtime/wamr-compiler/build/wamrc

CC := $(SDK)/bin/clang --sysroot $(SDK)/share/wasi-sysroot

WASM_ARGS ?= --target=wasm32-wasip1-threads \
		-O0 \
		-ggdb \
		-mexec-model=reactor \
		-pthread \
		-Wl,--no-entry \
		-Wl,--shared-memory \
		-Wl,--export-all \



WAMRC_ARGS ?= --emit-custom-sections=name,.debug_info,.debug_abbrev,.debug_str,.debug_line 

CLIENT_ROOT ?= ./client
SERVER_ROOT ?= ./server
P2P_BUS_ROOT ?= ./p2p_bus
P2P_UART_ROOT ?= ./p2p_uart
RTC_TIMER_ROOT ?= ./rtc_timer

all: build
clean: clean_sources

build: pre_build

run: build
	cargo run

pre_build:
	cargo build

clean_sources:
	cargo clean


include ./client/Makefile
include ./server/Makefile
include ./p2p_bus/Makefile
include ./p2p_uart/Makefile
include ./rtc_timer/Makefile
