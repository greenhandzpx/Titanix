use crate::trap::UserContext;

use super::SigSet;

pub struct SignalContext {
    pub blocked_sigs: SigSet,
    pub user_context: UserContext,
}
