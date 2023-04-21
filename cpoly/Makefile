SUBDIRS := c

all: build

build:
	$(MAKE) -C $(SUBDIRS) all
	bindgen c/poly.h -o src/bindings.rs

.PHONY: clean
	rm cpoly/src/bindings.rs

clean: 
	$(MAKE) -C $(SUBDIRS) clean

