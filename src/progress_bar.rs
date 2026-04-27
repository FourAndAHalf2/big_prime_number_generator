use tqdm::tqdm;
use crate::settings;

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