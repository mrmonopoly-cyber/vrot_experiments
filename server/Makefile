OUT_SERVER := server

build: server_build
clean: server_clean

server_build: 
	$(CC) $(WASM_ARGS) $(SERVER_ROOT)/src/main.c -o $(SERVER_ROOT)/$(OUT_SERVER).so
	$(WAMRC) -o $(SERVER_ROOT)/pkg/lib.so $(WAMRC_ARGS) $(SERVER_ROOT)/$(OUT_SERVER).so
	cd $(SERVER_ROOT)/pkg && tar -cf server.tar ./*
	mv $(SERVER_ROOT)/pkg/server.tar ./pkgs

server_clean:
	rm -f $(SERVER_ROOT)/$(OUT_SERVER).so
	rm -f $(SERVER_ROOT)/$(OUT_SERVER).aot
	rm -f $(SERVER_ROOT)/pkg/lib.so
	rm -f $(SERVER_ROOT)/pkg/$(OUT_SERVER).tar
	rm -rf ./pkgs/server.tar
