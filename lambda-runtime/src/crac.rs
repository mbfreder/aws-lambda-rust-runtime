/// A trait for receiving checkpoint/restore notifications.
///
/// The type that is interested in receiving a checkpoint/restore notification
/// implements this trait, and the instance created from that type is registered
/// inside the Runtime's list of resources, using the Runtime's register() method.
pub trait Resource {
    /// Invoked by Runtime as a notification about checkpoint (that snapshot is about to be taken)
    fn before_checkpoint(&self);
    /// Invoked by Runtime as a notification about restore (snapshot was restored)
    fn after_restore(&self);
}

// implement a no-op Resource for ()
impl Resource for () {
    fn before_checkpoint(&self) {
        // no op
    }
    fn after_restore(&self) {
        // no op
    }
}
