use super::wave::Wave;

pub struct Playback<W>
where
    W: Wave,
{
    wave: W,
    n: usize,
}

impl<W> Playback<W>
where
    W: Wave,
{
    pub fn new(wave: W, n: usize) -> Playback<W> {
        Playback { wave, n }
    }
}

impl<W> Iterator for Playback<W>
where
    W: Wave,
{
    type Item = W::Frame;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            return None;
        }
        self.n -= 1;
        Some(self.wave.next())
    }
}
