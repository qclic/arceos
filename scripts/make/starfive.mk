starfive: build
	gzip -9 -cvf $(OUT_BIN) > target/arceos-starfive.bin.gz
	mkimage -f tools/starfive/starfive_fdt.its target/arceos.itb
	@echo 'Built the FIT-uImage arceos.itb'
