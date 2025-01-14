



///
/// Create a new `WorkerTask`
///     - we can inject any WorkerTask realated code here like ( preprocessing, postprocessing, etc )
///     - `task_fn` is a closure that takes no arguments and returns nothing (`FnOnce`) 
///       incase we want to change the function signature in the future or add more arguments
///     - Task id functionality could be added later to track tasks
///
pub struct WorkerTask {
    task_fn: Box<dyn FnOnce() + Send + 'static>,
}


impl WorkerTask {
    pub fn new(task_fn: Box<dyn FnOnce() + Send + 'static>) -> Self {
        WorkerTask { task_fn }
    }

    /// Run the task
    ///     - take ownership of `self`
    ///     - call `self.task_fn`
    ///     - `self` is consumed, so calling `task_fn` is valid
    pub fn run(self) { // Take ownership of `self`
        (self.task_fn)(); // `self` is consumed, so calling `task_fn` is valid
    }
}