#[repr(C)]
#[derive(Copy, Clone)]
/// Auxiliary header
pub struct AuxHeader {
    /// Type
    pub aux_type: usize,
    /// Value
    pub value: usize,
}

// Execution of programs
/// end of vector
pub const AT_NULL: usize = 0;
/// entry should be ignored
#[allow(unused)]
pub const AT_IGNORE: usize = 1;
/// file descriptor of program
#[allow(unused)]
pub const AT_EXECFD: usize = 2; /* file descriptor of program */
/// program headers for program
pub const AT_PHDR: usize = 3; /* program headers for program */
/// size of program header entry
pub const AT_PHENT: usize = 4; /* size of program header entry */
/// number of program headers
pub const AT_PHNUM: usize = 5; /* number of program headers */
/// system page size
pub const AT_PAGESZ: usize = 6; /* system page size */
/// base address of interpreter
pub const AT_BASE: usize = 7; /* base address of interpreter */
/// flags
pub const AT_FLAGS: usize = 8; /* flags */
/// entry point of program
pub const AT_ENTRY: usize = 9; /* entry point of program */
/// program is not ELF
pub const AT_NOTELF: usize = 10; /* program is not ELF */
/// real uid
pub const AT_UID: usize = 11; /* real uid */
/// effective uid
pub const AT_EUID: usize = 12; /* effective uid */
/// real gid
pub const AT_GID: usize = 13; /* real gid */
/// effective gid
pub const AT_EGID: usize = 14; /* effective gid */
/// string identifying CPU for optimizations
pub const AT_PLATFORM: usize = 15; /* string identifying CPU for optimizations */
/// arch dependent hints at CPU capabilities
pub const AT_HWCAP: usize = 16; /* arch dependent hints at CPU capabilities */
/// frequency at which times() increments
pub const AT_CLKTCK: usize = 17; /* frequency at which times() increments */
/// AT_* values 18 through 22 are reserved
/// secure mode boolean
pub const AT_SECURE: usize = 23; /* secure mode boolean */
/// string identifying real platform, may differ from AT_PLATFORM.
#[allow(unused)]
pub const AT_BASE_PLATFORM: usize = 24; /* string identifying real platform, may
                                         * differ from AT_PLATFORM. */
/// address of 16 random bytes
pub const AT_RANDOM: usize = 25; /* address of 16 random bytes */
/// extension of AT_HWCAP
#[allow(unused)]
pub const AT_HWCAP2: usize = 26; /* extension of AT_HWCAP */
/// filename of program
pub const AT_EXECFN: usize = 31; /* filename of program */
/// Pointer to the global system page used for system calls and other
/// nice things.  
#[allow(unused)]
pub const AT_SYSINFO: usize = 32;
///
#[allow(unused)]
pub const AT_SYSINFO_EHDR: usize = 33;
