///
/// WorkerTask
///     - Represents a task to be executed by a worker
///     - We can add any WorkerTask realated code here like ( preprocessing, postprocessing, etc )
///     - `task_fn` is a closure that takes no arguments and returns nothing (`FnOnce`) in case we want to change the function signature in the future or add more arguments
///     - Task id functionality could be added later to track tasks
///
pub struct WorkerTask {
    task_fn: Box<dyn FnOnce() + Send + 'static>,
}

impl WorkerTask {
    /// Constructor
    pub fn new(task_fn: Box<dyn FnOnce() + Send + 'static>) -> Self {
        WorkerTask { task_fn }
    }

    /// Run the task
    ///     - take ownership of `self` and call the task function
    ///     - `self` is consumed, so calling `task_fn` is valid here because task can't run multiple times
    pub fn run(self) {
        (self.task_fn)();
    }
}
