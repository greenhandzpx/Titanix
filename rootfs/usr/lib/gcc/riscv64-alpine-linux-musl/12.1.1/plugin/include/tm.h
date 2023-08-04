#ifndef GCC_TM_H
#define GCC_TM_H
#ifndef LIBC_GLIBC
# define LIBC_GLIBC 1
#endif
#ifndef LIBC_UCLIBC
# define LIBC_UCLIBC 2
#endif
#ifndef LIBC_BIONIC
# define LIBC_BIONIC 3
#endif
#ifndef LIBC_MUSL
# define LIBC_MUSL 4
#endif
#ifndef DEFAULT_LIBC
# define DEFAULT_LIBC LIBC_MUSL
#endif
#ifndef ANDROID_DEFAULT
# define ANDROID_DEFAULT 0
#endif
#ifndef TARGET_DEFAULT_ISA_SPEC
# define TARGET_DEFAULT_ISA_SPEC ISA_SPEC_CLASS_20191213
#endif
#ifndef TARGET_RISCV_ATTRIBUTE
# define TARGET_RISCV_ATTRIBUTE 1
#endif
#ifndef TARGET_RISCV_DEFAULT_ARCH
# define TARGET_RISCV_DEFAULT_ARCH rv64gc
#endif
#ifndef TARGET_RISCV_DEFAULT_ABI
# define TARGET_RISCV_DEFAULT_ABI lp64d
#endif
#ifdef IN_GCC
# include "options.h"
# include "insn-constants.h"
# include "config/elfos.h"
# include "config/gnu-user.h"
# include "config/linux.h"
# include "config/glibc-stdint.h"
# include "config/riscv/riscv.h"
# include "config/riscv/linux.h"
# include "config/initfini-array.h"
#endif
#if defined IN_GCC && !defined GENERATOR_FILE && !defined USED_FOR_TARGET
# include "insn-flags.h"
#endif
#if defined IN_GCC && !defined GENERATOR_FILE
# include "insn-modes.h"
#endif
# include "defaults.h"
#endif /* GCC_TM_H */
