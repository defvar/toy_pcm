use super::hz::Signal;
use super::phase::Phase;

pub trait Wave {
    type Frame: Sized;
    fn next(&mut self) -> Self::Frame;
}

#[derive(Clone)]
pub struct Sine<H>
where
    H: Signal,
{
    phase: Phase<H>,
}

#[derive(Clone)]
pub struct Saw<H>
where
    H: Signal,
{
    phase: Phase<H>,
}

impl<H> Sine<H>
where
    H: Signal,
{
    pub fn new(phase: Phase<H>) -> Sine<H> {
        Sine { phase }
    }
}

impl<H> Saw<H>
where
    H: Signal,
{
    pub fn new(phase: Phase<H>) -> Saw<H> {
        Saw { phase }
    }
}

impl<H> Wave for Sine<H>
where
    H: Signal,
{
    type Frame = [f64; 1];

    #[inline]
    fn next(&mut self) -> Self::Frame {
        const PI_2: f64 = ::std::f64::consts::PI * 2.0;
        let phase = self.phase.next_phase();
        [(PI_2 * phase).sin()]
    }
}

impl<H> Wave for Saw<H>
where
    H: Signal,
{
    type Frame = [f64; 1];

    #[inline]
    fn next(&mut self) -> Self::Frame {
        let phase = self.phase.next_phase();
        let f = if phase == 0.0 {
            0.0
        } else {
            phase * -2.0 + 1.0
        };
        [f]
    }
}
