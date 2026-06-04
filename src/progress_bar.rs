use tqdm::tqdm;


pub struct ProgressBar {
    show: bool,
}

impl ProgressBar {
    pub fn new(show: bool) -> ProgressBar {
        ProgressBar { show }
    }

    pub fn iter<I>(&self, iterable: I) -> Box<dyn Iterator<Item = I::Item>>
    where
        I: IntoIterator,
        I::IntoIter: 'static,
        I::Item: 'static,
    {
        let iter = iterable.into_iter();

        if self.show {
            Box::new(tqdm(iter))
        } else {
            Box::new(iter)
        }
    }
}

impl<I> std::ops::FnOnce<(I,)> for ProgressBar
where
    I: IntoIterator,
    I::IntoIter: 'static,
    I::Item: 'static,
{
    type Output = Box<dyn Iterator<Item = I::Item>>;

    extern "rust-call" fn call_once(self, args: (I,)) -> Self::Output {
        self.iter(args.0)
    }
}

impl<I> std::ops::FnMut<(I,)> for ProgressBar
where
    I: IntoIterator,
    I::IntoIter: 'static,
    I::Item: 'static,
{
    extern "rust-call" fn call_mut(&mut self, args: (I,)) -> Self::Output {
        self.iter(args.0)
    }
}

impl<I> std::ops::Fn<(I,)> for ProgressBar
where
    I: IntoIterator,
    I::IntoIter: 'static,
    I::Item: 'static,
{
    extern "rust-call" fn call(&self, args: (I,)) -> Self::Output {
        self.iter(args.0)
    }
}

