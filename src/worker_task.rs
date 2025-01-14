
pub struct WorkerTask {
    task_fn: Box<dyn FnOnce() + Send + 'static>,
}

impl WorkerTask {
    pub fn new(task_fn: Box<dyn FnOnce() + Send + 'static>) -> Self {
        WorkerTask { task_fn }
    }

    pub fn run(self) { // Take ownership of `self`
        (self.task_fn)(); // `self` is consumed, so calling `task_fn` is valid
    }
}