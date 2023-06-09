# syscall

## execve

- All threads other than the calling thread are destroyed during an `execve()`.  Mutexes, condition variables, and other pthreads objects are not preserved.
