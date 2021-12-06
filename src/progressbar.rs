//! A progress bar implementation.
//!
//! version 0.1.1
//! https://github.com/wufangjie/utils/blob/main/src/progressbar.rs
//!
//! struct ProgressBar (use new(total), goto(i), quit() to control progress)
//! struct IterPro (use IterPro::new(iter) to print progress)
//! trait Progress (use iter.progress() to print progress)

use std::io::{self, Write};

pub struct ProgressBar {
    total: usize,
    nchar: usize,
    pre_p: usize,
}

impl ProgressBar {
    pub fn new(total: usize) -> Self {
        ProgressBar {
            total,
            nchar: 50,
            pre_p: 0,
        }
    }

    pub fn goto(&mut self, p: usize) {
        let percent = 100 * p / self.total;
        if percent != self.pre_p {
            self.pre_p = percent;
            let np = percent * self.nchar / 100;
            let (ns, arrow) = if np == self.nchar {
                (0, "")
            } else {
                (self.nchar - np - 1, ">")
            };
            print!(
                "{:\x08>nt$}|{:=>np$}{}{: >ns$}|[{: >3}%]",
                "",
                "",
                arrow,
                "",
                percent,
                nt = self.nchar + 8,
                np = np,
                ns = ns
            );
            io::stdout().flush().unwrap();
        }
    }

    pub fn quit(&mut self) {
        self.goto(self.total);
        println!("");
    }
}

pub struct IterPro<I: Iterator<Item = T>, T> {
    bar: ProgressBar,
    iter: I,
    count: usize,
}

impl<I, T> IterPro<I, T>
where
    I: Iterator<Item = T>,
{
    pub fn new(iter: I) -> Self {
        let (mut total, hi) = iter.size_hint();
        if let Some(hi) = hi {
            total = hi;
        }
        IterPro {
            bar: ProgressBar::new(total),
            iter,
            count: 0,
        }
    }
}

impl<I, T> Iterator for IterPro<I, T>
where
    I: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.bar.goto(self.count);
        let ret = self.iter.next();
        self.count += 1;
        if ret.is_none() {
            self.bar.quit();
        }
        ret
    }
}

pub trait Progress<I, T>
where
    I: Iterator<Item = T>,
{
    fn progress(self) -> IterPro<I, T>;
}

impl<I, T> Progress<I, T> for I
where
    I: Iterator<Item = T>,
{
    fn progress(self) -> IterPro<I, T> {
        IterPro::new(self)
    }
}

#[cfg(test)]
mod test_pb {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_pb() {
        // import ProgressBar struct
        let iter = (0..101).into_iter();
        let n = iter.len();
        let mut bar = ProgressBar::new(n);
        for i in iter {
            bar.goto(i);
            thread::sleep(Duration::from_millis(50));
        }
        bar.quit();
    }

    #[test]
    fn test_pb2() {
        // import IterPro struct
        for _ in IterPro::new((0..101).into_iter().take(60)) {
            thread::sleep(Duration::from_millis(50));
        }
    }

    #[test]
    fn test_pb3() {
        // import Progress trait
        for _ in (0..101).into_iter().skip(40).progress() {
            thread::sleep(Duration::from_millis(50));
        }
    }
}
