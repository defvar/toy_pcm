use super::phase::{phase, Phase};

pub fn rate(rate: f64) -> Rate {
    Rate { rate }
}

#[derive(Clone)]
pub struct Rate {
    rate: f64,
}

pub trait Signal {
    fn value(&self) -> f64;
}

#[derive(Clone)]
pub struct ConstHz {
    hz: f64,
}

#[derive(Clone)]
pub struct CalcHz {
    hz: f64,
    rate: f64,
}

impl Rate {
    pub fn const_hz(&self, hz: f64) -> ConstHz {
        ConstHz { hz: hz / self.rate }
    }
}

impl ConstHz {
    pub fn phase(self) -> Phase<Self> {
        phase(self)
    }
}

impl CalcHz {
    pub fn phase(self) -> Phase<CalcHz> {
        phase(self)
    }
}

impl Signal for ConstHz {
    #[inline]
    fn value(&self) -> f64 {
        self.hz
    }
}

impl Signal for CalcHz {
    #[inline]
    fn value(&self) -> f64 {
        self.hz / self.rate
    }
}
