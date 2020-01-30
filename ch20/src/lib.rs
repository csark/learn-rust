pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }

    pub fn exexute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        
    }
}
