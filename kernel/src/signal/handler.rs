use log::debug;

use crate::{process::thread::exit_and_terminate_all_threads, stack_trace};

pub const SIG_ERR: usize = usize::MAX;
pub const SIG_DFL: usize = 0;
pub const SIG_IGN: usize = 1;

pub fn term_sig_handler(signo: usize) {
    stack_trace!();
    log::info!("term sig handler, sig {}", signo);
    // TODO: not sure what the exit code should be here
    exit_and_terminate_all_threads(-1);
}

pub fn ign_sig_handler(signo: usize) {
    stack_trace!();
    debug!("ignore this sig {}", signo);
}

pub fn core_sig_handler(signo: usize) {
    stack_trace!();
    log::info!("core sig handler, sig {}", signo);
    // TODO: not sure what the exit code should be here
    exit_and_terminate_all_threads(-1);
    // exit_and_terminate_all_threads(0);
    // terminate_given_thread(current_task().tid(), 0);

    // TODO: add core dump ?
}

pub fn stop_sig_handler(_signo: usize) {
    // TODO: add process `stop` state
    todo!()
}

#[allow(unused)]
pub fn cont_sig_handler(_signo: usize) {
    // TODO: implement here when finishing `stop_sig_handler`
    todo!()
}

#[allow(unused)]
pub fn default_sig_handler(signo: usize) {
    // Nothing to do here
    stack_trace!();
    debug!("default handler for this sig {}", signo);
}
