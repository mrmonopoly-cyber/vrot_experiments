OUT_P2P_BUS := p2p_bus

build: build_pkg_p2p_bus
clean: p2p_bus_clean

build_pkg_p2p_bus: 
	cp target/x86_64-unknown-linux-gnu/debug/libp2p_bus.so $(P2P_BUS_ROOT)/pkg/lib.so
	cd $(P2P_BUS_ROOT)/pkg && tar -cf p2p_bus.tar ./*
	mv $(P2P_BUS_ROOT)/pkg/p2p_bus.tar ./pkgs

p2p_bus_clean:
	rm -f $(P2P_BUS_ROOT)/$(OUT_P2P_BUS).so
	rm -f $(P2P_BUS_ROOT)/$(OUT_P2P_BUS).aot
	rm -f $(P2P_BUS_ROOT)/pkg/lib.so
	rm -f $(P2P_BUS_ROOT)/pkg/$(OUT_P2P_BUS).tar
	rm -f pkgs/p2p_bus.tar

