//! Scales, biquads, weighting, and correlation used by the meter.

pub fn scale_ends(kind: i32) -> (f32, f32) {
    match kind {
        // RMS (1) is absolute dBFS and covers the reference meter's range; the K scales (2-4) are relative to their reference.
        1 => (-40.0, 4.0),
        2 | 3 | 4 => (-24.0, 12.0),
        5 => (-50.0, 5.0),
        6 => (-36.0, 9.0),
        // BBC marks 1..7 step 4 dB with mark 4 at the reference (reference meter manual, scale chart).
        7 => (-12.0, 12.0),
        8 => (-12.0, 12.0),
        _ => (-20.0, 3.0),
    }
}

pub fn lamp_index(kind: i32, db: f32) -> i32 {
    let (lo, hi) = scale_ends(kind);
    let t = ((db - lo) / (hi - lo)).clamp(0.0, 1.0);
    (t * 39.0).round() as i32
}

/// Index of the scale cell nearest 0 on EBU (8) and Nordic (6), where the calibration TEST mark sits.
pub fn test_mark_index(kind: i32) -> Option<i32> {
    if kind != 6 && kind != 8 { return None; }
    let (lo, hi) = scale_ends(kind);
    (0..40).min_by(|a, b| {
        let da = (lo + (hi - lo) * (*a as f32 / 39.0)).abs();
        let db = (lo + (hi - lo) * (*b as f32 / 39.0)).abs();
        da.total_cmp(&db)
    })
}

/// The lamp that shows a reading, or None while the display is at rest. On EBU (8) a burst only moves the
/// display when the reading is within 12 dB below the lowest mark (Tech 3205-E 3.6: a 10 ms burst at -21 dB
/// gives a just-perceptible movement); the other types keep the -80 dB floor.
pub fn lit_lamp(kind: i32, db: f32) -> Option<i32> {
    let floor = if kind == 8 { scale_ends(kind).0 - 12.0 } else { -80.0 };
    if db < floor { None } else { Some(lamp_index(kind, db)) }
}

pub fn scale_caption(kind: i32, index: i32) -> String {
    let (lo, hi) = scale_ends(kind);
    let db = lo + (hi - lo) * (index as f32 / 39.0);
    if test_mark_index(kind) == Some(index) { return "TEST".into(); }
    if kind == 7 {
        let mark = 1.0 + (db - lo) / (hi - lo) * 6.0;
        return format!("{mark:.1}");
    }
    if db.abs() < 0.05 { return "0".into(); }
    if db > 0.0 { format!("+{db:.1}") } else { format!("{db:.1}") }
}

#[derive(Clone, Copy, Default)]
pub struct Biquad {
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a1: f32,
    pub a2: f32,
    pub z1: f32,
    pub z2: f32,
}

impl Biquad {
    pub fn lowpass(hz: f32, q: f32, sr: f32) -> Self { Self::rbj(hz, q, sr, 0) }
    pub fn highpass(hz: f32, q: f32, sr: f32) -> Self { Self::rbj(hz, q, sr, 1) }
    pub fn bandpass(hz: f32, q: f32, sr: f32) -> Self { Self::rbj(hz, q, sr, 2) }

    /// Bilinear transform of an analogue biquad whose coefficients use
    /// frequency in Hz (rather than radians/second) in descending powers of
    /// s: b2*s^2 + b1*s + b0 over a2*s^2 + a1*s + a0.
    pub fn from_analog(num: [f64; 3], den: [f64; 3], sr: f32) -> Self {
        let k = sr as f64 / std::f64::consts::PI;
        let norm = den[0] * k * k + den[1] * k + den[2];
        let b0 = (num[0] * k * k + num[1] * k + num[2]) / norm;
        let b1 = 2.0 * (num[2] - num[0] * k * k) / norm;
        let b2 = (num[0] * k * k - num[1] * k + num[2]) / norm;
        let a1 = 2.0 * (den[2] - den[0] * k * k) / norm;
        let a2 = (den[0] * k * k - den[1] * k + den[2]) / norm;
        Self { b0: b0 as f32, b1: b1 as f32, b2: b2 as f32, a1: a1 as f32, a2: a2 as f32, z1: 0.0, z2: 0.0 }
    }

    /// Recover an analogue prototype from coefficients specified at 48 kHz,
    /// then remap it to the requested sample rate using the bilinear transform.
    pub fn remap_48k(b: [f64; 3], a: [f64; 3], sr: f32) -> Self {
        let k = 48_000.0_f64 / std::f64::consts::PI;
        let num = [b[0] - b[1] + b[2], 2.0 * k * (b[0] - b[2]), k * k * (b[0] + b[1] + b[2])];
        let den = [1.0 - a[1] + a[2], 2.0 * k * (1.0 - a[2]), k * k * (1.0 + a[1] + a[2])];
        Self::from_analog(num, den, sr)
    }

    fn rbj(hz: f32, q: f32, sr: f32, kind: i32) -> Self {
        let w = std::f32::consts::TAU * (hz / sr).clamp(0.001, 0.45);
        let cs = w.cos();
        let sn = w.sin();
        let alpha = sn / (2.0 * q.max(0.2));
        let (b0, b1, b2, a0, a1, a2) = match kind {
            1 => ((1.0 + cs) * 0.5, -(1.0 + cs), (1.0 + cs) * 0.5, 1.0 + alpha, -2.0 * cs, 1.0 - alpha),
            2 => (sn * 0.5, 0.0, -sn * 0.5, 1.0 + alpha, -2.0 * cs, 1.0 - alpha),
            _ => ((1.0 - cs) * 0.5, 1.0 - cs, (1.0 - cs) * 0.5, 1.0 + alpha, -2.0 * cs, 1.0 - alpha),
        };
        Self { b0: b0 / a0, b1: b1 / a0, b2: b2 / a0, a1: a1 / a0, a2: a2 / a0, z1: 0.0, z2: 0.0 }
    }

