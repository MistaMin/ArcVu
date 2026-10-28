//! Clean-room meter detector.
//!
//! The average follows a full-wave rectifier calibrated so a sine's reading
//! equals its RMS, then a lightly underdamped galvanometer. A step reaches
//! about 99 percent in 300 ms and overshoots by about 1 percent, which is the
//! published VU instrument behaviour (ANSI C16.5 / IEC 60268-17). Rise and
//! fall share that movement. There is no separate attack and release.
//!
//! The peak is a true peak on the same 0 VU reference: instant attack, a hold,
//! then a release. It is not fed through the galvanometer.

use std::os::raw::{c_float, c_int};

mod extra;
pub mod loudness;
mod loudness_panel;

use goodlookindri::color::Color;
use goodlookindri::scene::{Point, Rect, Scene};

const ZETA: f32 = 0.82;
const WN: f32 = 14.0;
const SINE_TO_RMS: f32 = std::f32::consts::PI / (2.0 * std::f32::consts::SQRT_2);
const PEAK_RELEASE_SECONDS: f32 = 1.6;
const FOLLOW_RELEASE_SECONDS: f32 = 0.05;
const MAX_CHANNELS: usize = 2;

#[derive(Clone, Copy)]
struct Channel {
    x: f32,
    v: f32,
    peak: f32,
    numeric_peak: f32,
    numeric_peak_hold_t: f32,
    hold: f32,
    over_run: u32,
    rms: f32,
    ppm: f32,
    ppm_slow: f32,
    ppm_fast: f32,
    ppm_third: f32,
    ppm_slow_out: f32,
    meter_hold: f32,
    meter_hold_t: f32,
    clip_t: f32,
    hp: [f32; 4],
    lp: [f32; 4],
}

impl Channel {
    fn silent() -> Self {
        Self { x: 1.0e-9, v: 0.0, peak: 1.0e-9, numeric_peak: 1.0e-9, numeric_peak_hold_t: 0.0, hold: 0.0, over_run: 0, rms: 1.0e-12, ppm: 1.0e-9, ppm_slow: 1.0e-18, ppm_fast: 1.0e-18, ppm_third: 1.0e-18, ppm_slow_out: 1.0e-9, meter_hold: -80.0, meter_hold_t: 0.0, clip_t: 0.0, hp: [0.0; 4], lp: [0.0; 4] }
    }
}

/// 0 = peak dot tracks the signal, 1 = hold for 3 seconds, 2 = hold until reset.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PeakMode { Follow = 0, Auto = 1, Hold = 2 }

pub struct Meter {
    sample_rate: f32,
    /// Sample-peak level, in dBFS, of the sine that reads 0.
    reference_db: f32,
    peak_mode: PeakMode,
    overs: u32,
    kind: i32,
    face: i32,
    single: bool,
    mid_side: bool,
    aes17: bool,
    rms_ms: f32,
    clip_db: f32,
    trim: [f32; 4],
    polarity: [bool; 2],
    mute: [bool; 2],
    monitor: i32,
    hpf_hz: f32,
    hpf_poles: i32,
    lpf_hz: f32,
    lpf_poles: i32,
    mono_hz: f32,
    mono_amt: f32,
    mono_lp: [f32; 2],
    dyn_hz: f32,
    dyn_depth: f32,
    dyn_on: bool,
    panel: i32,
    vu_model: i32,
    rise_ms: f32,
    fall_ms: f32,
    overshoot: f32,
    peak_hold_s: f32,
    value_hold_s: f32,
    clip_hold_s: f32,
    peak_thresh_db: f32,
    red_db: f32,
    comfort_db: f32,
    analog_rms: bool,
    weight: i32,
    trim_input: [f32; 4],
    trim_source_lr: usize,
    trim_source_ms: usize,
    force_mono: bool,
    trim_link: bool,
    in_place: bool,
    show_values: bool,
    slow_mode: bool,
    theme: i32,
    skin: i32,
    peak_scale: i32,
    loud: loudness::Loudness,
    loud_on: bool,
    loud_target: f32,
    loud_ceiling: f32,
    loud_scale: i32,
    loud_unit: i32,
    loud_window: f32,
    /// Measures what the plug-in outputs while an offline bounce is logged (see arcvu_bounce).
    bounce: loudness::Loudness,
    bounce_on: bool,
    bounce_frames: u64,
    bounce_rows: Vec<[f32; 5]>,
    show_hold: bool,
    show_tools: bool,
    show_label: bool,
    meters_only: bool,
    tooltips: bool,
    always_on_top: bool,
    drag_sens: f32,
    refresh_hz: i32,
    dyn_type: i32,
    dyn_listen: bool,
    gui_scale: i32,
    label: [u8; 32],
    correlation: f32,
    corr_xy: f32,
    corr_xx: f32,
    corr_yy: f32,
    dyn_env: f32,
    dyn_mem: [[f32; 4]; 4],
    weight_filters: [[extra::Biquad; 6]; 2],
    weight_filter_count: usize,
    weight_gain: f32,
    configured_weight: i32,
    configured_weight_sr: f32,
    bypass_hidden: bool,
    scratch: Vec<f32>,
    channels: [Channel; MAX_CHANNELS],
}

impl Meter {
    pub fn new(sample_rate: f64) -> Self {
        let mut meter = Self {
            sample_rate: 48_000.0,
            reference_db: -18.0,
            peak_mode: PeakMode::Auto,
            overs: 0,
            kind: 0,
            face: 0,
            single: false,
            mid_side: false,
            aes17: true,
            rms_ms: 300.0,
            clip_db: 0.0,
            trim: [0.0; 4],
            polarity: [false; 2],
            mute: [false; 2],
            monitor: 0,
            hpf_hz: 80.0,
            hpf_poles: 0,
            lpf_hz: 16000.0,
            lpf_poles: 0,
            mono_hz: 120.0,
            mono_amt: 0.0,
            mono_lp: [0.0; 2],
            dyn_hz: 4000.0,
            dyn_depth: 0.0,
            dyn_on: false,
            panel: 0,
            vu_model: 0,
            rise_ms: 300.0,
            fall_ms: 300.0,
            overshoot: 1.0,
            peak_hold_s: 3.0,
            value_hold_s: 5.0,
            clip_hold_s: 2.0,
            peak_thresh_db: -6.0,
            red_db: 0.0,
            comfort_db: 4.0,
            analog_rms: true,
            weight: 0,
            trim_input: [0.0; 4],
            trim_source_lr: 0,
            trim_source_ms: 2,
            force_mono: false,
            trim_link: false,
            in_place: true,
            show_values: true,
            slow_mode: false,
            theme: 0,
            skin: 0,
            peak_scale: 0,
            loud: loudness::Loudness::new(sample_rate),
            loud_on: false,
            loud_target: -14.0,
            loud_ceiling: -1.0,
            loud_scale: 0,
            loud_unit: 0,
            loud_window: 10.0,
            bounce: loudness::Loudness::new(sample_rate),
            bounce_on: false,
            bounce_frames: 0,
            bounce_rows: Vec::with_capacity(BOUNCE_ROWS),
            show_hold: true,
            show_tools: true,
            show_label: true,
            meters_only: false,
            tooltips: true,
            always_on_top: false,
            drag_sens: 1.0,
            refresh_hz: 30,
            dyn_type: 0,
            dyn_listen: false,
            gui_scale: 0,
            label: [0; 32],
            correlation: 0.0,
            corr_xy: 0.0,
            corr_xx: 1.0e-8,
            corr_yy: 1.0e-8,
            dyn_env: 0.0,
            dyn_mem: [[0.0; 4]; 4],
            weight_filters: [[extra::Biquad::default(); 6]; 2],
            weight_filter_count: 0,
            weight_gain: 1.0,
            configured_weight: -1,
            configured_weight_sr: 0.0,
            bypass_hidden: false,
            scratch: Vec::with_capacity(16384),
            channels: [Channel::silent(); MAX_CHANNELS],
        };
        meter.set_sample_rate(sample_rate);
        meter
    }

    pub fn set_sample_rate(&mut self, sample_rate: f64) {
        if (self.loud.sample_rate() - sample_rate).abs() > 0.5 { self.loud.set_sample_rate(sample_rate); self.bounce.set_sample_rate(sample_rate); self.bounce_frames = 0; self.bounce_rows.clear(); }
        self.sample_rate = sample_rate.clamp(8_000.0, 384_000.0) as f32;
        self.configure_weighting();
    }

    fn configure_weighting(&mut self) {
        if self.configured_weight == self.weight && self.configured_weight_sr == self.sample_rate { return; }
        let (sections, count, gain) = extra::weighting_sections(self.weight, self.sample_rate);
        self.weight_filters = [sections; 2];
        self.weight_filter_count = count;
        self.weight_gain = gain;
        self.configured_weight = self.weight;
        self.configured_weight_sr = self.sample_rate;
    }

    pub fn set_reference_db(&mut self, db: f32) {
        self.reference_db = db.clamp(-36.0, 0.0);
    }

    pub fn set_peak_mode(&mut self, mode: i32) {
        self.peak_mode = match mode {
            0 => PeakMode::Follow,
            2 => PeakMode::Hold,
            _ => PeakMode::Auto,
        };
    }

    pub fn reset_peaks(&mut self) {
        for channel in &mut self.channels {
            channel.peak = 1.0e-9;
            channel.numeric_peak = 1.0e-9;
            channel.numeric_peak_hold_t = 0.0;
            channel.hold = 0.0;
            channel.meter_hold = -80.0;
            channel.meter_hold_t = 0.0;
        }
    }

    pub fn reset_overs(&mut self) {
        self.overs = 0;
        for channel in &mut self.channels { channel.over_run = 0; }
    }

    pub fn reset_clip(&mut self) {
        for channel in &mut self.channels { channel.clip_t = 0.0; }
    }

    pub fn overs(&self) -> u32 { self.overs }

    pub fn apply_setup(&mut self, s: &ArcVuSetup) {
        self.kind = s.kind.clamp(0, 8);
        self.face = s.face.clamp(0, 2);
        self.single = s.single != 0;
        self.mid_side = s.mid_side != 0;
        self.aes17 = s.aes17 != 0;
        self.rms_ms = s.rms_ms.clamp(30.0, 2000.0);
        self.clip_db = s.clip_db.clamp(-12.0, 0.0);
        self.set_reference_db(s.reference_db);
        self.set_peak_mode(s.peak_mode);
        let incoming_trim = [s.trim_l, s.trim_r, s.trim_m, s.trim_s];
        let trim_link = s.trim_link != 0;
        if trim_link {
            let entering_link = !self.trim_link;
            let changed_l = incoming_trim[0] != self.trim_input[0];
            let changed_r = incoming_trim[1] != self.trim_input[1];
            let changed_m = incoming_trim[2] != self.trim_input[2];
            let changed_s = incoming_trim[3] != self.trim_input[3];
            if entering_link || (changed_l && changed_r) { self.trim_source_lr = 0; }
            else if changed_l != changed_r { self.trim_source_lr = if changed_l { 0 } else { 1 }; }
            if entering_link || (changed_m && changed_s) { self.trim_source_ms = 2; }
            else if changed_m != changed_s { self.trim_source_ms = if changed_m { 2 } else { 3 }; }
            self.trim = incoming_trim;
            self.trim[1 - self.trim_source_lr] = self.trim[self.trim_source_lr];
            self.trim[5 - self.trim_source_ms] = self.trim[self.trim_source_ms];
        } else {
            self.trim = incoming_trim;
            self.trim_source_lr = 0;
            self.trim_source_ms = 2;
        }
        self.trim_input = incoming_trim;
        self.polarity = [s.pol_l != 0, s.pol_r != 0];
        self.mute = [s.mute_l != 0, s.mute_r != 0];
        self.monitor = s.monitor.clamp(0, 5);
        self.hpf_hz = s.hpf_hz.clamp(20.0, 500.0);
        self.hpf_poles = s.hpf_poles.clamp(0, 4);
        self.lpf_hz = s.lpf_hz.clamp(1000.0, 20000.0);
        self.lpf_poles = s.lpf_poles.clamp(0, 4);
        self.mono_hz = s.mono_hz.clamp(20.0, 400.0);
        self.mono_amt = s.mono_amt.clamp(0.0, 1.0);
        self.dyn_hz = s.dyn_hz.clamp(80.0, 12000.0);
        self.dyn_depth = s.dyn_depth.clamp(0.0, 1.0);
        self.dyn_on = s.dyn_on != 0;
        self.panel = s.panel.clamp(0, 8);
        self.vu_model = s.vu_model.clamp(0, 2);
        self.rise_ms = s.rise_ms.clamp(50.0, 1000.0);
        self.fall_ms = s.fall_ms.clamp(50.0, 2000.0);
        self.overshoot = s.overshoot.clamp(0.2, 8.0);
        self.peak_hold_s = s.peak_hold_s;
        self.value_hold_s = s.value_hold_s;
        self.clip_hold_s = s.clip_hold_s;
        self.peak_thresh_db = s.peak_thresh_db.clamp(-24.0, 0.0);
        self.red_db = s.red_db.clamp(-12.0, 12.0);
        self.comfort_db = s.comfort_db.clamp(1.0, 12.0);
        self.analog_rms = s.analog_rms != 0;
        self.weight = s.weight.clamp(0, 5);
        self.configure_weighting();
        self.force_mono = s.force_mono != 0;
        self.trim_link = trim_link;
        self.in_place = s.in_place != 0;
        self.show_values = s.show_values != 0;
        self.show_hold = s.show_hold != 0;
        self.show_tools = s.show_tools != 0;
        self.show_label = s.show_label != 0;
        self.meters_only = s.meters_only != 0;
        self.dyn_type = s.dyn_type.clamp(0, 4);
        self.dyn_listen = s.dyn_listen != 0;
        self.gui_scale = s.gui_scale.clamp(0, 2);
        self.label = s.label;
        self.bypass_hidden = s.bypass_hidden != 0;
        self.tooltips = s.tooltips != 0;
        self.always_on_top = s.always_on_top != 0;
        self.drag_sens = s.drag_sens.clamp(0.25, 4.0);
        self.refresh_hz = s.refresh_hz.clamp(10, 60);
    }

    pub fn process(&mut self, interleaved: &[f32], channels: usize) {
        let channels = channels.clamp(1, MAX_CHANNELS);
        let frames = interleaved.len() / channels;
        let dt = 1.0 / self.sample_rate;
        let release_seconds = if self.peak_mode == PeakMode::Follow { FOLLOW_RELEASE_SECONDS } else { PEAK_RELEASE_SECONDS };
        let release = (-dt / release_seconds).exp();
        let configured_hold = if self.peak_hold_s < 0.0 { f32::MAX } else { self.peak_hold_s.max(0.0) };
        let hold_for = match self.peak_mode {
            PeakMode::Follow => 0.0,
            PeakMode::Auto => configured_hold,
            PeakMode::Hold => f32::MAX,
        };
        let (wn, zeta, fall_wn) = match self.vu_model {
            // Models 1 and 2 must also meet IEC 268-17 at their defaults. Model 1 is the standard response.
            1 => (WN, ZETA, WN),
            2 => {
                // Overshoot setting 1.0 gives the standard damping; higher settings overshoot more.
                let wn = 300.0 / self.rise_ms * WN;
                let fall_wn = 300.0 / self.fall_ms * WN;
                let zeta = (ZETA + 0.15 * (1.0 - self.overshoot)).clamp(0.45, 0.95);
                (wn, zeta, fall_wn)
            }
            _ => (WN, ZETA, WN),
        };
        let rms_a = 1.0 - (-1.0 / (self.rms_ms * 0.001 * self.sample_rate)).exp();
        let (ppm_atk, ppm_rel) = ppm_coeff(self.kind, self.sample_rate);
        // EBU (8): three amplitude stages (2.18 ms, 0.44 ms, 0.12 ms) fitted to Tech 3205 Table 2 and the 10 +/-2 ms integration time on the rectified signal.
        // Each stage releases 24 dB in 2.8 s (ITU-R BS.645-2 Table 1).
        let ebu_c1 = 1.0 - (-1.0 / (2.1834e-3 * self.sample_rate)).exp();
        let ebu_c2 = 1.0 - (-1.0 / (0.4374e-3 * self.sample_rate)).exp();
        let ebu_c3 = 1.0 - (-1.0 / (0.1167e-3 * self.sample_rate)).exp();
        let ebu_release = 10f32.powf(-24.0 / 2.8 / 20.0 / self.sample_rate);
        let ebu_slow_c = 1.0 - (-1.0 / (1.15 * self.sample_rate)).exp();
        let clip_lin = 10f32.powf(self.clip_db / 20.0);
        for frame in 0..frames {
            for ch in 0..channels {
                let sample = interleaved[frame * channels + ch];
                let rectified = sample.abs() * SINE_TO_RMS;
                let state = &mut self.channels[ch];
                let wn_now = if rectified < state.x { fall_wn } else { wn };
                let accel = wn_now * wn_now * (rectified - state.x) - (2.0 * zeta * wn_now) * state.v;
                state.v += accel * dt;
                state.x = (state.x + state.v * dt).max(1.0e-9);

                let mag = sample.abs().max(1.0e-9);
                if mag >= state.peak {
                    state.peak = mag;
                    state.hold = hold_for;
                } else if state.hold > 0.0 {
                    state.hold = (state.hold - dt).max(0.0);
                } else if self.peak_mode != PeakMode::Hold {
                    state.peak = (state.peak * release).max(mag);
                }
                if mag >= state.numeric_peak {
                    state.numeric_peak = mag;
                    state.numeric_peak_hold_t = if self.value_hold_s < 0.0 { 1.0e9 } else { self.value_hold_s.max(0.0) };
                } else if state.numeric_peak_hold_t > 0.0 {
                    state.numeric_peak_hold_t = (state.numeric_peak_hold_t - dt).max(0.0);
                } else {
                    state.numeric_peak = mag;
                }
                if mag > 1.0 {
                    state.over_run += 1;
                    if state.over_run == 3 { self.overs = self.overs.saturating_add(1); }
                } else {
                    state.over_run = 0;
                }
                state.rms += rms_a * (sample * sample - state.rms);
                state.rms = state.rms.max(1.0e-16);
                if self.kind == 8 {
                    // EBU type IIb: two power stages (9.0 ms, then 0.4 ms) fitted to Tech 3205 Table 2.
                    let level = sample.abs();
                    if level > state.ppm_slow { state.ppm_slow += ebu_c1 * (level - state.ppm_slow); } else { state.ppm_slow *= ebu_release; }
                    if state.ppm_slow > state.ppm_fast { state.ppm_fast += ebu_c2 * (state.ppm_slow - state.ppm_fast); } else { state.ppm_fast *= ebu_release; }
                    if state.ppm_fast > state.ppm_third { state.ppm_third += ebu_c3 * (state.ppm_fast - state.ppm_third); } else { state.ppm_third *= ebu_release; }
                    state.ppm = state.ppm_third.max(1.0e-9);
                    // Slow mode (Tech 3205-E Fig. 2): one-pole of 1.15 s on the amplitude, read only when slow_mode is set.
                    state.ppm_slow_out += ebu_slow_c * (state.ppm - state.ppm_slow_out);
                } else {
                    let coeff = if mag > state.ppm { ppm_atk } else { ppm_rel };
                    state.ppm += coeff * (mag - state.ppm);
                    state.ppm = state.ppm.max(1.0e-9);
                }
                let clip_hold = if self.clip_hold_s < 0.0 { 1.0e9 } else { self.clip_hold_s.max(0.05) };
                if mag >= clip_lin { state.clip_t = clip_hold; }
                else { state.clip_t = (state.clip_t - dt).max(0.0); }
            }
        }
        let held = [self.reading(0), self.reading(1)];
        for ch in 0..channels {
            let db = held[ch];
            let state = &mut self.channels[ch];
            if db >= state.meter_hold {
                state.meter_hold = db;
                state.meter_hold_t = if self.value_hold_s < 0.0 { 1.0e9 } else { self.value_hold_s.max(0.0) };
            } else if state.meter_hold_t > 0.0 {
                state.meter_hold_t -= dt * frames as f32;
            } else {
                state.meter_hold = db;
            }
        }
    }

