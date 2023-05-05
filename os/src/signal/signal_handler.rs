use crate::{process::thread::exit_and_terminate_all_threads, stack_trace};

pub fn term_sig_handler(signo: usize) {
    stack_trace!();
    // TODO: not sure what the exit code should be here
    exit_and_terminate_all_threads(-1);
}

pub fn ign_sig_handler(signo: usize) {
    stack_trace!();
}

pub fn core_sig_handler(signo: usize) {
    stack_trace!();
    // TODO: not sure what the exit code should be here
    exit_and_terminate_all_threads(-1);
    // TODO: add core dump ?
}

pub fn stop_sig_handler(signo: usize) {
    // TODO: add process `stop` state
    todo!()
}

pub fn cont_sig_handler(signo: usize) {
    // TODO: implement here when finishing `stop_sig_handler`
    todo!()
}

pub fn default_sig_handler(signo: usize) {
    // Nothing to do here
}
