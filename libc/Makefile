MUSL_LIB?=/home/greenhandzpx/Downloads/riscv64-linux-musl-cross/lib
PREFIX?=riscv64-linux-musl-
#MUSL_LIB?=/opt/riscv/musl-riscv-toolchain-8.2.0-1/riscv64-linux-musl/lib
#PREFIX?=riscv64-linux-musl-
CC=$(PREFIX)gcc
OBJCOPY=$(PREFIX)objcopy
OBJDUMP=$(PREFIX)objdump

STATIC_TESTCASES = $(shell cat static_testcases.txt)
DYAMIC_TESTCASES = $(shell cat dynamic_testcases.txt)
OUTPUT_TESTCASES = $(addprefix build/,  $(subst -,_,$(notdir $(basename $(STATIC_TESTCASES)))))

CFLAGS += -pipe -std=c99 -D_POSIX_C_SOURCE=200809L -Wall -Wno-unused-function -Wno-missing-braces -Wno-unused -Wno-overflow
CFLAGS += -Wno-unknown-pragmas -fno-builtin -frounding-math
CFLAGS += -Werror=implicit-function-declaration -Werror=implicit-int -Werror=pointer-sign -Werror=pointer-arith
CFLAGS += -Os -s
CFLAGS += -Isrc/common/
LDLIBS += -Os -s -lpthread -lm -lrt

# 支持静态测例
#CFLAGS += -static

COMMON_SRC_FILES = fdfill.c memfill.c mtest.c path.c print.c rand.c setrlim.c utf8.c vmfill.c
COMMON_SRCS = $(addprefix src/common/,  $(COMMON_SRC_FILES))

DSO_SRCS = src/functional/tls_align_dso.c src/functional/tls_init_dso.c src/functional/dlopen_dso.c src/regression/tls_get_new-dtv_dso.c
DSO_SOS = $(DSO_SRCS:%.c=%.so)

DIR ?= ./src/
TARGET_NAME ?= argv

all: build_static

build: $(DIR)$(TARGET_NAME).out

debug: $(DIR)$(TARGET_NAME).dbg

asm: $(DIR)$(TARGET_NAME).S

clean:
	find src -name '*.out' | xargs -t rm
	find src -name '*.dout' | xargs -t rm

# 静态测例
%.out : %.c
	@echo "------------ build static cases --------------"
	$(CC) $^ $(COMMON_SRCS) $(CFLAGS) -static -o $@
	mkdir -p build
	cp $@ $(addprefix build/,  $(subst -,_,$(notdir $(basename $@))))

#动态测例
%.dout : %.c
	$(CC) $(LDFLAGS) $(CFLAGS) $^ -DDYNAMIC $(COMMON_SRCS) -Lsrc/functional -Lsrc/regression -ltls_align_dso -ltls_init_dso -ldlopen_dso -ltls_get_new-dtv_dso -Wl,-rpath ./ -o $@
	mkdir -p build
	cp $@ $(addprefix build/,  $(subst -,_,$(notdir $@)))

%.so : %.c
	$(CC) $(CFLAGS) -shared -fPIC $^ -o $(dir $@)/lib$(notdir $@)
	mkdir -p build
	cp $(dir $@)/lib$(notdir $@) $(addprefix build/lib,  $(notdir $@))

src/functional/tls_align.out: src/functional/tls_align.c src/functional/tls_align_dso.c
	$(CC) $^ $(COMMON_SRCS) $(CFLAGS) -o $@	

%.dbg : %.c
	$(CC) $^ $(COMMON_SRCS) $(CFLAGS) -g -o $@ 

%.S : %.out
	$(OBJDUMP) -ld $^ > $@ 

test:
	@echo $(STATIC_TESTCASES)

build_static: $(STATIC_TESTCASES)

build_dynamic: $(DSO_SOS) $(DYAMIC_TESTCASES)
	mkdir -p build
	cp ./libc.so ./build/libc.so


dynamic_test: $(DSO_SOS) $(DIR)$(TARGET_NAME).c
	$(CC) $(LDFLAGS) $(CFLAGS) $(DIR)$(TARGET_NAME).c -DDYNAMIC $(COMMON_SRCS) -Lsrc/functional -Lsrc/regression -ltls_align_dso -ltls_init_dso -ldlopen_dso -ltls_get_new-dtv_dso -Wl,-rpath ./ -o dynamic.dout

str_array_in_rust:
	ls build --ignore=*.dout --ignore=*.so | xargs -i echo \"{}\",
	find build -name "*.dout" | xargs basename -a | xargs -i echo \"dyn/{}\",

.PHONY: all build debug asm clean test build_static build_dynamic
#$(CC) $(CFLAGS) -c $^ -o $@
#chmod +x $@
