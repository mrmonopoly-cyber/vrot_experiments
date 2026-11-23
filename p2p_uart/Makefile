OUT_P2P_UART := p2p_uart

build : build_pkg_p2p_uart
clean: p2p_uart_clean

build_pkg_p2p_uart: 
	cp target/x86_64-unknown-linux-gnu/debug/libp2p_uart.so $(P2P_UART_ROOT)/pkg/lib.so
	cd $(P2P_UART_ROOT)/pkg && tar -cf p2p_uart.tar ./*
	mv $(P2P_UART_ROOT)/pkg/p2p_uart.tar ./pkgs

p2p_uart_clean:
	rm -f $(P2P_UART_ROOT)/$(OUT_P2P_UART).so
	rm -f $(P2P_UART_ROOT)/$(OUT_P2P_UART).aot
	rm -f $(P2P_UART_ROOT)/pkg/lib.so
	rm -f $(P2P_UART_ROOT)/pkg/$(OUT_P2P_UART).tar
	rm -f pkgs/p2p_uart.tar