    fn reading(&self, channel: usize) -> f32 {
        let state = &self.channels[channel.min(MAX_CHANNELS - 1)];
        let align_peak = 10f32.powf(self.reference_db / 20.0);
        let align_rms = align_peak * std::f32::consts::FRAC_1_SQRT_2;
        let rms_db = 10.0 * state.rms.log10() + if self.aes17 { 3.0103 } else { 0.0 };
        let rms_reading = match self.kind {
            // Plain RMS is an absolute level in dBFS (with AES-17 a 0 dBFS sine reads 0), like the reference meter; the K scales are relative.
            1 => rms_db,
            2 => rms_db - (-12.0),
            3 => rms_db - (-14.0),
            4 => rms_db - (-20.0),
            _ => rms_db - self.reference_db,
        };
        match self.kind {
            1..=4 if !self.analog_rms => rms_reading,
            1..=4 => 20.0 * (state.x / align_rms).log10() + match self.kind {
                1 => self.reference_db + if self.aes17 { 0.0 } else { -3.0103 },
                2 => 12.0 + self.reference_db,
                3 => 14.0 + self.reference_db,
                4 => 20.0 + self.reference_db,
                _ => 0.0,
            },
            8 if self.slow_mode => 20.0 * (state.ppm_slow_out.max(1.0e-9) / align_peak).log10(),
            5 | 6 | 7 | 8 => 20.0 * (state.ppm / align_peak).log10(),
            _ => 20.0 * (state.x / align_rms).log10(),
        }
    }

    pub fn process_io(&mut self, interleaved: &mut [f32], channels: usize) {
        let channels = channels.clamp(1, MAX_CHANNELS);
        let frames = interleaved.len() / channels;
        let sr = self.sample_rate;
        let gl = db_lin(self.trim[0]);
        let gr = db_lin(self.trim[1]);
        let gm = db_lin(self.trim[2]);
        let gs = db_lin(self.trim[3]);
        let hp_c = one_pole_coeff(self.hpf_hz, sr);
        let lp_c = one_pole_coeff(self.lpf_hz, sr);
        let mono_c = one_pole_coeff(self.mono_hz, sr);
        let dyn_c = one_pole_coeff(self.dyn_hz, sr);
        // Band-pass edges: narrow uses two poles at /1.5 and x1.5, wide uses one pole at /4 and x4.
        let bpf_narrow = (one_pole_coeff(self.dyn_hz / 1.5, sr), one_pole_coeff(self.dyn_hz * 1.5, sr));
        let bpf_wide = (one_pole_coeff(self.dyn_hz / 4.0, sr), one_pole_coeff(self.dyn_hz * 4.0, sr));
        let run_fx = !(self.meters_only && self.bypass_hidden);
        self.scratch.clear();
        for frame in 0..frames {
            self.loud.process(interleaved[frame * channels], if channels > 1 { interleaved[frame * channels + 1] } else { 0.0 }, channels > 1);
            let mut l = interleaved[frame * channels];
            let mut r = if channels > 1 { interleaved[frame * channels + 1] } else { l };
            if channels == 1 || self.force_mono { r = l; }
            if self.mid_side && channels > 1 {
                let m = 0.5 * (l + r) * gm * if self.polarity[0] { -1.0 } else { 1.0 };
                let s = 0.5 * (l - r) * gs * if self.polarity[1] { -1.0 } else { 1.0 };
                let m = if self.mute[0] { 0.0 } else { m };
                let s = if self.mute[1] { 0.0 } else { s };
                self.scratch.push(m);
                self.scratch.push(s);
                l = m + s;
                r = m - s;
            } else {
                l *= gl * if self.polarity[0] { -1.0 } else { 1.0 };
                r *= gr * if self.polarity[1] { -1.0 } else { 1.0 };
                if self.mute[0] { l = 0.0; }
                if self.mute[1] { r = 0.0; }
                self.scratch.push(l);
                if channels > 1 { self.scratch.push(r); }
            }
            if run_fx {
            l = cascade(&mut self.channels[0].hp, l, hp_c, self.hpf_poles, true);
            r = cascade(&mut self.channels[1].hp, r, hp_c, self.hpf_poles, true);
            l = cascade(&mut self.channels[0].lp, l, lp_c, self.lpf_poles, false);
            r = cascade(&mut self.channels[1].lp, r, lp_c, self.lpf_poles, false);
            }
            if run_fx && self.mono_amt > 0.0 && channels > 1 {
                let mid = 0.5 * (l + r);
                let side = 0.5 * (l - r);
                self.mono_lp[0] += mono_c * (side - self.mono_lp[0]);
                let low = self.mono_lp[0];
                let high = side - low;
                let side = high + low * (1.0 - self.mono_amt);
                l = mid + side;
                r = mid - side;
            }
            if run_fx && self.dyn_on && self.dyn_depth > 0.0 && self.dyn_type > 0 {
                let (bl, br) = match self.dyn_type {
                    1 => (cascade(&mut self.dyn_mem[0], l, dyn_c, 2, true), cascade(&mut self.dyn_mem[1], r, dyn_c, 2, true)),
                    2 => (cascade(&mut self.dyn_mem[2], l, dyn_c, 2, false), cascade(&mut self.dyn_mem[3], r, dyn_c, 2, false)),
                    _ => {
                        let ((hp_c, lp_c), poles) = if self.dyn_type == 3 { (bpf_narrow, 2) } else { (bpf_wide, 1) };
                        let ql = cascade(&mut self.dyn_mem[0], l, hp_c, poles, true);
                        let qr = cascade(&mut self.dyn_mem[1], r, hp_c, poles, true);
                        (cascade(&mut self.dyn_mem[2], ql, lp_c, poles, false), cascade(&mut self.dyn_mem[3], qr, lp_c, poles, false))
                    }
                };
                self.dyn_env += dyn_c * (0.5 * (bl.abs() + br.abs()) - self.dyn_env);
                let g = 1.0 - (self.dyn_env * 6.0).clamp(0.0, 1.0) * self.dyn_depth;
                if self.dyn_listen { l = bl; r = br; }
                else { l = (l - bl) + bl * g; r = (r - br) + br * g; }
            }
            self.corr_xx = self.corr_xx * 0.9995 + l * l;
            self.corr_yy = self.corr_yy * 0.9995 + r * r;
            self.corr_xy = self.corr_xy * 0.9995 + l * r;
            let denom = (self.corr_xx * self.corr_yy).sqrt().max(1.0e-12);
            self.correlation = extra::correlation_db(self.corr_xy / denom);
            match self.monitor {
                1 => std::mem::swap(&mut l, &mut r),
                2 => { if self.in_place { r = 0.0; } else { r = l; } }
                3 => { if self.in_place { l = 0.0; } else { l = r; } }
                4 => { let m = 0.5 * (l + r); l = m; r = m; }
                5 => { let s = 0.5 * (l - r); l = s; r = -s; }
                _ => {}
            }
            interleaved[frame * channels] = l;
            if channels > 1 { interleaved[frame * channels + 1] = r; }
        }
        if self.bounce_on {
            // The log is of what the plug-in outputs, so it measures after the trims, filters and monitor choice.
            let sr = self.sample_rate as f64;
            for frame in interleaved.chunks_exact(channels) {
                self.bounce.process(frame[0], if channels > 1 { frame[1] } else { 0.0 }, channels > 1);
                self.bounce_frames += 1;
                if (self.bounce_frames as f64 / sr).floor() > ((self.bounce_frames - 1) as f64 / sr).floor() && self.bounce_rows.len() < BOUNCE_ROWS {
                    let r = self.bounce.readings();
                    self.bounce_rows.push([(self.bounce_frames as f64 / sr) as f32, r.momentary, r.short_term, r.integrated, r.true_peak]);
                }
            }
        }
        if self.single && channels > 1 {
            let pairs = self.scratch.len() / 2;
            for i in 0..pairs {
                let other = self.scratch[i * 2 + 1];
                let mid = 0.5 * (self.scratch[i * 2] + other);
                self.scratch[i * 2] = mid;
                self.scratch[i * 2 + 1] = other;
            }
        }
        if self.weight > 0 {
            for (index, sample) in self.scratch.iter_mut().enumerate() {
                let channel = if channels > 1 { index % channels } else { 0 };
                let mut weighted = *sample;
                for section in self.weight_filters[channel].iter_mut().take(self.weight_filter_count) {
                    weighted = section.process(weighted);
                }
                *sample = weighted * self.weight_gain;
            }
        }
        let meter_ch = if self.mid_side && channels > 1 { 2 } else { channels };
        let samples = std::mem::take(&mut self.scratch);
        self.process(&samples, meter_ch);
        self.scratch = samples;
    }

