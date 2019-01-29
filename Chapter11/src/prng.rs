const S1_MOD: f32 = 30269f32;
const S2_MOD: f32 = 30307f32;
const S3_MOD: f32 = 30323f32;

pub struct WichmannHillRng {
    s1: f32,
    s2: f32,
    s3: f32,
}

impl WichmannHillRng {
    fn new(s1: f32, s2: f32, s3: f32) -> WichmannHillRng {
        WichmannHillRng {
            s1: s1,
            s2: s2,
            s3: s3,
        }
    }

    pub fn seeded(seed: u32) -> WichmannHillRng {
        let t = seed;
        let s1 = (t % 29999) as f32;
        let s2 = (t % 29347) as f32;
        let s3 = (t % 29097) as f32;
        WichmannHillRng::new(s1, s2, s3)
    }

    pub fn next_f32(&mut self) -> f32 {
        self.s1 = (171f32 * self.s1) % S1_MOD;
        self.s2 = (172f32 * self.s2) % S2_MOD;
        self.s3 = (170f32 * self.s3) % S3_MOD;
        (self.s1 / S1_MOD + self.s2 / S2_MOD + self.s3 / S3_MOD) % 1f32
    }
}

pub struct LCG {
    xn: f32,
    m: f32,
    c: f32,
    a: f32,
}

impl LCG {
    fn seeded(seed: f32) -> LCG {
        LCG {
            xn: seed,
            // glibc defaults according to wikipedia
            m: 2e31,
            a: 1103515245f32,
            c: 12345f32,
        }
    }

    fn new(seed: f32, m: f32, a: f32, c: f32) -> LCG {
        LCG {
            xn: seed,
            m: m,
            a: a,
            c: c,
        }
    }

    fn next(&mut self) -> f32 {
        self.xn = (self.a * self.xn + self.c) % self.m;
        self.xn
    }
}