    pub fn process(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.z1;
        self.z1 = self.b1 * x - self.a1 * y + self.z2;
        self.z2 = self.b2 * x - self.a2 * y;
        y
    }
}

/// Build standards-based IEC A/B/C/D or ITU-R BS.1770 K weighting sections.
/// Sections are returned in signal-flow order and normalized near 1 kHz.
pub fn weighting_sections(kind: i32, sr: f32) -> ([Biquad; 6], usize, f32) {
    let mut sections = [Biquad::default(); 6];
    let mut count = 0;
    let mut push = |section: Biquad| { sections[count] = section; count += 1; };
    let prewarp = |hz: f64| sr as f64 / std::f64::consts::PI * (std::f64::consts::PI * hz / sr as f64).tan();
    let hp = |hz: f64| { let hz = prewarp(hz); Biquad::from_analog([0.0, 1.0, 0.0], [0.0, 1.0, hz], sr) };
    let lp = |hz: f64| { let hz = prewarp(hz); Biquad::from_analog([0.0, 0.0, hz], [0.0, 1.0, hz], sr) };
    match kind {
        1 => {
            push(hp(20.6)); push(hp(20.6)); push(hp(107.7)); push(hp(737.9));
            push(lp(12194.0)); push(lp(12194.0));
        }
        2 => {
            push(hp(20.6)); push(hp(20.6)); push(hp(158.5));
            push(lp(12194.0)); push(lp(12194.0));
        }
        3 => {
            push(hp(20.6)); push(hp(20.6)); push(lp(12194.0)); push(lp(12194.0));
        }
        4 => {
            // IEC D-weighting transfer function (IEC 537), factored into
            // first-order poles and the prescribed second-order pole pair.
            push(hp(79919.29_f64.sqrt()));
            push(lp(1345600.0_f64.sqrt()));
            let prewarp_pair = |linear: f64, constant: f64| {
                let natural = constant.sqrt();
                let q = natural / linear;
                let warped = prewarp(natural);
                (warped / q, warped * warped)
            };
            let (n1, n0) = prewarp_pair(1080768.16_f64.sqrt(), 1037918.48);
            let (d1, d0) = prewarp_pair(11723776.0_f64.sqrt(), 9837328.0);
            push(Biquad::from_analog(
                [1.0, n1, n0], [1.0, d1, d0], sr));
        }
        5 => {
            // ITU-R BS.1770-4 Table 1/2 K-weighting stages at 48 kHz.
            push(Biquad::remap_48k(
                [1.53512485958697, -2.69169618940638, 1.19839281085285],
                [1.0, -1.69065929318241, 0.73248077421585], sr));
            push(Biquad::remap_48k(
                [1.0, -2.0, 1.0], [1.0, -1.99004745483398, 0.99007225036621], sr));
        }
        _ => {}
    }
    // Normalize each standards curve to 0 dB at 1 kHz. The normalization is
    // scalar and therefore does not alter the shape.
    let mut response = 1.0_f64;
    for section in sections.iter().take(count) {
        let w = std::f64::consts::TAU * 1000.0 / sr as f64;
        let (c, s) = (w.cos(), w.sin());
        let nr = section.b0 as f64 + section.b1 as f64 * c + section.b2 as f64 * (2.0*c*c-1.0);
        let ni = -(section.b1 as f64 * s + section.b2 as f64 * 2.0*s*c);
        let dr = 1.0 + section.a1 as f64*c + section.a2 as f64*(2.0*c*c-1.0);
        let di = -(section.a1 as f64*s + section.a2 as f64*2.0*s*c);
        response *= (nr*nr+ni*ni).sqrt()/(dr*dr+di*di).sqrt();
    }
    (sections, count, (1.0 / response.max(1.0e-20)) as f32)
}

/// One-pole highpass used as a 6 dB stage. State is the lowpass memory.
pub fn one_pole_hpf(state: &mut f32, x: f32, hz: f32, sr: f32) -> f32 {
    let c = (1.0 - (-std::f32::consts::TAU * hz / sr).exp()).clamp(0.0, 1.0);
    *state += c * (x - *state);
    x - *state
}

pub fn one_pole_lpf(state: &mut f32, x: f32, hz: f32, sr: f32) -> f32 {
    let c = (1.0 - (-std::f32::consts::TAU * hz / sr).exp()).clamp(0.0, 1.0);
    *state += c * (x - *state);
    *state
}

/// Cascade 6 dB stages. `poles` is 0..4. Highpass when `hp` is true.
pub fn cascade(states: &mut [f32], mut x: f32, hz: f32, sr: f32, poles: i32, hp: bool) -> f32 {
    for state in states.iter_mut().take(poles.clamp(0, 4) as usize) {
        x = if hp { one_pole_hpf(state, x, hz, sr) } else { one_pole_lpf(state, x, hz, sr) };
    }
    x
}

/// Correlation coefficient in −1..1 mapped onto the phase scale.
/// 1 → 0, 0 → −20, −1 → −39.
pub fn correlation_db(coeff: f32) -> f32 {
    let c = coeff.clamp(-1.0, 1.0);
    if c >= 0.0 { -20.0 * (1.0 - c) } else { -20.0 + c * 19.0 }
}