    pub fn levels(&self, channel: usize) -> Levels {
        let state = self.channels[channel.min(MAX_CHANNELS - 1)];
        let align_peak = 10f32.powf(self.reference_db / 20.0);
        let vu_db = self.reading(channel);
        let to_peak_db = |peak: f32| match self.kind {
            2 => 20.0 * peak.log10() - (-12.0),
            3 => 20.0 * peak.log10() - (-14.0),
            4 => 20.0 * peak.log10() - (-20.0),
            _ => 20.0 * (peak / (align_peak * std::f32::consts::FRAC_1_SQRT_2)).log10(),
        };
        // Peak scale 1 reads the sample peak against full scale, so 0 dBFS is 0 on every meter type.
        let peak_db = if self.peak_scale == 1 { 20.0 * state.numeric_peak.max(1.0e-9).log10() } else { to_peak_db(state.numeric_peak) };
        let peak_lamp_db = to_peak_db(state.peak);
        Levels {
            vu_db,
            peak_db,
            vu_pos: extra::lamp_index(self.kind, vu_db) as f32 / 39.0,
            peak_pos: if peak_lamp_db >= self.peak_thresh_db { extra::lamp_index(self.kind, peak_lamp_db) as f32 / 39.0 } else { -1.0 },
            hold_db: state.meter_hold,
            clip: state.clip_t > 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Levels {
    pub vu_db: f32,
    pub peak_db: f32,
    pub vu_pos: f32,
    pub peak_pos: f32,
    pub hold_db: f32,
    pub clip: bool,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ArcVuSetup {
    pub kind: i32,
    pub face: i32,
    pub single: i32,
    pub mid_side: i32,
    pub aes17: i32,
    pub peak_mode: i32,
    pub rms_ms: f32,
    pub clip_db: f32,
    pub reference_db: f32,
    pub trim_l: f32,
    pub trim_r: f32,
    pub trim_m: f32,
    pub trim_s: f32,
    pub pol_l: i32,
    pub pol_r: i32,
    pub mute_l: i32,
    pub mute_r: i32,
    pub monitor: i32,
    pub hpf_hz: f32,
    pub hpf_poles: i32,
    pub lpf_hz: f32,
    pub lpf_poles: i32,
    pub mono_hz: f32,
    pub mono_amt: f32,
    pub dyn_hz: f32,
    pub dyn_depth: f32,
    pub dyn_on: i32,
    pub panel: i32,
    pub vu_model: i32,
    pub rise_ms: f32,
    pub fall_ms: f32,
    pub overshoot: f32,
    pub peak_hold_s: f32,
    pub value_hold_s: f32,
    pub clip_hold_s: f32,
    pub peak_thresh_db: f32,
    pub red_db: f32,
    pub comfort_db: f32,
    pub analog_rms: i32,
    pub weight: i32,
    pub force_mono: i32,
    pub trim_link: i32,
    pub in_place: i32,
    pub show_values: i32,
    pub show_hold: i32,
    pub show_tools: i32,
    pub show_label: i32,
    pub meters_only: i32,
    pub dyn_type: i32,
    pub dyn_listen: i32,
    pub gui_scale: i32,
    pub label: [u8; 32],
    pub drag_sens: f32,
    pub refresh_hz: i32,
    pub bypass_hidden: i32,
    pub tooltips: i32,
    pub always_on_top: i32,
}

fn db_lin(db: f32) -> f32 { 10f32.powf(db / 20.0) }

fn one_pole_coeff(hz: f32, sr: f32) -> f32 {
    (1.0 - (-std::f32::consts::TAU * hz / sr).exp()).clamp(0.0, 1.0)
}

fn cascade(state: &mut [f32; 4], x: f32, coeff: f32, poles: i32, highpass: bool) -> f32 {
    let mut y = x;
    for i in 0..poles.max(0).min(4) as usize {
        state[i] += coeff * (y - state[i]);
        y = if highpass { y - state[i] } else { state[i] };
    }
    y
}

fn ppm_coeff(kind: i32, sr: f32) -> (f32, f32) {
    let (attack_ms, decay_db, decay_s) = match kind {
        // Type I (ITU-R BS.645-2 Annex 2 Table 1): 5 ms integration, 20 dB in 1.7 s. DIN (5) and Nordic (6).
        5 | 6 => (5.0, 20.0, 1.7),
        // EBU (8): attack time constant fitted to Tech 3205 Table 2 (5 ms, 10 ms and 100 ms bursts).
        // Fall 24 dB in 2.8 s from ITU-R BS.645-2 Annex 2 Table 1.
        8 => (5.6, 24.0, 2.8),
        // BBC type IIa (ITU-R BS.645-2 Annex 2 Table 1): 10 ms integration, 26 dB in 3 s.
        7 => (10.0, 26.0, 3.0),
        _ => (10.0, 20.0, 1.5),
    };
    let atk = 1.0 - (-1.0 / (attack_ms * 0.001 * sr)).exp();
    let rel = 1.0 - (-decay_db / (20.0 * decay_s * sr) * std::f32::consts::LN_10).exp();
    (atk, rel)
}

/// GoodLookinUI VU scale: deflection follows voltage, and +3 VU is full scale.
pub fn vu_position_for_db(vu: f32) -> f32 {
    (10f32.powf(vu / 20.0) * 0.7079).clamp(0.0, 1.0)
}

#[repr(C)]
pub struct ArcVuLevels {
    pub vu_db: c_float,
    pub peak_db: c_float,
    pub vu_pos: c_float,
    pub peak_pos: c_float,
}

#[no_mangle]
pub extern "C" fn arcvu_create(sample_rate: f64) -> *mut Meter {
    Box::into_raw(Box::new(Meter::new(sample_rate)))
}

#[no_mangle]
pub extern "C" fn arcvu_destroy(meter: *mut Meter) {
    if !meter.is_null() {
        unsafe { drop(Box::from_raw(meter)) }
    }
}

#[no_mangle]
pub extern "C" fn arcvu_set_sample_rate(meter: *mut Meter, sample_rate: f64) {
    if let Some(meter) = unsafe { meter.as_mut() } {
        meter.set_sample_rate(sample_rate);
    }
}

#[no_mangle]
pub extern "C" fn arcvu_set_skin(meter: *mut Meter, skin: c_int) {
    let skin = skin.clamp(0, 1);
    SKIN.store(skin, std::sync::atomic::Ordering::Relaxed);
    if let Some(meter) = unsafe { meter.as_mut() } {
        meter.skin = skin;
    }
}

#[repr(C)]
pub struct ArcVuLoudSettings { pub on: c_int, pub target: c_float, pub ceiling: c_float, pub scale: c_int, pub unit: c_int, pub window: c_float }

/// Show the panel and set its target, true-peak ceiling, EBU scale (0 = +9, 1 = +18), unit (0 = LUFS, 1 = LU) and sliding window.
#[no_mangle]
pub extern "C" fn arcvu_set_loudness(meter: *mut Meter, settings: *const ArcVuLoudSettings) {
    let Some(s) = (unsafe { settings.as_ref() }) else { return };
    LOUDNESS.store((s.on != 0) as i32, std::sync::atomic::Ordering::Relaxed);
    if let Some(meter) = unsafe { meter.as_mut() } {
        meter.loud_on = s.on != 0;
        meter.loud_target = s.target.clamp(-40.0, -5.0);
        meter.loud_ceiling = s.ceiling.clamp(-12.0, 0.0);
        meter.loud_scale = s.scale.clamp(0, 1);
        meter.loud_unit = s.unit.clamp(0, 1);
        meter.loud_window = s.window.clamp(1.0, 30.0);
        meter.loud.set_window(meter.loud_window);
    }
}

#[no_mangle]
pub extern "C" fn arcvu_get_loudness(meter: *const Meter, out: *mut ArcVuLoudSettings) {
    if out.is_null() { return; }
    if let Some(meter) = unsafe { meter.as_ref() } {
        unsafe { *out = ArcVuLoudSettings { on: meter.loud_on as c_int, target: meter.loud_target, ceiling: meter.loud_ceiling, scale: meter.loud_scale, unit: meter.loud_unit, window: meter.loud_window }; }
    }
}

#[no_mangle]
pub extern "C" fn arcvu_loudness(meter: *const Meter, out: *mut loudness::Readings) {
    if out.is_null() { return; }
    if let Some(meter) = unsafe { meter.as_ref() } { unsafe { *out = meter.loud.readings(); } }
}

#[no_mangle]
pub extern "C" fn arcvu_reset_loudness(meter: *mut Meter) {
    if let Some(meter) = unsafe { meter.as_mut() } { meter.loud.reset(); }
}

/// Height the panel adds under the face (0 while it is off), so the editor can tell the face from the panel.
#[no_mangle]
pub extern "C" fn arcvu_loudness_height(face: c_int) -> c_float { loudness_height(face) }

/// A click at view coordinates: 0 = not on the panel, 1 = reset, 2 = a setting changed (read it back with arcvu_get_loudness).
#[no_mangle]
pub extern "C" fn arcvu_loudness_click(meter: *mut Meter, x: c_float, y: c_float) -> c_int {
    match unsafe { meter.as_mut() } { Some(meter) if meter.loud_on => loudness_panel::click(meter, x, y), _ => 0 }
}

/// One row per second of an offline bounce, ten hours at most.
const BOUNCE_ROWS: usize = 36_000;

#[no_mangle]
pub extern "C" fn arcvu_bounce(meter: *mut Meter, on: c_int) {
    if let Some(meter) = unsafe { meter.as_mut() } {
        if on != 0 && !meter.bounce_on {
            meter.bounce.reset();
            meter.bounce_frames = 0;
            meter.bounce_rows.clear();
        }
        meter.bounce_on = on != 0;
    }
}

#[no_mangle]
pub extern "C" fn arcvu_bounce_readings(meter: *const Meter, out: *mut loudness::Readings, frames: *mut u64) {
    if let Some(meter) = unsafe { meter.as_ref() } {
        unsafe {
            if !out.is_null() { *out = meter.bounce.readings(); }
            if !frames.is_null() { *frames = meter.bounce_frames; }
        }
    }
}

/// Copies up to `cap` rows of (seconds, momentary, short-term, integrated, true peak); returns the number copied.
#[no_mangle]
pub extern "C" fn arcvu_bounce_rows(meter: *const Meter, out: *mut c_float, cap: c_int) -> c_int {
    let Some(meter) = (unsafe { meter.as_ref() }) else { return 0 };
    if out.is_null() || cap <= 0 { return 0; }
    let n = meter.bounce_rows.len().min(cap as usize);
    let dest = unsafe { std::slice::from_raw_parts_mut(out, n * 5) };
    for (i, row) in meter.bounce_rows.iter().take(n).enumerate() { dest[i * 5..i * 5 + 5].copy_from_slice(row); }
    n as c_int
}

#[no_mangle]
pub extern "C" fn arcvu_set_peak_scale(meter: *mut Meter, scale: c_int) {
    if let Some(meter) = unsafe { meter.as_mut() } {
        meter.peak_scale = scale.clamp(0, 1);
    }
}

#[no_mangle]
pub extern "C" fn arcvu_set_theme(meter: *mut Meter, theme: c_int) {
    if let Some(meter) = unsafe { meter.as_mut() } {
        meter.theme = theme.clamp(0, THEME_COUNT - 1);
    }
}

#[no_mangle]
pub extern "C" fn arcvu_set_slow_mode(meter: *mut Meter, on: c_int) {
    if let Some(meter) = unsafe { meter.as_mut() } {
        meter.slow_mode = on != 0;
    }
}

#[no_mangle]
pub extern "C" fn arcvu_set_reference_db(meter: *mut Meter, db: c_float) {
    if let Some(meter) = unsafe { meter.as_mut() } {
        meter.set_reference_db(db);
    }
}

#[no_mangle]
pub extern "C" fn arcvu_setup(meter: *mut Meter, setup: *const ArcVuSetup) {
    if let (Some(meter), Some(setup)) = unsafe { (meter.as_mut(), setup.as_ref()) } {
        meter.apply_setup(setup);
    }
}

#[no_mangle]
pub extern "C" fn arcvu_set_peak_mode(meter: *mut Meter, mode: c_int) {
    if let Some(meter) = unsafe { meter.as_mut() } { meter.set_peak_mode(mode); }
}

#[no_mangle]
pub extern "C" fn arcvu_reset_peaks(meter: *mut Meter) {
    if let Some(meter) = unsafe { meter.as_mut() } { meter.reset_peaks(); }
}

#[no_mangle]
pub extern "C" fn arcvu_view_size(face: c_int, w: *mut c_float, h: *mut c_float) {
    let (vw, vh) = view_size(face);
    unsafe {
        if !w.is_null() { *w = vw; }
        if !h.is_null() { *h = vh; }
    }
}

#[no_mangle]
pub extern "C" fn arcvu_reset_clip(meter: *mut Meter) {
    if let Some(meter) = unsafe { meter.as_mut() } { meter.reset_clip(); }
}

#[no_mangle]
pub extern "C" fn arcvu_reset_overs(meter: *mut Meter) {
    if let Some(meter) = unsafe { meter.as_mut() } { meter.reset_overs(); }
}

#[no_mangle]
pub extern "C" fn arcvu_overs(meter: *const Meter) -> c_int {
    unsafe { meter.as_ref() }.map(|meter| meter.overs() as c_int).unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn arcvu_process(meter: *mut Meter, samples: *const c_float, frames: c_int, channels: c_int) {
    if samples.is_null() || frames <= 0 || channels <= 0 {
        return;
    }
    if let Some(meter) = unsafe { meter.as_mut() } {
        let count = frames as usize * channels as usize;
        meter.process_io(unsafe { std::slice::from_raw_parts_mut(samples as *mut c_float, count) }, channels as usize);
    }
}

/// The skin is a UI-wide setting, so the C entry point that sizes the window reads it from here.
static SKIN: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

pub fn view_size(face: i32) -> (f32, f32) {
    let (w, h) = view_size_for(face, SKIN.load(std::sync::atomic::Ordering::Relaxed));
    (w, h + loudness_height(face))
}

/// The loudness panel is a UI-wide setting too, so the window size reads it from here.
static LOUDNESS: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);

/// Height the loudness panel adds under a face, 0 while it is off.
pub fn loudness_height(face: i32) -> f32 {
    if LOUDNESS.load(std::sync::atomic::Ordering::Relaxed) == 0 { return 0.0; }
    loudness_panel::extra_height(view_size_for(face, SKIN.load(std::sync::atomic::Ordering::Relaxed)).0)
}

/// Classic needle meters need more height than the lamp ribbons on the arc face.
pub fn view_size_for(face: i32, skin: i32) -> (f32, f32) {
    match face {
        1 if skin == 1 => (1100.0, 340.0),
        1 => (1100.0, 292.0),
        2 => (380.0, 700.0),
        _ if skin == 1 => (1100.0, 440.0),
        _ => (1100.0, 252.0),
    }
}

pub const FACE_W: f32 = 1100.0;
pub const FACE_H: f32 = 500.0;

pub fn paint(meter: &Meter, _reference_norm: f32) -> Scene {
    let (face_w, face_h) = view_size_for(meter.face, meter.skin);
    THEME.with(|t| t.set(meter.theme));
    let colours = palette();
    let mut scene = Scene::new();
    scene.fill_rect(Rect::new(0.0, 0.0, face_w, face_h), colours.page.0, colours.page.1, true);
    scene.fill_rect(Rect::new(8.0, 8.0, 10.0, 10.0), Color(0xff2a3034), Color(0xff2a3034), true);
    scene.fill_rect(Rect::new(face_w - 18.0, 8.0, 10.0, 10.0), Color(0xff2a3034), Color(0xff2a3034), true);
    scene.fill_rect(Rect::new(8.0, face_h - 18.0, 10.0, 10.0), Color(0xff2a3034), Color(0xff2a3034), true);
    scene.fill_rect(Rect::new(face_w - 18.0, face_h - 18.0, 10.0, 10.0), Color(0xff2a3034), Color(0xff2a3034), true);
    for y in (38..face_h as i32 - 70).step_by(3) {
        scene.fill_rect(Rect::new(1.0, y as f32, face_w - 2.0, 0.4), Color(0x0cffffff), Color(0x0cffffff), true);
    }
    scene.fill_rect(Rect::new(0.0, 0.0, face_w, if meter.face == 2 { 72.0 } else { 36.0 }), Color(0xff2c3338), Color(0xff242a2e), true);
    scene.fill_rect(Rect::new(0.0, 35.0, face_w, 1.0), Color(0xff2a3034), Color(0xff2a3034), true);
    scene.text(Point::new(16.0, 25.0), "ArcVu", Color(0xfff2f6f3), 17.0, false);

    let kinds = ["VU", "RMS", "K-12", "K-14", "K-20", "DIN", "Nordic", "BBC", "EBU"];
    let faces = ["Arc", "Horizontal", "Vertical"];
    let reference = if (meter.reference_db + 20.0).abs() < 0.2 { "−20" } else if (meter.reference_db + 14.0).abs() < 0.2 { "−14" } else { "−18" };
    let bars = if meter.face == 2 {
        vec![(Rect::new(88.0, 5.0, 68.0, 26.0), kinds[meter.kind as usize], 1),
             (Rect::new(162.0, 5.0, 92.0, 26.0), faces[meter.face as usize], 2),
             (Rect::new(260.0, 5.0, 48.0, 26.0), reference, 3),
             (Rect::new(314.0, 5.0, 60.0, 26.0), "More", 7),
             (Rect::new(12.0, 41.0, 90.0, 26.0), "Preset", 8),
             (Rect::new(114.0, 41.0, 80.0, 26.0), "Meter", 4),
             (Rect::new(206.0, 41.0, 60.0, 26.0), "I/O", 5),
             (Rect::new(278.0, 41.0, 80.0, 26.0), "Tools", 6)]
    } else {
        vec![(Rect::new(90.0, 5.0, 80.0, 26.0), kinds[meter.kind as usize], 1),
             (Rect::new(178.0, 5.0, 100.0, 26.0), faces[meter.face as usize], 2),
             (Rect::new(286.0, 5.0, 56.0, 26.0), reference, 3),
             (Rect::new(350.0, 5.0, 78.0, 26.0), "Preset", 8),
             (Rect::new(436.0, 5.0, 70.0, 26.0), "Meter", 4),
             (Rect::new(514.0, 5.0, 52.0, 26.0), "I/O", 5),
             (Rect::new(574.0, 5.0, 70.0, 26.0), "Tools", 6),
             (Rect::new(652.0, 5.0, 68.0, 26.0), "More", 7)]
    };
    for (rect, caption, panel) in bars { menu_bar(&mut scene, rect, caption, meter.panel == panel); }
    let names = if meter.mid_side { ["M", "S"] } else { ["L", "R"] };
    if meter.face != 2 { digital_panel(&mut scene, Rect::new(736.0, 4.0, face_w - 736.0 - 12.0, 28.0), meter, names); }

    let row_y = face_h - if meter.face == 2 { 100.0 } else { 64.0 };
    let body_bottom = if meter.meters_only { face_h - 12.0 } else { row_y - 8.0 };
    // Single (sum) mode keeps the stereo interface: the first meter shows the sum and the second is switched off.
    let names = if meter.single { ["SUM", ""] } else { names };
    let off = |scene: &mut Scene, r: Rect| { if meter.single { inactive(scene, r); } };
    match meter.face {
        _ if meter.skin == 1 => {
            let (first, second) = if meter.face == 2 {
                let h = body_bottom - 84.0;
                (Rect::new(12.0, 84.0, face_w - 24.0, h * 0.5 - 4.0), Rect::new(12.0, 84.0 + h * 0.5 + 4.0, face_w - 24.0, h * 0.5 - 4.0))
            } else {
                let h = body_bottom - 44.0;
                (Rect::new(12.0, 44.0, face_w * 0.5 - 18.0, h), Rect::new(face_w * 0.5 + 6.0, 44.0, face_w * 0.5 - 18.0, h))
            };
            needle_meter(&mut scene, first, names[0], meter, 0);
            needle_meter(&mut scene, second, names[1], meter, 1);
            off(&mut scene, second);
        }
        1 => {
            let top = 44.0;
            let h = (body_bottom - top - 8.0).max(80.0);
            let first = Rect::new(12.0, top, face_w - 24.0, h * 0.5 - 4.0);
            let second = Rect::new(12.0, top + h * 0.5 + 4.0, face_w - 24.0, h * 0.5 - 4.0);
            horizontal_meter(&mut scene, first, names[0], meter, 0);
            horizontal_meter(&mut scene, second, names[1], meter, 1);
            off(&mut scene, second);
        }
        2 => {
            let h = body_bottom - 84.0;
            let width = 150.0 * 2.0 + 16.0;
            stereo_vertical_meter(&mut scene, Rect::new((face_w - width) * 0.5, 84.0, width, h), names, meter);
        }
        _ => {
            let h = body_bottom - 44.0;
            let first = Rect::new(12.0, 44.0, face_w * 0.5 - 18.0, h);
            let second = Rect::new(face_w * 0.5 + 6.0, 44.0, face_w * 0.5 - 18.0, h);
            well(&mut scene, first, names[0], meter, 0);
            well(&mut scene, second, names[1], meter, 1);
            off(&mut scene, second);
        }
    }
    if !meter.meters_only {
        hardware_row(&mut scene, meter, row_y, face_w);
        scene.text(Point::new(face_w - 74.0, face_h - 9.0), "MADE IN GERMANY", Color(0xff8b9498), 10.5, true);
    }
    if meter.loud_on {
        let extra = loudness_panel::extra_height(face_w);
        scene.fill_rect(Rect::new(0.0, face_h, face_w, extra), colours.page.0, colours.page.1, true);
        loudness_panel::paint_panel(&mut scene, meter, face_w, face_h);
    }
    match meter.panel {
        1 => type_menu(&mut scene, meter),
        2 => face_menu(&mut scene, meter),
        3 => ref_menu(&mut scene, meter),
        4 => meter_menu(&mut scene, meter),
        5 => io_menu(&mut scene, meter),
        6 => tools_menu(&mut scene, meter),
        7 => more_menu(&mut scene, meter),
        8 => preset_menu(&mut scene, meter),
        _ => {}
    }
    scene
}

fn menu_bar(scene: &mut Scene, r: Rect, label: &str, open: bool) {
    // A bordered field, so the header reads as controls and not as plain text.
    let field = Rect::new(r.x - 4.0, r.y, r.w + 2.0, r.h);
    let (top, bottom, edge) = if open { (Color(0xff46535a), Color(0xff36424a), Color(0xffb7c4ca)) } else { (Color(0xff1f262a), Color(0xff161b1e), Color(0xff515d64)) };
    scene.fill_round(field, 5.0, top, bottom, true);
    scene.stroke_round(field, 5.0, edge, 1.0);
    scene.text(Point::new(r.x + 6.0, r.center().y + 5.5), label, if open { Color(0xffffffff) } else { Color(0xffe6edf0) }, 14.0, false);
    let tip = Point::new(field.right() - 12.0, r.center().y + 1.0);
    scene.fill_poly(
        vec![tip.add(Point::new(-4.0, -2.0)), tip.add(Point::new(4.0, -2.0)), tip.add(Point::new(0.0, 3.0))],
        Color(0xffb4c2c9),
        Color(0xffb4c2c9),
    );
}

fn card(scene: &mut Scene, r: Rect) {
    scene.fill_round(r.translated(2.0, 4.0), 8.0, Color(0x99000000), Color(0x99000000), true);
    scene.fill_round(r, 8.0, Color(0xff1a1e22), Color(0xff101416), true);
    scene.stroke_round(r, 8.0, Color(0xff8a949c), 1.0);
}

fn menu_row(scene: &mut Scene, r: Rect, label: &str, on: bool) {
    if on {
        scene.fill_round(r, 4.0, Color(0xff2a3338), Color(0xff2a3338), true);
    }
    scene.text(Point::new(r.x + 14.0, r.center().y + 5.0), label, if on { Color(0xfff7f1e6) } else { Color(0xffb7b0a4) }, 13.0, false);
}

fn value_chip(scene: &mut Scene, r: Rect, label: &str, on: bool) {
    scene.fill_rect(r, Color(0xff0b1013), Color(0xff0b1013), true);
    let cap = r.reduced(2.0, 2.0);
    scene.fill_rect(cap, if on { Color(0xff53656c) } else { Color(0xff3e494f) }, Color(0xff20292e), true);
    scene.fill_rect(Rect::new(cap.x, cap.y, cap.w, 1.0), Color(0xff819198), Color(0xff819198), true);
    scene.fill_rect(Rect::new(cap.x, cap.bottom() - 1.0, cap.w, 1.0), Color(0xff101619), Color(0xff101619), true);
    if on { scene.fill_rect(Rect::new(cap.x + 5.0, cap.y + 2.0, cap.w - 10.0, 3.0), Color(0xff70ef87), Color(0xff70ef87), true); }
    scene.text(Point::new(r.center().x, r.center().y + 1.0), label, if on { Color(0xfff4fff7) } else { Color(0xffe3eaed) }, 13.0, true);
}

fn type_menu(scene: &mut Scene, meter: &Meter) {
    card(scene, Rect::new(90.0, 36.0, 180.0, 286.0));
    for (i, name) in ["VU", "RMS", "K-12", "K-14", "K-20", "DIN", "Nordic", "BBC", "EBU"].iter().enumerate() {
        menu_row(scene, Rect::new(98.0, 44.0 + i as f32 * 30.0, 164.0, 28.0), name, meter.kind == i as i32);
    }
}

fn face_menu(scene: &mut Scene, meter: &Meter) {
    card(scene, Rect::new(178.0, 36.0, 168.0, 112.0));
    for (i, name) in ["Arc", "Horizontal", "Vertical"].iter().enumerate() {
        menu_row(scene, Rect::new(186.0, 44.0 + i as f32 * 32.0, 152.0, 28.0), name, meter.face == i as i32);
    }
}

fn ref_menu(scene: &mut Scene, meter: &Meter) {
    card(scene, Rect::new(286.0, 36.0, 156.0, 112.0));
    for (i, (db, name)) in [(-20.0, "−20   AES"), (-18.0, "−18   EBU"), (-14.0, "−14")].iter().enumerate() {
        let on = (meter.reference_db - db).abs() < 0.2;
        menu_row(scene, Rect::new(294.0, 44.0 + i as f32 * 32.0, 140.0, 28.0), name, on);
    }
}

fn meter_menu(scene: &mut Scene, meter: &Meter) {
    card(scene, Rect::new(430.0, 36.0, 246.0, 248.0));
    scene.text(Point::new(446.0, 58.0), "Peak hold", Color(0xff8b9498), 11.0, false);
    for (i, name) in ["Follow", "Auto", "Hold"].iter().enumerate() {
        value_chip(scene, Rect::new(446.0 + i as f32 * 72.0, 70.0, 68.0, 26.0), name, meter.peak_mode as i32 == i as i32);
    }
    menu_row(scene, Rect::new(438.0, 108.0, 230.0, 28.0), "Reset peak", false);
    menu_row(scene, Rect::new(438.0, 138.0, 230.0, 28.0), &format!("Reset overs     {}", meter.overs), meter.overs > 0);
    menu_row(scene, Rect::new(438.0, 176.0, 230.0, 28.0), if meter.aes17 { "AES-17          on" } else { "AES-17          off" }, meter.aes17);
    menu_row(scene, Rect::new(438.0, 214.0, 230.0, 28.0), if meter.single { "Display        single" } else { "Display        dual" }, meter.single);
}

fn io_menu(scene: &mut Scene, meter: &Meter) {
    card(scene, Rect::new(400.0, 36.0, 420.0, 292.0));
    scene.text(Point::new(416.0, 56.0), "Meter", Color(0xff8b9498), 11.0, false);
    value_chip(scene, Rect::new(416.0, 68.0, 80.0, 26.0), "L / R", !meter.mid_side);
    value_chip(scene, Rect::new(502.0, 68.0, 80.0, 26.0), "M / S", meter.mid_side);
    let hidden = if meter.mid_side { meter.trim[0].abs().max(meter.trim[1].abs()) } else { meter.trim[2].abs().max(meter.trim[3].abs()) };
    if hidden > 0.05 {
        scene.fill_ellipse(Rect::new(590.0, 74.0, 10.0, 10.0), Color(0xff3dcc4a), Color(0xff3dcc4a));
    }
    scene.text(Point::new(620.0, 56.0), "Listen", Color(0xff8b9498), 11.0, false);
    for (i, name) in ["Stereo", "Swap", "Left", "Right", "Mid", "Side"].iter().enumerate() {
        menu_row(scene, Rect::new(612.0, 68.0 + i as f32 * 28.0, 190.0, 26.0), name, meter.monitor == i as i32);
    }
    let trims = [
        ("Trim L", meter.trim[0]),
        ("Trim R", meter.trim[1]),
        ("Trim M", meter.trim[2]),
        ("Trim S", meter.trim[3]),
    ];
    for (i, (name, db)) in trims.iter().enumerate() {
        menu_row(scene, Rect::new(412.0, 112.0 + i as f32 * 30.0, 180.0, 28.0), &format!("{name}   {db:+.1}"), false);
    }
    value_chip(scene, Rect::new(416.0, 246.0, 72.0, 26.0), "Polar L", meter.polarity[0]);
    value_chip(scene, Rect::new(494.0, 246.0, 72.0, 26.0), "Polar R", meter.polarity[1]);
    value_chip(scene, Rect::new(416.0, 280.0, 72.0, 26.0), "Mute L", meter.mute[0]);
    value_chip(scene, Rect::new(494.0, 280.0, 72.0, 26.0), "Mute R", meter.mute[1]);
}

fn tools_menu(scene: &mut Scene, meter: &Meter) {
    card(scene, Rect::new(420.0, 36.0, 400.0, 188.0));
    let slope = |poles: i32| if poles == 0 { "Off".to_string() } else { format!("{} dB", poles * 6) };
    let rows = [
        ("High-pass", slope(meter.hpf_poles), format!("{:.0} Hz", meter.hpf_hz), meter.hpf_poles > 0),
        ("Low-pass", slope(meter.lpf_poles), format!("{:.0} Hz", meter.lpf_hz), meter.lpf_poles > 0),
        ("Mono maker", format!("{:.0} Hz", meter.mono_hz), format!("{:.0}%", meter.mono_amt * 100.0), meter.mono_amt > 0.0),
        ("Dynamic", if meter.dyn_on { format!("{:.0} Hz", meter.dyn_hz) } else { "Off".to_string() }, format!("{:.0}%", meter.dyn_depth * 100.0), meter.dyn_on),
    ];
    for (i, (name, a, b, on)) in rows.iter().enumerate() {
        let y = 52.0 + i as f32 * 40.0;
        scene.text(Point::new(436.0, y + 14.0), *name, Color(0xffe4ebe6), 13.0, false);
        value_chip(scene, Rect::new(600.0, y, 96.0, 28.0), a, *on);
        value_chip(scene, Rect::new(704.0, y, 96.0, 28.0), b, false);
    }
}

impl Meter {
    fn label_bytes(&self) -> [u8; 32] { self.label }
}

fn more_menu(scene: &mut Scene, meter: &Meter) {
    card(scene, Rect::new(400.0, 80.0, 420.0, 400.0));
    let rows = [
        format!("VU model {}", meter.vu_model),
        format!("Rise {:.0}  Fall {:.0}  Over {:.1}", meter.rise_ms, meter.fall_ms, meter.overshoot),
        format!("Weight {}", ["Off", "A", "B", "C", "D", "K"][meter.weight as usize]),
        format!("Force mono {}", if meter.force_mono { "on" } else { "off" }),
        format!("Link trims {}", if meter.trim_link { "on" } else { "off" }),
        format!("In place {}", if meter.in_place { "on" } else { "off" }),
        format!("Values {}  Hold {}  Tools {}  Label {}", on(meter.show_values), on(meter.show_hold), on(meter.show_tools), on(meter.show_label)),
        format!("Meters only {}", if meter.meters_only { "on" } else { "off" }),
        format!("Dyn {} {}", ["off", "HPF", "LPF", "BPF n", "BPF w"][meter.dyn_type as usize], if meter.dyn_listen { "listen" } else { "" }),
        format!("RMS {:.0} ms  analog {}", meter.rms_ms, if meter.analog_rms { "on" } else { "off" }),
        format!("Clip {:.1}  peak lamp {:.1}", meter.clip_db, meter.peak_thresh_db),
        format!("Size {}", [100, 125, 150][meter.gui_scale as usize]),
        format!("Bypass hidden {}", if meter.bypass_hidden { "on" } else { "off" }),
        format!("Tooltips {}", on(meter.tooltips)),
        format!("Above windows {}", on(meter.always_on_top)),
        format!("Drag {:.1}", meter.drag_sens),
        format!("Refresh {} Hz", meter.refresh_hz),
    ];
    for (i, line) in rows.iter().enumerate() {
        menu_row(scene, Rect::new(408.0, 92.0 + i as f32 * 30.0, 404.0, 28.0), line, false);
    }
}

fn on(v: bool) -> &'static str { if v { "on" } else { "off" } }

/// Preset menu rows in order. The editor's `runPresetRow` maps each index to an action.
pub const PRESET_ROWS: [&str; 18] = [
    "◀ Previous preset", "Next preset ▶", "Category",
    "Broadcast VU", "K-14", "DIN peak", "Podcast −14",
    "Reset", "Save default", "Load default", "Copy", "Paste",
    "Save preset file...", "Load preset file...", "User presets...",
    "Drag mode", "Save global settings", "Slow mode (EBU)",
];
/// Rows per wide-face column; the editor's hit test and the value overlay use the same split.
pub const PRESET_COLUMN_ROWS: usize = 6;

fn preset_menu(scene: &mut Scene, meter: &Meter) {
    if meter.face == 2 {
        card(scene, Rect::new(12.0, 72.0, 280.0, PRESET_ROWS.len() as f32 * 21.0 + 13.0));
        for (i, name) in PRESET_ROWS.iter().enumerate() {
            menu_row(scene, Rect::new(20.0, 78.0 + i as f32 * 21.0, 264.0, 20.0), name, false);
        }
        return;
    }
    // Wide faces are short, so the list runs in three columns of six rows.
    card(scene, Rect::new(350.0, 36.0, 728.0, PRESET_COLUMN_ROWS as f32 * 21.0 + 13.0));
    for (i, name) in PRESET_ROWS.iter().enumerate() {
        let (column, row) = (i / PRESET_COLUMN_ROWS, i % PRESET_COLUMN_ROWS);
        menu_row(scene, Rect::new(358.0 + column as f32 * 240.0, 42.0 + row as f32 * 21.0, 226.0, 20.0), name, false);
    }
}

fn activity_status(scene: &mut Scene, meter: &Meter, x: f32, y: f32, gap: f32) {
    let processing = meter.hpf_poles > 0 || meter.lpf_poles > 0 || meter.mono_amt > 0.0 || meter.dyn_on;
    let trims = meter.trim.iter().any(|db| db.abs() > 0.05);
    let channel = meter.polarity.iter().any(|on| *on) || meter.mute.iter().any(|on| *on);
    for (i, (label, active)) in [("FX", processing), ("TR", trims), ("CH", channel)].iter().enumerate() {
        let left = x + gap * i as f32;
        let color = if *active { Color(0xff3dcc4a) } else { Color(0xff33413b) };
        scene.fill_ellipse(Rect::new(left, y, 8.0, 8.0), color, color);
        scene.text(Point::new(left + 11.0, y + 8.5), *label, Color(0xffe6edf0), 10.5, false);
    }
}

fn hardware_row(scene: &mut Scene, meter: &Meter, y: f32, face_w: f32) {
    if meter.face == 2 {
        scene.fill_rect(Rect::new(0.0, y - 6.0, face_w, 106.0), palette().deck.0, palette().deck.1, true);
        for (i, (name, on)) in [("Auto", meter.peak_mode == PeakMode::Auto), ("Hold", meter.peak_mode == PeakMode::Hold), ("Reset", false)].iter().enumerate() {
            value_chip(scene, Rect::new(8.0 + i as f32 * 60.0, y, 56.0, 28.0), name, *on);
        }
        scene.fill_rect(Rect::new(192.0, y, 52.0, 28.0), Color(0xff090d0e), Color(0xff090d0e), true);
        scene.text(Point::new(218.0, y + 14.0), &format!("{:03}", meter.overs.min(999)), if meter.overs > 0 { Color(0xffff4a38) } else { Color(0xff496057) }, 16.0, true);
        value_chip(scene, Rect::new(248.0, y, 44.0, 28.0), "Clr", false);
        if meter.show_tools {
            value_chip(scene, Rect::new(296.0, y, 36.0, 28.0), "φL", meter.polarity[0]);
            value_chip(scene, Rect::new(336.0, y, 36.0, 28.0), "φR", meter.polarity[1]);
            value_chip(scene, Rect::new(8.0, y + 32.0, 48.0, 28.0), "L/R", !meter.mid_side);
            value_chip(scene, Rect::new(60.0, y + 32.0, 48.0, 28.0), "M/S", meter.mid_side);
        }
        for (i, db) in [-20.0f32, -18.0, -14.0].iter().enumerate() {
            value_chip(scene, Rect::new(112.0 + i as f32 * 38.0, y + 32.0, 36.0, 28.0), &format!("{:.0}", -db), (meter.reference_db - db).abs() < 0.2);
        }
        let hidden_trim = if meter.mid_side { meter.trim[0].abs().max(meter.trim[1].abs()) } else { meter.trim[2].abs().max(meter.trim[3].abs()) };
        if hidden_trim > 0.05 {
            scene.fill_ellipse(Rect::new(221.0, y + 40.0, 7.0, 7.0), Color(0xff3dcc4a), Color(0xff3dcc4a));
        }
        phase_gauge(scene, Rect::new(230.0, y + 35.0, 112.0, 24.0), meter.correlation);
        scene.text(Point::new(230.0, y + 79.0), "PHASE −39 / −20 / 0", Color(0xffe6edf0), 10.5, false);
        if meter.show_label {
            let label = std::str::from_utf8(&meter.label).unwrap_or("").trim_end_matches('\0');
            scene.text(Point::new(16.0, y + 82.0), label, Color(0xffeef3f5), 13.0, false);
        }
        activity_status(scene, meter, 112.0, y + 67.0, 38.0);
        return;
    }
    scene.fill_rect(Rect::new(0.0, y - 6.0, face_w, 70.0), palette().deck.0, palette().deck.1, true);
    let keys = [("Auto", meter.peak_mode as i32 == 1), ("Hold", meter.peak_mode as i32 == 2), ("Reset", false)];
    for (i, (name, on)) in keys.iter().enumerate() {
        value_chip(scene, Rect::new(8.0 + i as f32 * 60.0, y, 56.0, 28.0), name, *on);
    }
    let overs = format!("{:03}", meter.overs.min(999));
    scene.fill_rect(Rect::new(192.0, y, 52.0, 28.0), Color(0xff090d0e), Color(0xff090d0e), true);
    scene.text(Point::new(218.0, y + 14.0), &overs, if meter.overs > 0 { Color(0xffff4a38) } else { Color(0xff496057) }, 16.0, true);
    value_chip(scene, Rect::new(248.0, y, 44.0, 28.0), "Clr", false);
    if meter.show_tools {
        value_chip(scene, Rect::new(296.0, y, 36.0, 28.0), "φL", meter.polarity[0]);
        value_chip(scene, Rect::new(336.0, y, 36.0, 28.0), "φR", meter.polarity[1]);
        value_chip(scene, Rect::new(376.0, y, 48.0, 28.0), "L/R", !meter.mid_side);
        value_chip(scene, Rect::new(428.0, y, 48.0, 28.0), "M/S", meter.mid_side);
    }
    for (i, (db, name)) in [(-20.0, "20"), (-18.0, "18"), (-14.0, "14")].iter().enumerate() {
        let on = (meter.reference_db - db).abs() < 0.2;
        value_chip(scene, Rect::new(480.0 + i as f32 * 38.0, y, 36.0, 28.0), name, on);
    }
    let phase_x = 598.0;
    let phase_w = (face_w - 78.0 - phase_x).max(40.0);
    phase_gauge(scene, Rect::new(phase_x, y + 3.0, phase_w, 24.0), meter.correlation);
    value_chip(scene, Rect::new(face_w - 70.0, y, 60.0, 28.0), "More", meter.panel == 7);
    let hidden = if meter.mid_side { meter.trim[0].abs().max(meter.trim[1].abs()) } else { meter.trim[2].abs().max(meter.trim[3].abs()) };
    if hidden > 0.05 {
        scene.fill_ellipse(Rect::new(452.0, y + 35.0, 7.0, 7.0), Color(0xff3dcc4a), Color(0xff3dcc4a));
    }
    scene.text(Point::new(192.0, y + 43.0), "OVERS", Color(0xffe6edf0), 10.5, false);
    scene.text(Point::new(phase_x, y + 43.0), "PHASE   −39 / −20 / 0", Color(0xffe6edf0), 10.5, false);
    scene.text(Point::new(376.0, y + 43.0), "CHANNEL / REF", Color(0xffe6edf0), 10.5, false);
    if meter.show_label {
        let bytes = meter.label_bytes();
        let label = std::str::from_utf8(&bytes).unwrap_or("").trim_end_matches('\0');
        if !label.is_empty() { scene.text(Point::new(16.0, y + 49.0), label, Color(0xffeef3f5), 13.0, false); }
    }
    activity_status(scene, meter, 760.0, y + 55.0, 72.0);
}

/// Colours of the Classic skin's printed face: (top, bottom, ink) for each theme.
fn classic_face() -> (Color, Color, Color, Color, Color, Color) {
    // (top, bottom, ink, caution band, needle, danger text)
    match THEME.with(|t| t.get()) {
        // Dark dial with orange markings, like a back-lit panel.
        1 => (Color(0xff2d2927), Color(0xff131110), Color(0xffff9f1c), Color(0xffffe27a), Color(0xffffffff), Color(0xffff5a48)),
        // Ice: white dial with a cold tint; caution band is a strong orange.
        2 => (Color(0xfff2fafd), Color(0xffbad9e6), Color(0xff0c2230), Color(0xffff9a1a), Color(0xff14110c), Color(0xffb01e16)),
        // Ivory dial with a deep orange caution band.
        _ => (Color(0xfff6ecc8), Color(0xffd8c68c), Color(0xff1c1a14), Color(0xffe8741a), Color(0xff14110c), Color(0xffb01e16)),
    }
}

/// An analogue-style needle meter: printed dial, red over-zone, and a needle that follows the reading smoothly.
fn needle_meter(scene: &mut Scene, r: Rect, name: &str, meter: &Meter, channel: usize) {
    let levels = meter.levels(channel);
    let kind = meter.kind;
    let (face_top, face_bottom, ink, caution, needle_colour, danger_text) = classic_face();
    let bezel = plate(scene, r);
    let dial = bezel.reduced(4.0, 2.0);
    scene.fill_round(dial, 5.0, face_top, face_bottom, true);
    scene.stroke_round(dial, 5.0, Color(0x66000000), 1.0);
    scene.clip_round(dial, 5.0);
    // The classic VU dial runs to +4 so its printed marks fall on -20, -16 ... 0, +4.
    let (lo, hi) = if kind == 0 { (-20.0, 4.0) } else { extra::scale_ends(kind) };
    let half = 1.1f32;
    let pivot = Point::new(dial.center().x, dial.bottom() - 26.0);
    let reach = (pivot.y - dial.y - 30.0).min((dial.w * 0.5 - 34.0) / (half.sin() * 0.95)).max(40.0);
    let scale_r = reach * 0.9;
    let label_size = (reach * 0.1).clamp(13.0, 24.0);
    let angle_of = |t: f32| -half + 2.0 * half * t.clamp(0.0, 1.0);
    let point_at = |a: f32, radius: f32| pivot.add(Point::new(a.sin(), -a.cos()).mul(radius));
    // One continuous scale arc, coloured by zone, so the colours read as part of the same line as the ticks.
    let red_t = ((meter.red_db - lo) / (hi - lo)).clamp(0.0, 1.0);
    let caution_cells: Vec<i32> = (0..40).filter(|i| lamp_zone(kind, *i, meter.red_db, meter.comfort_db) == 1.0).collect();
    let caution_t = match (caution_cells.first(), caution_cells.last()) {
        (Some(first), Some(_)) => Some((((*first as f32 - 0.5) / 39.0).max(0.0), red_t)), // the caution band runs right up to the red zone
        _ => None,
    };
    let arc_w = (reach * 0.055).clamp(6.0, 14.0);
    let arc_piece = |scene: &mut Scene, from: f32, to: f32, colour: Color| {
        if to - from < 0.001 { return; }
        let pts: Vec<Point> = (0..=24).map(|i| point_at(angle_of(from + (to - from) * i as f32 / 24.0), scale_r)).collect();
        scene.stroke_poly(pts, colour, arc_w, false);
    };
    let normal_end = caution_t.map_or(red_t, |(from, _)| from);
    arc_piece(scene, 0.0, normal_end + 0.002, ink);
    if let Some((from, to)) = caution_t { arc_piece(scene, from, to + 0.002, caution); }
    arc_piece(scene, red_t, 1.0, Color(0xffd8332a));
    // Ticks sit on whole dB (or whole BBC marks) so every printed number is an integer.
    let range = hi - lo;
    let mut ticks: Vec<(f32, bool, String)> = Vec::new();
    if kind == 7 {
        for half_mark in 0..=12 {
            let mark = 1.0 + half_mark as f32 * 0.5;
            let major = half_mark % 2 == 0;
            ticks.push(((mark - 1.0) / 6.0, major, if major { format!("{:.0}", mark) } else { String::new() }));
        }
    } else {
        let raw = range / 7.5;
        let step = [1.0f32, 2.0, 3.0, 4.0, 5.0, 10.0].into_iter().find(|v| *v >= raw).unwrap_or(10.0);
        let minor = if range <= 30.0 { 1.0 } else { step / 2.0 };
        let mut db = (lo / minor).ceil() * minor;
        let mut last_major = f32::NEG_INFINITY;
        while db <= hi + 0.001 {
            let major = ((db / step).round() * step - db).abs() < 0.001;
            if major { last_major = db; }
            let text = if !major { String::new() } else if db.abs() < 0.001 { "0".to_string() } else if db > 0.0 { format!("+{:.0}", db) } else { format!("{:.0}", db) };
            ticks.push(((db - lo) / range, major, text));
            db += minor;
        }
        // Always print the end of the scale; a last labelled mark too close to it gives way to it.
        if hi - last_major >= 0.5 {
            if hi - last_major < 2.5 {
                if let Some(entry) = ticks.iter_mut().rev().find(|(_, major, _)| *major) { entry.2.clear(); }
            }
            ticks.push((1.0, true, format!("{:+.0}", hi)));
        }
    }
    for (t, major, text) in ticks {
        // End ticks sit just inside the arc's end so they join it instead of poking out as a bracket.
        let inset = 1.6 / scale_r;
        let a = angle_of(t) + if t <= 0.0 { inset } else if t >= 1.0 { -inset } else { 0.0 };
        let db = lo + t * range;
        let danger = if kind == 7 { t >= (meter.red_db - lo) / range } else { db >= meter.red_db - 0.001 };
        let in_caution = caution_t.is_some_and(|(from, to)| t >= from && t <= to) && !danger;
        let tick_colour = if danger { Color(0xffd8332a) } else if in_caution { caution } else { ink };
        let label_colour = if danger { danger_text } else { ink };
        // The arc itself ends the scale, so no tick is drawn at the two ends (they looked like loose brackets).
        if t > 0.0 && t < 1.0 {
            scene.line(point_at(a, scale_r - arc_w * 0.5 - if major { reach * 0.1 } else { reach * 0.05 }), point_at(a, scale_r), tick_colour, if major { 4.4 } else { 2.4 });
        }
        if !text.is_empty() {
            scene.text(point_at(a, scale_r + arc_w * 0.5 + label_size * 1.1 + 3.0), &text, label_colour, label_size, true);
        }
    }
    let (title, standard) = match kind {
        1 => ("RMS", ""),
        2 => ("K-12", "0 = -12 dBFS"),
        3 => ("K-14", "0 = -14 dBFS"),
        4 => ("K-20", "0 = -20 dBFS"),
        5 => ("PPM", "DIN"),
        6 => ("PPM", "NORDIC"),
        7 => ("PPM", "BBC"),
        8 => ("PPM", "EBU"),
        _ => ("VU", ""),
    };
    let title_size = (reach * 0.3).clamp(18.0, 56.0);
    let faint = Color((ink.0 & 0x00ffffff) | 0xb0000000);
    scene.text(Point::new(dial.center().x, pivot.y - reach * 0.42), title, faint, title_size, true);
    if !standard.is_empty() {
        scene.text(Point::new(dial.center().x, pivot.y - reach * 0.42 + title_size * 0.75), standard, faint, (title_size * 0.34).max(12.0), true);
    }
    // Needle: a dark blade with a soft shadow beside it.
    let t = ((levels.vu_db.max(lo - 6.0) - lo) / (hi - lo)).clamp(-0.05, 1.04);
    let a = angle_of(t);
    scene.line(pivot.add(Point::new(2.5, 2.5)), point_at(a, reach).add(Point::new(2.5, 2.5)), Color(0x38000000), 6.0);
    scene.line(point_at(a, -10.0), point_at(a, reach), needle_colour, 4.8);
    scene.pop_clip();
    // Pivot cover and the shield that hides the needle's tail.
    let shield = Rect::new(dial.x, pivot.y + 6.0, dial.w, dial.bottom() - pivot.y - 6.0);
    scene.fill_round(shield, 5.0, Color(0xff23282b), Color(0xff0e1113), true);
    scene.fill_ellipse(Rect::new(pivot.x - 10.0, pivot.y - 10.0, 20.0, 20.0), Color(0xff5b666c), Color(0xff14181a));
    scene.text(Point::new(dial.x + 14.0, shield.center().y + 5.0), name, Color(0xffeaf0f2), 16.0, false);
    if meter.face == 2 {
        digital_row(scene, dial.right() - 108.0, shield.y + 4.0, 10.0, meter, channel);
    }
    let lamp_colour = if levels.clip { (Color(0xffff4a38), Color(0xffd8332a)) } else { (Color(0xff5a2420), Color(0xff3a1612)) };
    scene.fill_ellipse(Rect::new(dial.right() - 26.0, dial.y + 10.0, 12.0, 12.0), lamp_colour.0, lamp_colour.1);
}

/// Phase correlation as a glowing line that slides along a scale: -39 (out of phase) on the left, 0 (mono) on the right.
fn phase_gauge(scene: &mut Scene, r: Rect, correlation: f32) {
    let p = palette();
    scene.fill_round(r, 4.0, Color(0xff05090b), Color(0xff0c1214), true);
    scene.stroke_round(r, 4.0, Color(0xff000000), 1.0);
    let track = r.reduced(7.0, 4.0);
    for i in 0..=12 {
        let x = track.x + track.w * i as f32 / 12.0;
        let tall = i % 6 == 0;
        scene.fill_rect(Rect::new(x - 0.5, track.bottom() - if tall { 9.0 } else { 4.0 }, 1.0, if tall { 9.0 } else { 4.0 }), Color(0xff5c6a70), Color(0xff5c6a70), true);
    }
    let t = ((correlation.clamp(-39.0, 0.0) + 39.0) / 39.0).clamp(0.0, 1.0);
    let x = track.x + track.w * t;
    let colour = if correlation < -20.0 { p.high.0 } else { p.low.0 };
    for (spread, alpha) in [(15.0f32, 0x16u32), (10.0, 0x2a), (6.0, 0x50), (3.5, 0x80)] {
        let glow = Color((colour.0 & 0x00ffffff) | (alpha << 24));
        scene.fill_round(Rect::new(x - spread, r.y + 2.0, spread * 2.0, r.h - 4.0), spread, glow, glow, true);
    }
    scene.fill_rect(Rect::new(x - 1.2, r.y + 3.0, 2.4, r.h - 6.0), colour, colour, true);
    scene.fill_rect(Rect::new(x - 0.5, r.y + 3.0, 1.0, r.h - 6.0), Color(0xffffffff), Color(0xffffffff), true);
}

fn plate(scene: &mut Scene, r: Rect) -> Rect {
    let colours = palette();
    scene.fill_round(r, 9.0, colours.bezel.0, colours.bezel.1, true);
    scene.stroke_round(r, 9.0, colours.bezel.2, 1.0);
    let face = r.reduced(4.0, 4.0);
    scene.fill_round(face, 6.0, Color(0xff0b1013), Color(0xff05080a), true);
    scene.stroke_round(face, 6.0, Color(0xff000000), 1.5);
    scene.fill_round(Rect::new(face.x + 2.0, face.y + 2.0, face.w - 4.0, face.h * 0.42), 5.0, Color(0x1cffffff), Color(0x03ffffff), true);
    r.reduced(6.0, 3.0)
}

/// Soft halo drawn behind a lit lamp so the display reads as backlit.
fn halo(color: Color) -> Color { Color((color.0 & 0x00ffffff) | 0x38000000) }

fn lamp_zone(kind: i32, index: i32, red_db: f32, comfort: f32) -> f32 {
    let (lo, hi) = extra::scale_ends(kind);
    let db = lo + (hi - lo) * (index as f32 / 39.0);
    if db >= red_db { 2.0 }
    else if (2..=4).contains(&kind) && db >= -comfort && db <= comfort { 1.0 }
    else if db >= red_db - 6.0 { 1.0 }
    else { 0.0 }
}

const THEME_COUNT: i32 = 3;

/// Colours that change with the look: lamp zones (lit, dim) and the panel tints.
struct Palette {
    low: (Color, Color),
    mid: (Color, Color),
    high: (Color, Color),
    page: (Color, Color),
    bezel: (Color, Color, Color),
    deck: (Color, Color),
}

thread_local! { static THEME: std::cell::Cell<i32> = const { std::cell::Cell::new(0) }; }

fn palette() -> Palette {
    match THEME.with(|t| t.get()) {
        // Amber: warm lamps on brown-black, like a lit valve console.
        1 => Palette {
            low: (Color(0xffffa21f), Color(0xff4a2e0c)),
            mid: (Color(0xffffe27a), Color(0xff4d4219)),
            high: (Color(0xffff3a2a), Color(0xff4f1c18)),
            page: (Color(0xff3a322b), Color(0xff191512)),
            bezel: (Color(0xff6a5c4e), Color(0xff211c17), Color(0xff9b8b79)),
            deck: (Color(0xff332b24), Color(0xff1c1713)),
        },
        // Ice: cold cyan lamps on blue steel.
        2 => Palette {
            low: (Color(0xff2ad4ff), Color(0xff0e3140)),
            mid: (Color(0xffc9f1ff), Color(0xff2c4752)),
            high: (Color(0xffff4a78), Color(0xff4f1c2c)),
            page: (Color(0xff28363f), Color(0xff11181e)),
            bezel: (Color(0xff526a7a), Color(0xff172028), Color(0xff8fb1c4)),
            deck: (Color(0xff22313a), Color(0xff131c22)),
        },
        _ => Palette {
            low: (Color(0xff3dcc4a), Color(0xff1b4422)),
            mid: (Color(0xfff0d23a), Color(0xff4d4619)),
            high: (Color(0xffff3a2a), Color(0xff4f1c18)),
            page: (Color(0xff30383d), Color(0xff161b1e)),
            bezel: (Color(0xff56626a), Color(0xff1d2327), Color(0xff8a979f)),
            deck: (Color(0xff2a3237), Color(0xff1a1f22)),
        },
    }
}

fn zone_color(zone: f32, lit: bool) -> Color {
    let p = palette();
    let pair = if zone >= 2.0 { p.high } else if zone >= 1.0 { p.mid } else { p.low };
    if lit { pair.0 } else { pair.1 }
}

/// One seven-segment cell: segment masks are a,b,c,d,e,f,g from the lowest bit.
fn seven_segment(scene: &mut Scene, x: f32, y: f32, h: f32, ch: char, lit: Color) -> f32 {
    let t = (h * 0.14).max(1.4);
    let w = h * 0.58;
    if ch == '.' {
        let dot = Rect::new(x + t * 0.4, y + h - t * 1.3, t * 1.3, t * 1.3);
        scene.fill_rect(dot, lit, lit, true);
        return t * 2.4;
    }
    let mask: u8 = match ch {
        '0' => 0b0111111, '1' => 0b0000110, '2' => 0b1011011, '3' => 0b1001111, '4' => 0b1100110,
        '5' => 0b1101101, '6' => 0b1111101, '7' => 0b0000111, '8' => 0b1111111, '9' => 0b1101111,
        '-' => 0b1000000, _ => 0,
    };
    let segments = [
        Rect::new(x + t, y, w - 2.0 * t, t),
        Rect::new(x + w - t, y + t, t, h * 0.5 - 1.5 * t),
        Rect::new(x + w - t, y + h * 0.5 + 0.5 * t, t, h * 0.5 - 1.5 * t),
        Rect::new(x + t, y + h - t, w - 2.0 * t, t),
        Rect::new(x, y + h * 0.5 + 0.5 * t, t, h * 0.5 - 1.5 * t),
        Rect::new(x, y + t, t, h * 0.5 - 1.5 * t),
        Rect::new(x + t, y + h * 0.5 - 0.5 * t, w - 2.0 * t, t),
    ];
    let ghost = Color((lit.0 & 0x00ffffff) | 0x22000000);
    let halo = Color((lit.0 & 0x00ffffff) | 0x30000000);
    for (i, seg) in segments.iter().enumerate() {
        if mask & (1 << i) != 0 {
            scene.fill_rect(seg.expanded(1.2), halo, halo, true);
            scene.fill_rect(*seg, lit, lit, true);
        } else {
            scene.fill_rect(*seg, ghost, ghost, true);
        }
    }
    w + t * 1.2
}

/// A level as fixed-width digital digits, one decimal, right-aligned in `chars` cells ("--.-" when silent).
fn digital_value(scene: &mut Scene, at: Point, h: f32, db: f32, chars: usize, lit: Color) {
    let text = if db < -99.0 { "-".repeat(chars.saturating_sub(2)) + ".-" } else { format!("{:>width$.1}", db.min(99.9), width = chars) };
    let cell = h * 0.58 + h * 0.14 * 1.2;
    let mut x = at.x;
    for ch in text.chars() {
        if ch == '.' { x += seven_segment(scene, x, at.y, h, ch, lit); } else { seven_segment(scene, x, at.y, h, ch, lit); x += cell; }
    }
}

fn digital_width(h: f32, chars: usize) -> f32 {
    (chars as f32 - 1.0) * (h * 0.58 + h * 0.14 * 1.2) + h * 0.14 * 2.4
}

/// PEAK and HOLD digits for one channel in a row; returns the width used.
fn digital_row(scene: &mut Scene, x0: f32, y: f32, digit_h: f32, meter: &Meter, channel: usize) -> f32 {
    let levels = meter.levels(channel);
    let lit = palette().low.0;
    let mut x = x0;
    let peak_label = if meter.peak_scale == 1 { "PEAK FS" } else { "PEAK" };
    for (label, show, value) in [(peak_label, meter.show_values, levels.peak_db), ("HOLD", meter.show_hold, levels.hold_db)] {
        if !show { continue; }
        scene.text(Point::new(x, y), label, Color(0xff8b9aa1), 7.5, false);
        digital_value(scene, Point::new(x, y + 2.0), digit_h, value, 5, lit);
        x += digital_width(digit_h, 5) + 12.0;
    }
    x - x0
}

/// Peak and hold numbers for the channels as a small digital panel in the header bar.
fn digital_panel(scene: &mut Scene, r: Rect, meter: &Meter, names: [&str; 2]) {
    if !meter.show_values && !meter.show_hold { return; }
    scene.fill_round(r, 4.0, Color(0xff05090b), Color(0xff0c1214), true);
    scene.stroke_round(r, 4.0, Color(0xff000000), 1.0);
    let channels = if meter.single { 1 } else { 2 };
    let per = r.w / channels as f32;
    for channel in 0..channels {
        let x = r.x + channel as f32 * per + 8.0;
        let name = if meter.single { "SUM" } else { names[channel] };
        scene.text(Point::new(x, r.center().y + 4.0), name, Color(0xffeaf0f2), 12.0, false);
        digital_row(scene, x + 20.0, r.y + 6.0, 12.0, meter, channel);
    }
}

fn well(scene: &mut Scene, r: Rect, name: &str, meter: &Meter, channel: usize) {
    let levels = meter.levels(channel);
    let kind = meter.kind;
    plate(scene, r);
    scene.text(Point::new(r.x + 12.0, r.y + 17.0), name, Color(0xffeaf0f2), 15.0, false);
    let half_w = (r.w - 28.0) * 0.5;
    let sagitta = 36.0;
    let radius = (half_w * half_w + sagitta * sagitta) / (2.0 * sagitta);
    let half = (half_w / radius).asin();
    let centre = Point::new(r.center().x, r.y + 44.0 + radius);
    let mut ribbon = Vec::new();
    for i in 0..16 {
        let a = -half - 0.01 + (2.0 * half + 0.02) * i as f32 / 15.0;
        ribbon.push(centre.add(Point::new(a.sin(), -a.cos()).mul(radius + 8.0)));
    }
    for i in (0..16).rev() {
        let a = -half - 0.01 + (2.0 * half + 0.02) * i as f32 / 15.0;
        ribbon.push(centre.add(Point::new(a.sin(), -a.cos()).mul(radius - 44.0)));
    }
    scene.fill_poly(ribbon, Color(0xff060b0d), Color(0xff060b0d));
    let step = 2.0 * half / 39.0;
    let width = step * radius * 0.75;
    let vu_led = extra::lit_lamp(kind, levels.vu_db);
    let peak_led = if levels.peak_pos < 0.0 { None } else { Some((levels.peak_pos * 39.0).round() as i32) };
    for i in 0..40 {
        let a = -half + step * i as f32;
        let dir = Point::new(a.sin(), -a.cos());
        let on = vu_led.is_some_and(|end| i <= end);
        let colour = zone_color(lamp_zone(kind, i, meter.red_db, meter.comfort_db), on);
        if on { lamp(scene, centre, dir, radius - 41.0, radius - 13.0, width * 1.5, halo(colour)); }
        lamp(scene, centre, dir, radius - 37.0, radius - 17.0, width, colour);
        if i % 3 == 0 || i == 39 {
            scene.text(centre.add(dir.mul(radius - 4.0)), &extra::scale_caption(kind, i), Color(0xffeaf0f2), 11.5, true);
        }
    }
    if let Some(peak) = peak_led {
        let a = -half + step * peak as f32;
        lamp(scene, centre, Point::new(a.sin(), -a.cos()), radius - 39.0, radius - 15.0, width * 0.3, Color(0xffffffff));
    }
    if levels.clip { scene.text(Point::new(r.right() - 44.0, r.y + 14.0), "CLIP", Color(0xffe0352b), 13.0, false); }
}

fn horizontal_meter(scene: &mut Scene, r: Rect, name: &str, meter: &Meter, channel: usize) {
    let levels = meter.levels(channel);
    let kind = meter.kind;
    let glass = plate(scene, r);
    let window = Rect::new(glass.x + 6.0, glass.y + 4.0, glass.w - 12.0, (glass.h - 8.0).max(36.0));
    scene.fill_round(window, 3.0, Color(0xff070808), Color(0xff040505), true);
    scene.text(Point::new(window.x + 10.0, window.y + 19.0), name, Color(0xffeaf0f2), 15.0, false);
    let n = 40;
    let row = Rect::new(window.x + 8.0, window.bottom() - 22.0, window.w - 16.0, 16.0);
    let step = row.w / n as f32;
    let vu_led = extra::lit_lamp(kind, levels.vu_db);
    let peak_led = if levels.peak_pos < 0.0 { None } else { Some((levels.peak_pos * 39.0).round() as i32) };
    for i in 0..n {
        let x = row.x + i as f32 * step;
        let on = vu_led.is_some_and(|end| i <= end);
        let color = zone_color(lamp_zone(kind, i, meter.red_db, meter.comfort_db), on);
        if on { scene.fill_rect(Rect::new(x - 1.0, row.y - 3.0, step + 1.0, row.h + 6.0), halo(color), halo(color), true); }
        scene.fill_rect(Rect::new(x + 0.4, row.y, (step - 1.0).max(2.0), row.h), color, color, true);
        scene.text(Point::new(x + step * 0.5, row.y - 8.0), &extra::scale_caption(kind, i), Color(0xffeaf0f2), 10.0, true);
    }
    if let Some(peak) = peak_led {
        let x = row.x + peak as f32 * step;
        scene.fill_rect(Rect::new(x + 1.0, row.y - 2.0, (step - 2.0).max(2.0), 4.0), Color(0xffffffff), Color(0xffffffff), true);
    }
    if levels.clip {
        scene.fill_ellipse(Rect::new(window.right() - 14.0, window.y + 4.0, 10.0, 10.0), Color(0xffff2a22), Color(0xffff2a22));
    }
}

/// The second meter in single mode: the same panel, dimmed, so the layout does not change.
fn inactive(scene: &mut Scene, r: Rect) {
    scene.fill_round(r, 9.0, Color(0xf0101416), Color(0xf0101416), true);
    scene.text(r.center(), "OFF", Color(0xff6f7b82), 15.0, true);
}

/// Both channels in one window, bars side by side with a shared scale between them.
fn stereo_vertical_meter(scene: &mut Scene, r: Rect, names: [&str; 2], meter: &Meter) {
    let kind = meter.kind;
    plate(scene, r);
    let slot = Rect::new(r.x + 14.0, r.y + 56.0, r.w - 28.0, r.h - 68.0);
    scene.fill_round(slot, 3.0, Color(0xff070808), Color(0xff040505), true);
    let bar_w = 100.0;
    let left_x = slot.x + 8.0;
    let right_x = slot.right() - 8.0 - bar_w;
    let top = slot.y + 6.0;
    let height = slot.h - 12.0;
    let step = height / 40.0;
    let mid = slot.center().x;
    for (channel, x) in [(0usize, left_x), (1usize, right_x)] {
        let levels = meter.levels(channel);
        let centre = x + bar_w * 0.5;
        scene.text(Point::new(centre, r.y + 20.0), names[channel], Color(0xffeaf0f2), 15.0, true);
        let off_channel = meter.single && channel == 1;
        if !off_channel { digital_row(scene, x, r.y + 33.0, 11.0, meter, channel); }
        let vu_led = if off_channel { None } else { extra::lit_lamp(kind, levels.vu_db) };
        let peak_led = if off_channel || levels.peak_pos < 0.0 { None } else { Some((levels.peak_pos * 39.0).round() as i32) };
        for i in 0..40 {
            let y = top + height - (i as f32 + 1.0) * step;
            let on = vu_led.is_some_and(|end| i <= end);
            let color = zone_color(lamp_zone(kind, i, meter.red_db, meter.comfort_db), on);
            if on { scene.fill_rect(Rect::new(x - 3.0, y - 0.6, bar_w + 6.0, step + 1.0), halo(color), halo(color), true); }
            scene.fill_rect(Rect::new(x, y + 0.4, bar_w, (step - 1.0).max(2.0)), color, color, true);
        }
        if let Some(peak) = peak_led {
            let y = top + height - (peak as f32 + 1.0) * step;
            scene.fill_rect(Rect::new(x, y, bar_w, (step - 1.0).max(2.0)), Color(0xffffffff), Color(0xffffffff), true);
        }
        if off_channel { scene.text(Point::new(centre, top + height * 0.5), "OFF", Color(0xff6f7b82), 15.0, true); }
        if levels.clip && !off_channel {
            scene.fill_ellipse(Rect::new(centre - 5.0, slot.y - 12.0, 10.0, 10.0), Color(0xffff2a22), Color(0xffff2a22));
        }
    }
    for i in 0..40 {
        if i % 2 == 0 || i == 39 {
            let y = top + height - (i as f32 + 1.0) * step;
            scene.text(Point::new(mid, y + step * 0.5 + 3.5), &extra::scale_caption(kind, i), Color(0xffeaf0f2), 11.0, true);
        }
    }
}

fn lamp(scene: &mut Scene, centre: Point, dir: Point, r0: f32, r1: f32, width: f32, color: Color) {
    let side = Point::new(-dir.y, dir.x).mul(width * 0.5);
    let a = centre.add(dir.mul(r0));
    let b = centre.add(dir.mul(r1));
    scene.fill_poly(vec![a.add(side), b.add(side.mul(0.82)), b.add(side.mul(-0.82)), a.add(side.mul(-1.0))], color, color);
}

const MAX_POINTS: usize = 32;

#[repr(C)]
pub struct ArcVuPaint {
    pub kind: c_int,
    pub x: c_float,
    pub y: c_float,
    pub w: c_float,
    pub h: c_float,
    pub x2: c_float,
    pub y2: c_float,
    pub radius: c_float,
    pub width: c_float,
    pub c0: u32,
    pub c1: u32,
    pub flags: c_int,
    pub npts: c_int,
    pub pts: [c_float; MAX_POINTS * 2],
    pub text: [u8; 48],
}

fn pack(cmd: &goodlookindri::scene::Cmd) -> ArcVuPaint {
    use goodlookindri::scene::Cmd::*;
    let mut out = ArcVuPaint {
        kind: 0,
        x: 0.0,
        y: 0.0,
        w: 0.0,
        h: 0.0,
        x2: 0.0,
        y2: 0.0,
        radius: 0.0,
        width: 0.0,
        c0: 0,
        c1: 0,
        flags: 0,
        npts: 0,
        pts: [0.0; MAX_POINTS * 2],
        text: [0; 48],
    };
    let put_pts = |out: &mut ArcVuPaint, pts: &[Point]| {
        let n = pts.len().min(MAX_POINTS);
        out.npts = n as c_int;
        for (i, p) in pts.iter().take(n).enumerate() {
            out.pts[i * 2] = p.x;
            out.pts[i * 2 + 1] = p.y;
        }
    };
    let put_text = |out: &mut ArcVuPaint, text: &str| {
        let bytes = text.as_bytes();
        let n = bytes.len().min(47);
        out.text[..n].copy_from_slice(&bytes[..n]);
    };
    match cmd {
        FillEllipse { rect, top, bottom, diagonal } => {
            out.kind = 1;
            out.x = rect.x; out.y = rect.y; out.w = rect.w; out.h = rect.h;
            out.c0 = top.0; out.c1 = bottom.0; out.flags = *diagonal as c_int;
        }
        StrokeEllipse { rect, color, width } => {
            out.kind = 2;
            out.x = rect.x; out.y = rect.y; out.w = rect.w; out.h = rect.h;
            out.c0 = color.0; out.width = *width;
        }
        FillRoundRect { rect, radius, top, bottom, vertical } => {
            out.kind = 3;
            out.x = rect.x; out.y = rect.y; out.w = rect.w; out.h = rect.h;
            out.radius = *radius; out.c0 = top.0; out.c1 = bottom.0; out.flags = *vertical as c_int;
        }
        StrokeRoundRect { rect, radius, color, width } => {
            out.kind = 4;
            out.x = rect.x; out.y = rect.y; out.w = rect.w; out.h = rect.h;
            out.radius = *radius; out.c0 = color.0; out.width = *width;
        }
        FillRect { rect, top, bottom, vertical } => {
            out.kind = 5;
            out.x = rect.x; out.y = rect.y; out.w = rect.w; out.h = rect.h;
            out.c0 = top.0; out.c1 = bottom.0; out.flags = *vertical as c_int;
        }
        Line { a, b, color, width } => {
            out.kind = 6;
            out.x = a.x; out.y = a.y; out.x2 = b.x; out.y2 = b.y;
            out.c0 = color.0; out.width = *width;
        }
        FillPoly { pts, top, bottom, diagonal } => {
            out.kind = 7;
            out.c0 = top.0; out.c1 = bottom.0; out.flags = *diagonal as c_int;
            put_pts(&mut out, pts);
        }
        StrokePoly { pts, color, width, closed } => {
            out.kind = 8;
            out.c0 = color.0; out.width = *width; out.flags = *closed as c_int;
            put_pts(&mut out, pts);
        }
        StrokeArc { center, radius, from, to, color, width } => {
            out.kind = 9;
            out.x = center.x; out.y = center.y; out.radius = *radius;
            out.x2 = *from; out.y2 = *to; out.c0 = color.0; out.width = *width;
        }
        Text { at, text, color, size, center } => {
            out.kind = 10;
            out.x = at.x; out.y = at.y; out.w = *size; out.c0 = color.0; out.flags = *center as c_int;
            put_text(&mut out, text);
        }
        ClipRound { rect, radius } => {
            out.kind = 11;
            out.x = rect.x; out.y = rect.y; out.w = rect.w; out.h = rect.h; out.radius = *radius;
        }
        PopClip => out.kind = 12,
    }
    out
}

#[no_mangle]
pub extern "C" fn arcvu_paint(meter: *const Meter, reference_norm: c_float, out: *mut ArcVuPaint, cap: c_int) -> c_int {
    if out.is_null() || cap <= 0 {
        return 0;
    }
    let Some(meter) = (unsafe { meter.as_ref() }) else { return 0 };
    let scene = paint(meter, reference_norm);
    let n = scene.cmds.len().min(cap as usize);
    let dest = unsafe { std::slice::from_raw_parts_mut(out, n) };
    for (slot, cmd) in dest.iter_mut().zip(scene.cmds.iter()) {
        *slot = pack(cmd);
    }
    n as c_int
}

#[no_mangle]
pub extern "C" fn arcvu_levels(meter: *const Meter, channel: c_int, out: *mut ArcVuLevels) {
    if out.is_null() {
        return;
    }
    if let Some(meter) = unsafe { meter.as_ref() } {
        let levels = meter.levels(channel.max(0) as usize);
        unsafe {
            *out = ArcVuLevels {
                vu_db: levels.vu_db,
                peak_db: levels.peak_db,
                vu_pos: levels.vu_pos,
                peak_pos: levels.peak_pos,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tone(meter: &mut Meter, amplitude: f32, seconds: f32) {
        let frames = (seconds * meter.sample_rate) as usize;
        let mut block = vec![0.0f32; 64];
        let mut n = 0usize;
        let mut phase = 0.0f32;
        let step = std::f32::consts::TAU * 1000.0 / meter.sample_rate;
        while n < frames {
            let count = (frames - n).min(block.len());
            for sample in &mut block[..count] {
                *sample = amplitude * phase.sin();
                phase += step;
            }
            meter.process(&block[..count], 1);
            n += count;
        }
    }

    #[test]
    fn identical_channels_read_as_correlated() {
        let mut meter = Meter::new(48_000.0);
        let mut block = vec![0.2f32, 0.2, 0.0, 0.0, -0.2, -0.2];
        for _ in 0..4000 { meter.process_io(&mut block, 2); }
        assert!(meter.correlation > -2.0, "corr {}", meter.correlation);
    }

    #[test]
    fn inverted_channels_read_as_out_of_phase() {
        let mut meter = Meter::new(48_000.0);
        let mut block = vec![0.2f32, -0.2, 0.0, 0.0, -0.2, 0.2];
        for _ in 0..8000 { meter.process_io(&mut block, 2); }
        assert!(meter.correlation < -30.0, "corr {}", meter.correlation);
    }

    #[test]
    fn din_scale_is_not_the_vu_scale() {
        assert!(extra::lamp_index(0, 0.0) != extra::lamp_index(5, 0.0));
        assert_eq!(extra::lamp_index(0, 3.0), 39);
        assert_eq!(extra::lamp_index(5, 5.0), 39);
    }

    #[test]
    fn highpass_drops_a_low_tone() {
        let mut meter = Meter::new(48_000.0);
        meter.hpf_hz = 200.0;
        meter.hpf_poles = 2;
        let mut energy = 0.0f32;
        let mut phase = 0.0f32;
        let step = std::f32::consts::TAU * 40.0 / meter.sample_rate;
        for _ in 0..2000 {
            let mut block = [phase.sin(), phase.sin()];
            phase += step;
            meter.process_io(&mut block, 2);
            energy += block[0] * block[0];
        }
        assert!(energy / 2000.0 < 0.05, "energy {}", energy / 2000.0);
    }

    #[test]
    fn k_scale_analog_and_rms_modes_share_the_same_calibration() {
        for (kind, offset) in [(2, 12.0), (3, 14.0), (4, 20.0)] {
            let mut analog = Meter::new(48_000.0);
            analog.kind = kind;
            analog.analog_rms = true;
            analog.set_reference_db(-18.0);
            tone(&mut analog, 10.0f32.powf(-18.0 / 20.0), 1.0);
            let analog_db = analog.levels(0).vu_db;

            let mut rms = Meter::new(48_000.0);
            rms.kind = kind;
            rms.analog_rms = false;
            rms.set_reference_db(-18.0);
            tone(&mut rms, 10.0f32.powf(-18.0 / 20.0), 1.0);
            let rms_db = rms.levels(0).vu_db;
            let expected = -18.0 + offset;
            assert!((analog_db - expected).abs() < 0.2, "kind {kind} analog {analog_db}, expected {expected}");
            assert!((rms_db - expected).abs() < 0.2, "kind {kind} RMS {rms_db}, expected {expected}");
            assert!((analog_db - rms_db).abs() < 0.2, "kind {kind} analog {analog_db}, RMS {rms_db}");
        }
    }

    #[test]
    fn sine_at_reference_reads_about_zero_vu() {
        let mut meter = Meter::new(48_000.0);
        meter.set_reference_db(-18.0);
        let peak = 10f32.powf(-18.0 / 20.0);
        tone(&mut meter, peak, 1.0);
        let levels = meter.levels(0);
        assert!((levels.vu_db).abs() < 0.15, "vu {}", levels.vu_db);
        assert!((levels.peak_db - 3.01).abs() < 0.2, "peak {}", levels.peak_db);
        assert!(levels.peak_pos > levels.vu_pos);
    }

    /// Rise to 99% of reference and overswing for one VU model, with the 1 kHz step used by the standard.
    fn vu_step_response(model: i32, overshoot: f32) -> (f32, f32) {
        let mut meter = Meter::new(48_000.0);
        meter.set_reference_db(0.0);
        meter.vu_model = model;
        meter.overshoot = overshoot;
        let input = 1.0 / SINE_TO_RMS;
        let (mut first_99, mut max_x, mut n) = (None, 0.0f32, 0usize);
        let block = vec![input; 48];
        while n < (0.8 * 48_000.0) as usize {
            meter.process(&block, 1);
            n += block.len();
            max_x = max_x.max(meter.channels[0].x);
            if first_99.is_none() && meter.channels[0].x >= 0.99 { first_99 = Some(n as f32 / 48_000.0); }
        }
        (first_99.expect("never reached 99 percent"), max_x - 1.0)
    }

    #[test]
    fn standard_vu_meets_response_time_and_overswing() {
        // IEC 268-17 clause 8.1 (0.3 s +/-10% to 99%) and clause 9 (overswing at least 1%, at most 1.5%).
        let (rise, overswing) = vu_step_response(0, 1.0);
        assert!((0.27..=0.33).contains(&rise), "response time {rise} s outside 0.3 s +/-10%");
        assert!((0.010..=0.015).contains(&overswing), "overswing {overswing} outside 1% to 1.5%");
    }

    /// Steady-state movement for a 0 VU sine at `hz`, averaged over the last second.
    fn vu_steady_movement(model: i32, hz: f32) -> f32 {
        // 192 kHz keeps at least twelve samples per cycle up to 16 kHz, so the sampled rectifier is not biased.
        let rate = 192_000.0f32;
        let mut meter = Meter::new(rate as f64);
        meter.set_reference_db(0.0);
        meter.vu_model = model;
        let amp = 1.0 / SINE_TO_RMS;
        let (mut total, mut n) = (0.0f32, 0usize);
        let total_frames = (2.0 * rate) as usize;
        let mut block = vec![0.0f32; 48];
        let mut frame = 0usize;
        while frame < total_frames {
            for sample in &mut block {
                let t = frame as f64 / rate as f64;
                *sample = amp * (std::f64::consts::TAU * hz as f64 * t).sin() as f32;
                frame += 1;
            }
            meter.process(&block, 1);
            if frame > rate as usize { total += meter.channels[0].x; n += 1; }
        }
        total / n as f32
    }

    #[test]
    fn standard_vu_amplitude_response_stays_within_half_a_decibel() {
        // IEC 268-17 clause 7: deviation less than 0.5 dB from 31.5 Hz to 16 kHz.
        for model in [0, 1, 2] {
            let reference = vu_steady_movement(model, 1000.0);
            for hz in [31.5f32, 100.0, 4000.0, 8000.0, 16000.0] {
                let deviation_db = 20.0 * (vu_steady_movement(model, hz) / reference).log10();
                assert!(deviation_db.abs() < 0.5, "model {model}: {hz} Hz deviates {deviation_db:.2} dB");
            }
        }
    }


    /// Steady PPM reading of an EBU (type IIb) meter for a 1 kHz tone at `target_db` relative to test.
    fn ebu_steady_reading(target_db: f32, hz: f32, rate: f32) -> f32 {
        let mut meter = Meter::new(rate as f64);
        meter.set_reference_db(0.0);
        meter.kind = 8;
        let amp = 10f32.powf(target_db / 20.0);
        let mut block = vec![0.0f32; 48];
        let mut frame = 0usize;
        while frame < rate as usize {
            for sample in &mut block {
                *sample = amp * (std::f32::consts::TAU * hz * frame as f32 / rate).sin();
                frame += 1;
            }
            meter.process(&block, 1);
        }
        meter.reading(0)
    }

    #[test]
    fn ppm_fall_times_match_itu_table_1() {
        // ITU-R BS.645-2 Annex 2 Table 1: time to fall the stated amount after a steady tone is removed.
        for (kind, fall_db, fall_s) in [(5, 20.0f32, 1.7f32), (6, 20.0, 1.7), (7, 26.0, 3.0), (8, 24.0, 2.8)] {
            let mut meter = Meter::new(48_000.0);
            meter.set_reference_db(0.0);
            meter.kind = kind;
            let amp = 10f32.powf(9.0 / 20.0);
            let mut block = vec![0.0f32; 48];
            let mut frame = 0usize;
            while frame < 48_000 {
                for sample in &mut block { *sample = amp * (std::f32::consts::TAU * 1000.0 * frame as f32 / 48_000.0).sin(); frame += 1; }
                meter.process(&block, 1);
            }
            let start_reading = meter.reading(0);
            let start = frame;
            let mut time = None;
            while frame < 10 * 48_000 {
                for sample in &mut block { *sample = 0.0; frame += 1; }
                meter.process(&block, 1);
                if time.is_none() && meter.reading(0) <= start_reading - fall_db { time = Some((frame - start) as f32 / 48_000.0); }
            }
            let time = time.expect("never fell");
            assert!((time - fall_s).abs() <= 0.1, "kind {kind}: {fall_db} dB fall took {time:.2} s, expected {fall_s}");
        }
    }

    #[test]
    fn ebu_slow_mode_meets_table_3() {
        // Tech 3205-E Table 3 (slow mode, 5 kHz bursts at a steady +8): 5000 ms +8 +/-0.5, 10 ms -3 +/-1,
        // 10 ms repeated every 100 ms +7 +/-0.5. The 1.15 s slow network is from Fig. 2.
        let rate = 48_000.0f32;
        let amp = 10f32.powf(8.0 / 20.0);
        let run = |pattern: &dyn Fn(usize) -> bool, seconds: f32| -> Vec<f32> {
            let mut meter = Meter::new(rate as f64);
            meter.set_reference_db(0.0);
            meter.kind = 8;
            meter.slow_mode = true;
            let total = (seconds * rate) as usize;
            let mut readings = Vec::with_capacity(total);
            let mut block = vec![0.0f32; 48];
            let mut frame = 0usize;
            while frame < total {
                for sample in &mut block {
                    *sample = if pattern(frame) { amp * (std::f32::consts::TAU * 5000.0 * frame as f32 / rate).sin() } else { 0.0 };
                    frame += 1;
                }
                meter.process(&block, 1);
                readings.push(meter.reading(0));
            }
            readings
        };
        let steady = *run(&|_| true, 5.0).last().unwrap();
        assert!((steady - 8.0).abs() <= 0.5, "5000 ms steady slow reading {steady:.2}");
        let burst_ms = 10usize * 48;
        let burst = run(&|f| f < burst_ms, 3.0).into_iter().fold(f32::MIN, f32::max);
        assert!((burst + 3.0).abs() <= 1.0, "10 ms slow burst {burst:.2}");
        let repeated = run(&|f| (f % 4800) < burst_ms, 5.0);
        // One reading per 48-frame block, so the last 1000 readings are the final second.
        let settled = repeated[repeated.len() - 1000..].iter().sum::<f32>() / 1000.0;
        assert!((settled - 7.0).abs() <= 0.5, "repeated 10 ms slow reading {settled:.2}");
    }

    /// EBU meter peak reading for a burst of `hz` at `level_db` (relative to test), lasting `dur_s`, then a 0.3 s tail.
    fn ebu_burst_peak(level_db: f32, dur_s: f32, hz: f32, rate: f32) -> f32 {
        let mut meter = Meter::new(rate as f64);
        meter.set_reference_db(0.0);
        meter.kind = 8;
        let amp = 10f32.powf(level_db / 20.0);
        let burst = (dur_s * rate) as usize;
        let total = burst + (0.3 * rate) as usize;
        let mut block = vec![0.0f32; 48];
        let (mut frame, mut peak) = (0usize, f32::MIN);
        while frame < total {
            for sample in &mut block {
                *sample = if frame < burst { amp * (std::f32::consts::TAU * hz * frame as f32 / rate).sin() } else { 0.0 };
                frame += 1;
            }
            meter.process(&block, 1);
            peak = peak.max(meter.reading(0));
        }
        peak
    }

    #[test]
    fn ebu_integration_time_is_10_ms() {
        // Clause 3.3: the burst that reads 2 dB below a steady +9 is 10 +/-2 ms long.
        let steady = ebu_steady_reading(9.0, 5000.0, 96_000.0);
        let mut found = None;
        for step in 1..=150 {
            let dur = step as f32 * 0.1 * 0.001;
            if ebu_burst_peak(9.0, dur, 5000.0, 96_000.0) >= steady - 2.0 {
                found = Some(dur * 1000.0);
                break;
            }
        }
        let ms = found.expect("never reached -2 dB");
        assert!((8.0..=12.0).contains(&ms), "integration time {ms:.1} ms, expected 10 +/-2");
    }

    #[test]
    fn ebu_100_microsecond_pulse_reads_at_most_minus_10() {
        // Clause 3.4: a 1 ms unidirectional pulse adjusted to read Test (0), then a 100 us pulse of the same amplitude.
        let rate = 96_000.0f32;
        let pulse = |amp: f32, dur_s: f32| -> f32 {
            let mut meter = Meter::new(rate as f64);
            meter.set_reference_db(0.0);
            meter.kind = 8;
            let n = (dur_s * rate) as usize;
            let total = n + (0.3 * rate) as usize;
            let mut block = vec![0.0f32; 48];
            let (mut frame, mut peak) = (0usize, f32::MIN);
            while frame < total {
                for sample in &mut block { *sample = if frame < n { amp } else { 0.0 }; frame += 1; }
                meter.process(&block, 1);
                peak = peak.max(meter.reading(0));
            }
            peak
        };
        let (mut lo, mut hi) = (0.0001f32, 10.0f32);
        for _ in 0..60 {
            let mid = (lo * hi).sqrt();
            if pulse(mid, 0.001) < 0.0 { lo = mid; } else { hi = mid; }
        }
        let short = pulse(lo, 0.0001);
        assert!(short <= -10.0, "100 us pulse reads {short:.2}, limit -10");
    }

    #[test]
    fn ebu_linearity_within_1_db_over_the_scale() {
        // Clause 3.5: 10 ms bursts reduced in steps; each change in indication matches the change in level within 1 dB.
        let top = ebu_burst_peak(9.0, 0.01, 5000.0, 96_000.0);
        for level in [7.0f32, 5.0, 3.0, 1.0, -1.0, -3.0, -6.0, -9.0, -12.0] {
            let reading = ebu_burst_peak(level, 0.01, 5000.0, 96_000.0);
            let error = (reading - top) - (level - 9.0);
            assert!(error.abs() <= 1.0, "level {level}: indication error {error:.2} dB");
        }
    }

    #[test]
    fn ebu_overload_rises_5_to_10_db_for_a_10_db_step() {
        // Clause 3.7: a 1.5 ms burst at +9 that is raised by 10 dB reads 5 to 10 dB higher.
        let before = ebu_burst_peak(9.0, 0.0015, 5000.0, 96_000.0);
        let after = ebu_burst_peak(19.0, 0.0015, 5000.0, 96_000.0);
        let rise = after - before;
        // The step is exactly 10 dB, the upper limit; allow 0.01 dB of float rounding on that limit.
        assert!((5.0..=10.01).contains(&rise), "overload rise {rise:.2} dB");
    }

    #[test]
    fn ebu_reversibility_error_is_at_most_half_a_decibel() {
        // Clause 2.4: a sine at +9, with positive peaks reduced to 25% and DC removed, read in both polarities.
        let rate = 96_000.0f32;
        let amp = 10f32.powf(9.0 / 20.0);
        let reading = |sign: f32| -> f32 {
            let n = (0.5 * rate) as usize;
            let mut signal: Vec<f32> = (0..n).map(|i| sign * amp * (std::f32::consts::TAU * 1000.0 * i as f32 / rate).sin()).collect();
            for v in &mut signal { if *v > 0.0 { *v *= 0.25; } }
            let mean = signal.iter().sum::<f32>() / n as f32;
            for v in &mut signal { *v -= mean; }
            let mut meter = Meter::new(rate as f64);
            meter.set_reference_db(0.0);
            meter.kind = 8;
            let mut peak = f32::MIN;
            for chunk in signal.chunks(48) { meter.process(chunk, 1); peak = peak.max(meter.reading(0)); }
            peak
        };
        let error = (reading(1.0) - reading(-1.0)).abs();
        assert!(error <= 0.5, "reversibility error {error:.2} dB");
    }

    #[test]
    fn ebu_minus_21_db_burst_moves_the_display_just_perceptibly() {
        // Clause 3.6: an isolated 10 ms 5 kHz burst at -21 dB (re Test) gives a just-perceptible movement.
        // Here: it lights the first lamp, a much quieter burst leaves the display at rest, and so does silence.
        let burst = ebu_burst_peak(-21.0, 0.010, 5000.0, 96_000.0);
        assert_eq!(extra::lit_lamp(8, burst), Some(0), "the -21 dB burst reads {burst:.1} dB and must light the first lamp");
        let quiet = ebu_burst_peak(-36.0, 0.010, 5000.0, 96_000.0);
        assert_eq!(extra::lit_lamp(8, quiet), None, "a -36 dB burst reads {quiet:.1} dB and must not move the display");
        assert_eq!(extra::lit_lamp(8, -90.0), None, "silence leaves the display at rest");
    }

    #[test]
    fn rms_meter_reads_absolute_dbfs_whatever_the_reference() {
        // A 1 kHz sine at -12 dBFS peak: with AES-17 the RMS meter reads -12, without it -15.
        for (analog, aes17, expected) in [(false, true, -12.0f32), (false, false, -15.0), (true, true, -12.0), (true, false, -15.0)] {
            for reference in [-20.0f32, -18.0, -14.0] {
                let mut meter = Meter::new(48_000.0);
                meter.set_reference_db(reference);
                meter.kind = 1;
                meter.analog_rms = analog;
                meter.aes17 = aes17;
                tone(&mut meter, 10f32.powf(-12.0 / 20.0), 3.0);
                let reading = meter.reading(0);
                assert!((reading - expected).abs() < 0.4, "analog {analog} aes17 {aes17} ref {reference}: read {reading:.2}, expected {expected}");
            }
        }
    }

    #[test]
    fn dbfs_peak_scale_puts_full_scale_at_zero_on_every_meter_type() {
        // A -10 dBFS sample peak reads -10 on the PEAK digits for every type once the peak scale is dBFS.
        for kind in 0..=8 {
            for reference in [-20.0f32, -18.0, -14.0] {
                let mut meter = Meter::new(48_000.0);
                meter.set_reference_db(reference);
                meter.kind = kind;
                tone(&mut meter, 10f32.powf(-10.0 / 20.0), 1.0);
                meter.peak_scale = 1;
                let absolute = meter.levels(0).peak_db;
                assert!((absolute + 10.0).abs() < 0.3, "kind {kind} ref {reference}: dBFS peak {absolute:.2}");
            }
        }
    }

    #[test]
    fn ebu_calibration_meets_table_1() {
        // Tech 3205-E Table 1: +/-0.3 dB at -8, -4, 0, +4, +8 and +/-0.5 dB at -12 and +12.
        for (target, tolerance) in [(-12.0f32, 0.5f32), (0.0, 0.3), (4.0, 0.3), (8.0, 0.3), (12.0, 0.5)] {
            let reading = ebu_steady_reading(target, 1000.0, 48_000.0);
            assert!((reading - target).abs() <= tolerance, "EBU reads {reading:.2} for {target} dB, limit {tolerance}");
        }
    }

    #[test]
    fn ebu_amplitude_frequency_response_meets_clause_2_3() {
        // Clause 2.3: within +/-0.3 dB from 31.5 Hz to 16 kHz. Measured at 192 kHz so the sampled rectifier is not biased.
        let reference = ebu_steady_reading(4.0, 1000.0, 192_000.0);
        for hz in [31.5f32, 100.0, 4000.0, 8000.0, 16000.0] {
            let deviation = ebu_steady_reading(4.0, hz, 192_000.0) - reference;
            assert!(deviation.abs() <= 0.3, "{hz} Hz deviates {deviation:.2} dB");
        }
    }

    #[test]
    fn ebu_delay_overswing_and_return_meet_clauses_3_8_to_3_10() {
        let mut meter = Meter::new(48_000.0);
        meter.set_reference_db(0.0);
        meter.kind = 8;
        let nine = 10f32.powf(9.0 / 20.0);
        let twelve = 10f32.powf(12.0 / 20.0);
        let tone = |amp: f32, frame: usize| amp * (std::f32::consts::TAU * 1000.0 * frame as f32 / 48_000.0).sin();

        // 3.8: passes +8 within 150 ms of a +9 onset.
        let (mut frame, mut block) = (0usize, vec![0.0f32; 48]);
        let mut delay = None;
        while frame < 24_000 {
            for sample in &mut block { *sample = tone(nine, frame); frame += 1; }
            meter.process(&block, 1);
            if delay.is_none() && meter.reading(0) >= 8.0 { delay = Some(frame as f32 / 48_000.0); }
        }
        assert!(delay.expect("never reached +8") <= 0.150, "delay {delay:?}");

        // 3.9: overswing at most 0.5 dB over the steady +9 reading.
        let mut peak = f32::MIN;
        let steady = {
            let mut m = Meter::new(48_000.0);
            m.set_reference_db(0.0);
            m.kind = 8;
            let mut f = 0usize;
            while f < 48_000 {
                for sample in &mut block { *sample = tone(nine, f); f += 1; }
                m.process(&block, 1);
            }
            m.reading(0)
        };
        for _ in 0..50 {
            for sample in &mut block { *sample = tone(nine, frame); frame += 1; }
            meter.process(&block, 1);
            peak = peak.max(meter.reading(0));
        }
        assert!(peak - steady <= 0.5, "overswing {:.2} dB", peak - steady);

        // 3.10: from +12 down to -12 in 2.8 +/-0.3 s.
        let mut frame = 0usize;
        while frame < 48_000 {
            for sample in &mut block { *sample = tone(twelve, frame); frame += 1; }
            meter.process(&block, 1);
        }
        let start = frame;
        let mut return_time = None;
        while frame < 288_000 {
            for sample in &mut block { *sample = 0.0; frame += 1; }
            meter.process(&block, 1);
            if return_time.is_none() && meter.reading(0) <= -12.0 { return_time = Some((frame - start) as f32 / 48_000.0); }
        }
        let return_time = return_time.expect("never reached -12");
        assert!((return_time - 2.8).abs() <= 0.3, "return time {return_time:.2} s");
    }

    #[test]
    fn ebu_tone_burst_response_meets_table_2() {
        // Normal-mode 5 kHz bursts relative to a steady +8 reading: (duration ms, indication, tolerance).
        let rate = 96_000.0f32;
        let amp = 10f32.powf(8.0 / 20.0);
        for (ms, indication, tolerance) in [(100.0f32, 8.0f32, 0.5f32), (10.0, 6.0, 0.5), (5.0, 4.0, 0.75), (1.5, -1.0, 1.0), (0.5, -9.0, 2.0)] {
            let mut meter = Meter::new(rate as f64);
            meter.set_reference_db(0.0);
            meter.kind = 8;
            let burst = (ms * 0.001 * rate) as usize;
            let total = burst + (0.3 * rate) as usize;
            let (mut frame, mut block, mut peak) = (0usize, vec![0.0f32; 96], f32::MIN);
            while frame < total {
                for sample in &mut block {
                    *sample = if frame < burst { amp * (std::f32::consts::TAU * 5000.0 * frame as f32 / rate).sin() } else { 0.0 };
                    frame += 1;
                }
                meter.process(&block, 1);
                peak = peak.max(meter.reading(0));
            }
            assert!((peak - indication).abs() <= tolerance, "{ms} ms burst reads {peak:.2}, expected {indication} +/-{tolerance}");
        }
    }

    #[test]
    fn vu_models_one_and_two_meet_the_standard_at_their_defaults() {
        // Both models must comply with IEC 268-17 clauses 8.1 and 9 with the default settings (overshoot 1.0, rise and fall 300 ms).
        for model in [1, 2] {
            let (rise, overswing) = vu_step_response(model, 1.0);
            assert!((0.27..=0.33).contains(&rise), "model {model} response time {rise} s outside 0.3 s +/-10%");
            assert!((0.010..=0.015).contains(&overswing), "model {model} overswing {overswing} outside 1% to 1.5%");
        }
    }

    #[test]
    fn step_reaches_99_percent_near_300_ms_with_small_overshoot() {
        let mut meter = Meter::new(48_000.0);
        meter.set_reference_db(0.0);
        let target = 1.0f32;
        let rectified = target;
        let input = rectified / SINE_TO_RMS;
        let mut first_99 = None;
        let mut max_x = 0.0f32;
        let frames = (0.8 * meter.sample_rate) as usize;
        let mut block = vec![input; 48];
        let mut n = 0;
        while n < frames {
            meter.process(&block, 1);
            n += block.len();
            max_x = max_x.max(meter.channels[0].x);
            if first_99.is_none() && meter.channels[0].x >= 0.99 * target {
                first_99 = Some(n as f32 / meter.sample_rate);
            }
        }
        let rise = first_99.expect("never reached 99 percent");
        let overshoot = max_x / target - 1.0;
        assert!((rise - 0.30).abs() < 0.04, "rise {rise}");
        assert!((overshoot - 0.012).abs() < 0.008, "overshoot {overshoot}");
    }


    #[test]
    fn rise_and_fall_share_the_movement() {
        let mut up = Meter::new(48_000.0);
        up.set_reference_db(0.0);
        let input = 1.0 / SINE_TO_RMS;
        let mut block = vec![input; 48];
        let mut n = 0;
        let mut rise = 0.0;
        while n < 48_000 {
            up.process(&block, 1);
            n += block.len();
            if up.channels[0].x >= 0.99 {
                rise = n as f32 / up.sample_rate;
                break;
            }
        }
        let mut down = Meter::new(48_000.0);
        down.channels[0].x = 1.0;
        block.fill(0.0);
        n = 0;
        let mut fall = 0.0;
        while n < 48_000 {
            down.process(&block, 1);
            n += block.len();
            if down.channels[0].x <= 0.01 {
                fall = n as f32 / down.sample_rate;
                break;
            }
        }
        assert!((rise - fall).abs() < 0.03, "rise {rise} fall {fall}");
    }

    #[test]
    fn paint_fits_the_command_buffer() {
        let meter = Meter::new(48_000.0);
        let scene = paint(&meter, 0.5);
        assert!(scene.cmds.len() < 4096, "cmds {}", scene.cmds.len());
        for cmd in &scene.cmds {
            if let goodlookindri::scene::Cmd::FillPoly { pts, .. } | goodlookindri::scene::Cmd::StrokePoly { pts, .. } = cmd {
                assert!(pts.len() <= 32, "poly {}", pts.len());
            }
        }
    }

    #[test]
    fn loudness_panel_paints_on_every_face_and_its_buttons_change_the_settings() {
        for face in 0..3 {
            for skin in 0..2 {
                let mut meter = Meter::new(48_000.0);
                meter.face = face;
                meter.skin = skin;
                let off = paint(&meter, 0.5).cmds.len();
                meter.loud_on = true;
                LOUDNESS.store(1, std::sync::atomic::Ordering::Relaxed);
                let (_, face_h) = view_size_for(face, skin);
                let scene = paint(&meter, 0.5);
                assert!(scene.cmds.len() > off && scene.cmds.len() < 4096, "face {face} skin {skin}: {} cmds", scene.cmds.len());
                let lay = loudness_panel::layout(view_size_for(face, skin).0, face_h);
                assert!(lay.buttons.iter().all(|b| b.bottom() <= face_h + loudness_panel::extra_height(view_size_for(face, skin).0)), "buttons inside the window");
                // Target button cycles through the presets and wraps.
                let c = lay.buttons[1].center();
                let first = meter.loud_target;
                assert_eq!(loudness_panel::click(&mut meter, c.x, c.y), 2);
                assert!((meter.loud_target - first).abs() > 0.5);
                for _ in 0..3 { loudness_panel::click(&mut meter, c.x, c.y); }
                assert_eq!(meter.loud_target, first);
                let c = lay.buttons[4].center();
                loudness_panel::click(&mut meter, c.x, c.y);
                assert_eq!(meter.loud_scale, 1);
                assert_eq!(loudness_panel::click(&mut meter, 5.0, 5.0), 0);
                LOUDNESS.store(0, std::sync::atomic::Ordering::Relaxed);
            }
        }
    }

    #[test]
    fn loudness_follows_the_audio_that_reaches_the_meter() {
        let mut meter = Meter::new(48_000.0);
        let mut tone: Vec<f32> = (0..48_000 * 6).flat_map(|i| { let v = 0.070794578 * (std::f32::consts::TAU * 1000.0 * i as f32 / 48_000.0).sin(); [v, v] }).collect();
        meter.process_io(&mut tone, 2);
        let r = meter.loud.readings();
        assert!((r.integrated + 23.0).abs() < 0.15, "I {}", r.integrated);
    }

    #[test]
    fn bounce_log_measures_the_output_and_writes_a_row_per_second() {
        let mut meter = Meter::new(48_000.0);
        arcvu_bounce(&mut meter, 1);
        // A -23 dBFS tone through the plug-in: the log must read the output.
        let mut s = ArcVuSetup::default();
        s.reference_db = -18.0; s.trim_l = 0.0; s.trim_r = 0.0; s.trim_m = 0.0; s.trim_s = 0.0;
        s.hpf_hz = 20.0; s.lpf_hz = 20000.0; s.mono_hz = 100.0; s.dyn_hz = 1000.0; s.rms_ms = 300.0; s.clip_db = 0.0;
        s.value_hold_s = 5.0; s.peak_hold_s = 3.0; s.rise_ms = 300.0; s.fall_ms = 300.0; s.refresh_hz = 30;
        s.trim_l = 0.0;
        arcvu_setup(&mut meter, &s);
        let mut tone: Vec<f32> = (0..48_000 * 5).flat_map(|i| { let v = 0.070794578 * (std::f32::consts::TAU * 1000.0 * i as f32 / 48_000.0).sin(); [v, v] }).collect();
        meter.process_io(&mut tone, 2);
        let mut readings = std::mem::MaybeUninit::<loudness::Readings>::uninit();
        let mut frames = 0u64;
        arcvu_bounce_readings(&meter, readings.as_mut_ptr(), &mut frames);
        let r = unsafe { readings.assume_init() };
        assert_eq!(frames, 48_000 * 5);
        assert!((r.short_term + 23.0).abs() < 0.3, "S {}", r.short_term);
        assert!(r.sample_peak > -23.1 && r.sample_peak < -22.9, "sample peak {}", r.sample_peak);
        let mut rows = [0.0f32; 5 * 10];
        let n = arcvu_bounce_rows(&meter, rows.as_mut_ptr(), 10);
        assert_eq!(n, 5);
        assert_eq!(rows[0], 1.0);
        assert_eq!(rows[5 * 4], 5.0);
        // Off stops the measurement, a new start clears it.
        arcvu_bounce(&mut meter, 0);
        let mut more = vec![0.5f32; 4800];
        meter.process_io(&mut more, 2);
        arcvu_bounce_readings(&meter, readings.as_mut_ptr(), &mut frames);
        assert_eq!(frames, 48_000 * 5);
        arcvu_bounce(&mut meter, 1);
        assert_eq!(arcvu_bounce_rows(&meter, rows.as_mut_ptr(), 10), 0);
    }

    #[test]
    fn three_full_scale_samples_count_as_one_over() {
        let mut meter = Meter::new(48_000.0);
        meter.process(&[0.5, 1.1, 1.1, 0.2, 1.1, 1.1, 1.1], 1);
        assert_eq!(meter.overs(), 1);
        meter.process(&[0.0, 1.1, 1.1, 1.1], 1);
        assert_eq!(meter.overs(), 2);
        meter.reset_overs();
        assert_eq!(meter.overs(), 0);
    }

    #[test]
    fn custom_model_fall_is_slower_than_rise() {
        let mut meter = Meter::new(48_000.0);
        meter.vu_model = 2;
        meter.rise_ms = 150.0;
        meter.fall_ms = 800.0;
        meter.set_reference_db(0.0);
        let input = 1.0 / SINE_TO_RMS;
        let mut block = vec![input; 48];
        let mut n = 0;
        let mut rise = 1.0;
        while n < 48_000 {
            meter.process(&block, 1);
            n += block.len();
            if meter.channels[0].x >= 0.9 { rise = n as f32 / meter.sample_rate; break; }
        }
        block.fill(0.0);
        n = 0;
        let mut fall = 1.0;
        while n < 96_000 {
            meter.process(&block, 1);
            n += block.len();
            if meter.channels[0].x <= 0.1 { fall = n as f32 / meter.sample_rate; break; }
        }
        assert!(fall > rise * 1.5, "rise {rise} fall {fall}");
    }

    #[test]
    fn a_weight_quiets_a_low_tone_more_than_a_high_one() {
        let low = measured_weighting_db(1, 40.0, 48_000.0);
        let high = measured_weighting_db(1, 1000.0, 48_000.0);
        assert!(low < high - 30.0, "40 Hz {low:.2} dB, 1 kHz {high:.2} dB");
    }

    #[test]
    fn single_meter_keeps_the_other_channel() {
        let mut meter = Meter::new(48_000.0);
        meter.single = true;
        let mut block = vec![0.4f32, 0.05, 0.4, 0.05];
        for _ in 0..4000 { meter.process_io(&mut block, 2); }
        assert!(meter.levels(0).vu_db > meter.levels(1).vu_db + 3.0, "sum {} other {}", meter.levels(0).vu_db, meter.levels(1).vu_db);
    }

    #[test]
    fn quiet_peak_hides_below_the_threshold() {
        let mut meter = Meter::new(48_000.0);
        meter.peak_thresh_db = -6.0;
        meter.process(&[0.001; 64], 1);
        assert!(meter.levels(0).peak_pos < 0.0);
    }

    #[test]
    fn force_mono_copies_left_onto_right() {
        let mut meter = Meter::new(48_000.0);
        meter.force_mono = true;
        let mut block = [0.4f32, 0.0, 0.4, 0.0];
        meter.process_io(&mut block, 2);
        assert!((block[1] - block[0]).abs() < 1.0e-5, "right {}", block[1]);
    }

    #[test]
    fn trim_link_copies_left_trim() {
        let mut meter = Meter::new(48_000.0);
        let setup = ArcVuSetup { trim_l: -6.0, trim_r: 3.0, trim_link: 1, ..ArcVuSetup::default() };
        meter.apply_setup(&setup);
        assert!((meter.trim[1] - meter.trim[0]).abs() < 1.0e-4);
    }

    #[test]
    fn linked_trim_follows_edits_from_either_side_in_lr_and_ms() {
        let mut meter = Meter::new(48_000.0);
        let mut setup = ArcVuSetup {
            trim_link: 1, trim_l: -6.0, trim_r: -6.0,
            trim_m: -3.0, trim_s: -3.0, mid_side: 1,
            ..ArcVuSetup::default()
        };
        meter.apply_setup(&setup);

        setup.trim_r = 2.0;
        meter.apply_setup(&setup);
        meter.apply_setup(&setup); // A steady host block must not switch the source back.
        assert_eq!(meter.trim[0], 2.0, "right edit should drive linked left trim");
        assert_eq!(meter.trim[1], 2.0);

        setup.trim_l = -1.0;
        meter.apply_setup(&setup);
        meter.apply_setup(&setup);
        assert_eq!(meter.trim[0], -1.0);
        assert_eq!(meter.trim[1], -1.0, "left edit should drive linked right trim");

        setup.trim_s = 4.0;
        meter.apply_setup(&setup);
        meter.apply_setup(&setup);
        assert_eq!(meter.trim[2], 4.0, "side edit should drive linked mid trim");
        assert_eq!(meter.trim[3], 4.0);

        setup.trim_m = 1.0;
        meter.apply_setup(&setup);
        meter.apply_setup(&setup);
        assert_eq!(meter.trim[2], 1.0);
        assert_eq!(meter.trim[3], 1.0, "mid edit should drive linked side trim");
    }

    #[test]
    fn right_and_side_link_sources_change_the_processed_signal() {
        let mut meter = Meter::new(48_000.0);
        let mut setup = ArcVuSetup {
            trim_link: 1, trim_l: 0.0, trim_r: 0.0, ..ArcVuSetup::default()
        };
        meter.apply_setup(&setup);
        setup.trim_r = 6.0;
        meter.apply_setup(&setup);
        let mut lr = [0.1f32, 0.1];
        meter.process_io(&mut lr, 2);
        assert!((lr[0] - 0.1 * 10.0f32.powf(6.0 / 20.0)).abs() < 1.0e-5);
        assert!((lr[1] - lr[0]).abs() < 1.0e-5);

        setup.mid_side = 1;
        setup.trim_s = 6.0;
        setup.trim_m = 0.0;
        meter.apply_setup(&setup);
        let mut ms = [0.1f32, 0.0];
        meter.process_io(&mut ms, 2);
        assert!((ms[0] - 0.1 * 10.0f32.powf(6.0 / 20.0)).abs() < 1.0e-5, "M/S side-source trim output {}", ms[0]);
        assert!(ms[1].abs() < 1.0e-5);
    }

    #[test]
    fn mono_maker_reduces_low_side() {
        let mut meter = Meter::new(48_000.0);
        meter.mono_amt = 1.0;
        meter.mono_hz = 200.0;
        let mut energy = 0.0;
        for n in 0..8000 {
            let t = n as f32 / 48_000.0;
            let mut block = [0.2 * (2.0 * std::f32::consts::PI * 40.0 * t).sin(), 0.0];
            meter.process_io(&mut block, 2);
            let side = 0.5 * (block[0] - block[1]);
            if n > 4000 { energy += side * side; }
        }
        assert!(energy / 4000.0 < 0.002, "side energy {energy}");
    }
    fn dynamic_listen_rms(dyn_type: i32, hz: f32) -> f32 {
        let mut meter = Meter::new(48_000.0);
        meter.dyn_on = true;
        meter.dyn_type = dyn_type;
        meter.dyn_hz = 1000.0;
        meter.dyn_depth = 0.5;
        meter.dyn_listen = true;
        let mut energy = 0.0f32;
        let mut n = 0usize;
        for block in 0..192 {
            let mut samples = vec![0.0; 1024];
            for frame in 0..512 {
                let t = (block * 512 + frame) as f32 / meter.sample_rate;
                let sample = 0.2 * (std::f32::consts::TAU * hz * t).sin();
                samples[frame * 2] = sample;
                samples[frame * 2 + 1] = sample;
            }
            meter.process_io(&mut samples, 2);
            if block >= 96 { for frame in 0..512 { energy += samples[frame*2] * samples[frame*2]; n += 1; } }
        }
        (energy / n as f32).sqrt()

    }

    /// Steady-state level change in dB for one tone through the HPF or LPF at the given slope.
    fn filter_gain_db(hpf: bool, poles: i32, cutoff: f32, tone_hz: f32) -> f32 {
        let mut meter = Meter::new(48_000.0);
        if hpf { meter.hpf_hz = cutoff; meter.hpf_poles = poles; } else { meter.lpf_hz = cutoff; meter.lpf_poles = poles; }
        let (mut energy_in, mut energy_out, mut n) = (0.0f32, 0.0f32, 0usize);
        for block in 0..192 {
            let mut samples = vec![0.0; 1024];
            for frame in 0..512 {
                // Phase is wrapped in f64 so the -80 dB measurement floor is not f32 phase error.
                let cycles = ((block * 512 + frame) as f64 * tone_hz as f64 / meter.sample_rate as f64).fract();
                let sample = (0.2 * (std::f64::consts::TAU * cycles).sin()) as f32;
                samples[frame * 2] = sample;
                samples[frame * 2 + 1] = sample;
            }
            let input = samples.clone();
            meter.process_io(&mut samples, 2);
            if block >= 96 {
                for frame in 0..512 {
                    energy_in += input[frame * 2] * input[frame * 2];
                    energy_out += samples[frame * 2] * samples[frame * 2];
                    n += 1;
                }
            }
        }
        10.0 * (energy_out / energy_in).log10()
    }

    #[test]
    fn filter_slopes_follow_six_db_per_pole_per_octave() {
        // Each stage is a one-pole section, which only reaches its 6 dB/octave asymptote well away
        // from the cutoff, so the slope is measured between one and two octaves beyond the cutoff.
        for poles in 1..=4 {
            let expected = -6.0 * poles as f32;
            let hpf = filter_gain_db(true, poles, 1000.0, 125.0) - filter_gain_db(true, poles, 1000.0, 250.0);
            // The LPF is tested lower down: near 8 kHz the discrete one-pole response flattens.
            let lpf = filter_gain_db(false, poles, 250.0, 4000.0) - filter_gain_db(false, poles, 250.0, 2000.0);
            assert!((hpf - expected).abs() < 1.0, "HPF {poles} poles: {hpf:.2} dB/oct, expected {expected}");
            assert!((lpf - expected).abs() < 1.0, "LPF {poles} poles: {lpf:.2} dB/oct, expected {expected}");
        }
    }


    #[test]
    fn zero_pole_filters_are_bypassed() {
        assert!(filter_gain_db(true, 0, 1000.0, 250.0).abs() < 0.1);
        assert!(filter_gain_db(false, 0, 1000.0, 4000.0).abs() < 0.1);
    }

    #[test]
    fn dynamic_bandpass_listen_centres_on_its_frequency() {
        // Narrow (3) passes the centre and is at least 12 dB down three octaves either side.
        let narrow = dynamic_listen_rms(3, 1000.0);
        assert!(narrow > dynamic_listen_rms(3, 50.0) * 4.0, "narrow BPF passed 50 Hz");
        assert!(narrow > dynamic_listen_rms(3, 8000.0) * 4.0, "narrow BPF passed 8 kHz");
        // Wide (4) passes the centre and rejects the low side, but is broader than narrow at the high side.
        let wide = dynamic_listen_rms(4, 1000.0);
        assert!(wide > dynamic_listen_rms(4, 50.0) * 4.0, "wide BPF passed 50 Hz");
        let narrow_ratio = narrow / dynamic_listen_rms(3, 8000.0);
        let wide_ratio = wide / dynamic_listen_rms(4, 8000.0);
        assert!(wide_ratio < narrow_ratio * 0.7, "wide BPF is not wider than narrow: wide {wide_ratio:.2}, narrow {narrow_ratio:.2}");
    }


    #[test]
    fn dynamic_zero_depth_leaves_the_signal_untouched() {
        let mut meter = Meter::new(48_000.0);
        meter.dyn_on = true;
        meter.dyn_type = 3;
        meter.dyn_hz = 1000.0;
        meter.dyn_depth = 0.0;
        let (mut energy_in, mut energy_out) = (0.0f32, 0.0f32);
        for block in 0..96 {
            let mut samples = vec![0.0; 1024];
            for frame in 0..512 {
                let t = (block * 512 + frame) as f32 / meter.sample_rate;
                let sample = 0.2 * (std::f32::consts::TAU * 300.0 * t).sin();
                samples[frame * 2] = sample;
                samples[frame * 2 + 1] = sample;
            }
            let input = samples.clone();
            meter.process_io(&mut samples, 2);
            for frame in 0..512 {
                energy_in += input[frame * 2] * input[frame * 2];
                energy_out += samples[frame * 2] * samples[frame * 2];
            }
        }
        assert!((energy_out / energy_in - 1.0).abs() < 0.01, "zero depth changed the level by {}", energy_out / energy_in);
    }

    #[test]
    fn dynamic_highpass_listen_passes_highs_and_rejects_lows() {
        let low = dynamic_listen_rms(1, 50.0);
        let high = dynamic_listen_rms(1, 8000.0);
        assert!(high > low * 10.0, "HPF listen low {low}, high {high}");
    }

    #[test]
    fn dynamic_lowpass_listen_passes_lows_and_rejects_highs() {
        let low = dynamic_listen_rms(2, 50.0);
        let high = dynamic_listen_rms(2, 8000.0);
        assert!(low > high * 10.0, "LPF listen low {low}, high {high}");
    }

    #[test]
    fn weighting_history_is_independent_per_stereo_channel() {
        let mut meter = Meter::new(48_000.0);
        meter.weight = 1;
        meter.configure_weighting();
        for block in 0..192 {
            let mut samples = vec![0.0; 1024];
            for frame in 0..512 {
                let t = (block * 512 + frame) as f32 / meter.sample_rate;
                samples[frame * 2] = 0.2 * (std::f32::consts::TAU * 1000.0 * t).sin();
            }
            meter.process_io(&mut samples, 2);
        }
        assert!(meter.levels(1).vu_db < -80.0, "silent right channel read {} dB", meter.levels(1).vu_db);
        assert!(meter.levels(0).vu_db > -40.0, "left channel unexpectedly quiet: {} dB", meter.levels(0).vu_db);
    }

    fn measured_weighting_db(kind: i32, hz: f32, sample_rate: f32) -> f32 {
        let mut meter = Meter::new(sample_rate as f64);
        meter.weight = kind;
        meter.configure_weighting();
        let frames = sample_rate as usize;
        let mut samples = vec![0.0f32; 128];
        for block in 0..frames / 128 {
            for frame in 0..128 {
                let t = (block * 128 + frame) as f32 / sample_rate;
                samples[frame] = 0.1 * (std::f32::consts::TAU * hz * t).sin();
            }
            meter.process_io(&mut samples, 1);
        }
        10.0 * (meter.channels[0].rms / (0.1 * 0.1 * 0.5)).log10()
    }

    #[test]
    fn weighting_filters_match_reference_frequency_responses() {
        // Relative dB at [20, 40, 100, 1000, 4000, 10000] Hz, normalized at
        // 1 kHz. IEC A/B/C/D curves and ITU-R BS.1770 K coefficients.
        let expected: [[f32; 6]; 5] = [
            [-50.39, -34.54, -19.15, 0.0, 0.96, -2.49],
            [-24.16, -14.10, -5.65, 0.0, -0.73, -4.30],
            [-6.22, -1.98, -0.30, 0.0, -0.83, -4.41],
            [-20.63, -14.68, -7.20, 0.0, 11.10, 3.44],
            [-13.97, -6.26, -1.83, 0.0, 3.27, 3.34],
        ];
        let frequencies = [20.0, 40.0, 100.0, 1000.0, 4000.0, 10000.0];
        for kind in 1..=5 {
            for (i, hz) in frequencies.iter().enumerate() {
                let actual = measured_weighting_db(kind, *hz, 48_000.0);
                let error = (actual - expected[(kind - 1) as usize][i]).abs();
                let tolerance_db = if kind == 4 { 1.5 } else { 0.65 };
                assert!(error < tolerance_db, "weight {kind} at {hz} Hz: got {actual:.2} dB, expected {:.2} dB (error {error:.2}, limit {tolerance_db:.2})", expected[(kind - 1) as usize][i]);
            }
        }
    }

    #[test]
    fn numeric_peak_and_peak_lamp_use_independent_hold_settings() {
        let mut meter = Meter::new(48_000.0);
        meter.set_peak_mode(PeakMode::Hold as i32);
        meter.value_hold_s = 0.02;
        meter.process(&[0.8], 1);
        let held_numeric = meter.levels(0).peak_db;
        let held_lamp = meter.levels(0).peak_pos;
        let silence = vec![0.0; 4800];
        meter.process(&silence, 1);
        assert!(meter.levels(0).peak_db < held_numeric - 3.0, "numeric peak did not release: {} -> {}", held_numeric, meter.levels(0).peak_db);
        assert!((meter.levels(0).peak_pos - held_lamp).abs() < 1.0e-4, "peak lamp hold changed independently");

        meter.reset_peaks();
        meter.set_peak_mode(PeakMode::Follow as i32);
        meter.value_hold_s = 1.0;
        meter.process(&[0.8], 1);
        let held_numeric = meter.levels(0).peak_db;
        let follow_silence = vec![0.0; 24_000];
        meter.process(&follow_silence, 1);
        assert!((meter.levels(0).peak_db - held_numeric).abs() < 1.0e-4, "numeric value hold expired early");
        assert!(meter.levels(0).peak_pos < 0.0, "follow peak lamp did not release");
    }

    /// Change in the peak lamp position after a full-scale-ish pulse and then `silence_s` of silence.
    fn peak_lamp_change(mode: PeakMode, hold_s: f32, silence_s: f32) -> f32 {
        let mut meter = Meter::new(48_000.0);
        meter.set_peak_mode(mode as i32);
        meter.peak_hold_s = hold_s;
        meter.process(&[0.8], 1);
        let start = meter.levels(0).peak_pos;
        meter.process(&vec![0.0; (silence_s * 48_000.0) as usize], 1);
        meter.levels(0).peak_pos - start
    }

    #[test]
    fn peak_hold_follows_auto_hold_and_infinity_modes() {
        // Auto holds for the configured time, then releases.
        assert!(peak_lamp_change(PeakMode::Auto, 1.0, 0.5).abs() < 1.0e-4, "auto hold released early");
        // The release is slow (about 1.6 s time constant), so the lamp turns off by 6 s.
        assert!(peak_lamp_change(PeakMode::Auto, 1.0, 6.0) < -0.05, "auto hold never released");
        // Follow never holds, so it releases as soon as the level drops.
        assert!(peak_lamp_change(PeakMode::Follow, 1.0, 0.5) < -0.05, "follow did not release");
        // Hold and a negative (infinite) Auto hold keep the peak indefinitely.
        assert!(peak_lamp_change(PeakMode::Hold, 1.0, 10.0).abs() < 1.0e-4, "hold mode released");
        assert!(peak_lamp_change(PeakMode::Auto, -1.0, 10.0).abs() < 1.0e-4, "infinite auto hold released");
    }


    #[test]
    fn clip_fires_above_threshold_and_holds_for_its_time() {
        let mut meter = Meter::new(48_000.0);
        meter.clip_db = -3.0;
        meter.clip_hold_s = 0.5;
        meter.process(&[0.6], 1);
        assert!(!meter.levels(0).clip, "clip fired below threshold");
        meter.process(&[0.8], 1);
        assert!(meter.levels(0).clip, "clip missed a sample above threshold");
        meter.process(&vec![0.0; 14_400], 1);
        assert!(meter.levels(0).clip, "clip released before its hold time");
        meter.process(&vec![0.0; 14_400], 1);
        assert!(!meter.levels(0).clip, "clip never released");
    }

    #[test]
    fn clip_infinity_and_reset_work() {
        let mut meter = Meter::new(48_000.0);
        meter.clip_db = -3.0;
        meter.clip_hold_s = -1.0;
        meter.process(&[0.8], 1);
        meter.process(&vec![0.0; 480_000], 1);
        assert!(meter.levels(0).clip, "infinite clip hold released");
        meter.reset_clip();
        assert!(!meter.levels(0).clip, "reset did not clear clip");
    }

    #[test]
    fn reset_clears_numeric_hold_as_well_as_peak() {
        let mut meter = Meter::new(48000.0);
        tone(&mut meter, 0.2, 1.0);
        assert!(meter.levels(0).hold_db > -20.0);
        meter.reset_peaks();
        assert!(meter.levels(0).hold_db <= -80.0);
        assert!(meter.levels(0).peak_db < -80.0);
    }

    #[test]
    fn every_face_shows_processing_trim_and_channel_status() {
        use goodlookindri::scene::Cmd;
        for face in 0..3 {
            let mut meter = Meter::new(48_000.0);
            meter.face = face;
            meter.hpf_poles = 1;
            meter.trim[2] = 2.0;
            meter.polarity[0] = true;
            let scene = paint(&meter, 0.5);
            let labels: Vec<_> = scene.cmds.iter().filter_map(|cmd| match cmd {
                Cmd::Text { text, .. } => Some(text.as_str()),
                _ => None,
            }).collect();
            assert!(labels.contains(&"FX"), "FX status missing on face {face}");
            assert!(labels.contains(&"TR"), "trim status missing on face {face}");
            assert!(labels.contains(&"CH"), "channel status missing on face {face}");
        }
    }

    #[test]
    fn compact_face_indicates_nonzero_hidden_ms_trim() {
        use goodlookindri::scene::Cmd;
        let mut meter = Meter::new(48_000.0);
        meter.face = 2;
        meter.trim[2] = 6.0;
        meter.show_tools = false;
        let scene = paint(&meter, 0.5);
        assert!(scene.cmds.iter().any(|cmd| matches!(cmd,
            Cmd::FillEllipse { rect, top, .. } if (rect.x - 221.0).abs() < 0.01 && top.0 == 0xff3dcc4a
        )));
    }

    fn painted_texts(meter: &Meter) -> Vec<String> {
        use goodlookindri::scene::Cmd;
        paint(meter, 0.5).cmds.iter().filter_map(|cmd| match cmd {
            Cmd::Text { text, .. } => Some(text.clone()),
            _ => None,
        }).collect()
    }

    #[test]
    fn show_hold_and_label_flags_hide_their_readouts_on_every_face() {
        for face in 0..3 {
            let mut meter = Meter::new(48_000.0);
            meter.face = face;
            meter.label[..6].copy_from_slice(b"Zq-Tst");
            tone(&mut meter, 0.2, 0.5);

            meter.show_hold = true;
            meter.show_label = true;
            let on = painted_texts(&meter);
            assert!(on.iter().any(|t| t == "HOLD"), "hold readout missing on face {face}");
            assert!(on.iter().any(|t| t.contains("Zq-Tst")), "label missing on face {face}");

            meter.show_hold = false;
            meter.show_label = false;
            let off = painted_texts(&meter);
            assert!(!off.iter().any(|t| t == "HOLD"), "hold readout still shown on face {face}");
            assert!(!off.iter().any(|t| t.contains("Zq-Tst")), "label still shown on face {face}");
        }
    }

    #[test]
    fn show_values_flag_hides_numeric_readouts_on_every_face() {
        for face in 0..3 {
            let mut meter = Meter::new(48_000.0);
            meter.face = face;
            tone(&mut meter, 0.2, 0.5);
            meter.show_values = true;
            assert!(painted_texts(&meter).iter().any(|t| t == "PEAK"), "values missing on face {face}");
            meter.show_values = false;
            assert!(!painted_texts(&meter).iter().any(|t| t == "PEAK"), "values still shown on face {face}");
        }
    }

    #[test]
    fn monitor_left_and_right_honour_in_place_and_centred_output() {
        // Monitor 2 is left, 3 is right. In place silences the other side; centred copies the monitored side.
        let frame = [0.3f32, -0.2];
        let run = |monitor: i32, in_place: bool| {
            let mut meter = Meter::new(48_000.0);
            meter.monitor = monitor;
            meter.in_place = in_place;
            let mut block = frame;
            meter.process_io(&mut block, 2);
            block
        };
        assert_eq!(run(2, true), [0.3, 0.0], "left in place");
        assert_eq!(run(2, false), [0.3, 0.3], "left centred");
        assert_eq!(run(3, true), [0.0, -0.2], "right in place");
        assert_eq!(run(3, false), [-0.2, -0.2], "right centred");
    }

    #[test]
    fn show_tools_hides_channel_hardware_keys() {
        use goodlookindri::scene::Cmd;
        let mut meter = Meter::new(48_000.0);
        meter.show_tools = false;
        let scene = paint(&meter, 0.5);
        let labels: Vec<_> = scene.cmds.iter().filter_map(|cmd| match cmd {
            Cmd::Text { text, .. } => Some(text.as_str()),
            _ => None,
        }).collect();
        assert!(!labels.contains(&"φL"));
        assert!(!labels.contains(&"φR"));
        assert!(!labels.contains(&"L/R"));
        assert!(!labels.contains(&"M/S"));
    }

    #[test]
    fn bbc_marks_sit_on_the_reference_scale() {
        let (lo, hi) = extra::scale_ends(7);
        let mark_at = |db: f32| 1.0 + (db - lo) / (hi - lo) * 6.0;
        assert!((mark_at(0.0) - 4.0).abs() < 1e-4, "mark 4 must sit at the reference");
        assert!((mark_at(-12.0) - 1.0).abs() < 1e-4, "mark 1 must sit 12 dB below");
        assert!((mark_at(12.0) - 7.0).abs() < 1e-4, "mark 7 must sit 12 dB above");
        let nearest_to_four = (0..40)
            .map(|i| extra::scale_caption(7, i).parse::<f32>().unwrap())
            .fold(f32::MAX, |best, v| if (v - 4.0).abs() < (best - 4.0).abs() { v } else { best });
        assert!((nearest_to_four - 4.0).abs() < 0.2, "a cell must read 4 near the reference");
    }

    #[test]
    fn captions_are_unique_on_all_scales() {
        for kind in 0..9 {
            let labels: std::collections::HashSet<_> = (0..40).map(|i| extra::scale_caption(kind, i)).collect();
            assert_eq!(labels.len(), 40, "repeated captions on kind {kind}");
        }
    }

    #[test]
    fn ebu_and_nordic_show_test_where_zero_sits() {
        for kind in [6, 8] {
            let marks: Vec<_> = (0..40).filter(|&i| extra::scale_caption(kind, i) == "TEST").collect();
            assert_eq!(marks.len(), 1, "one TEST cell on kind {kind}");
            let (lo, hi) = extra::scale_ends(kind);
            let db = lo + (hi - lo) * (marks[0] as f32 / 39.0);
            assert!(db.abs() <= (hi - lo) / 78.0 + 1e-3, "TEST on kind {kind} sits at {db} dB");
        }
        assert!((0..40).all(|i| extra::scale_caption(0, i) != "TEST"));
    }

}
