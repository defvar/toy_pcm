use super::hz::Signal;
use super::wave::{Saw, Sine};

pub fn phase<H: Signal>(hz: H) -> Phase<H> {
    Phase { next: 0.0, hz }
}

#[derive(Clone)]
pub struct Phase<H>
where
    H: Signal,
{
    next: f64,
    hz: H,
}

impl<H> Phase<H>
where
    H: Signal,
{
    pub fn step(&self) -> f64 {
        self.hz.value()
    }

    #[inline]
    pub fn next_phase_wrapped_to(&mut self, rem: f64) -> f64 {
        let phase = self.next;
        self.next = (self.next + self.step()) % rem;
        phase
    }

    #[inline]
    pub fn next_phase(&mut self) -> f64 {
        self.next_phase_wrapped_to(1.0)
    }

    #[inline]
    pub fn sine(self) -> Sine<H> {
        Sine::new(self)
    }

    #[inline]
    pub fn saw(self) -> Saw<H> {
        Saw::new(self)
    }
}
