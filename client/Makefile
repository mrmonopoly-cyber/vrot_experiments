OUT_CLIENT := client

build: client_build
clean: client_clean

client_build: 
	$(CC) $(WASM_ARGS) $(CLIENT_ROOT)/src/main.c -o  $(CLIENT_ROOT)/$(OUT_CLIENT).so
	$(WAMRC) -o  $(CLIENT_ROOT)/pkg/lib.so $(WAMRC_ARGS)  $(CLIENT_ROOT)/$(OUT_CLIENT).so
	cd $(CLIENT_ROOT)/pkg && tar -cf client.tar ./*
	mv $(CLIENT_ROOT)/pkg/client.tar ./pkgs

client_clean:
	rm -f $(CLIENT_ROOT)/$(OUT_CLIENT).so
	rm -f $(CLIENT_ROOT)/$(OUT_CLIENT).aot
	rm -f $(CLIENT_ROOT)/pkg/lib.so
	rm -f $(CLIENT_ROOT)/pkg/$(OUT_CLIENT).tar
	rm -rf ./pkgs/client.tar
