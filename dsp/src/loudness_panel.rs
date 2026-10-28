//! The loudness readout drawn under the face: M, S, sliding, I, LRA and true peak as numbers, a bar against the
//! target on an EBU scale, and a row of buttons. Layout and hit areas come from one function so they cannot drift.

use super::*;
use crate::loudness::FLOOR;

pub const TARGETS: [f32; 4] = [-23.0, -18.0, -16.0, -14.0];
pub const CEILINGS: [f32; 3] = [-1.0, -2.0, -3.0];
pub const WINDOWS: [f32; 5] = [5.0, 10.0, 15.0, 20.0, 30.0];

pub fn narrow(face_w: f32) -> bool { face_w < 600.0 }

/// Height added under the face while the panel is on.
pub fn extra_height(face_w: f32) -> f32 { if narrow(face_w) { 252.0 } else { 168.0 } }

pub struct Layout {
    pub panel: Rect,
    pub cells: [Rect; 6],
    pub bar: Rect,
    pub buttons: [Rect; 6],
}

pub fn layout(face_w: f32, y0: f32) -> Layout {
    let wide = !narrow(face_w);
    let panel = Rect::new(12.0, y0 + 4.0, face_w - 24.0, extra_height(face_w) - 10.0);
    let inner = panel.reduced(16.0, 0.0);
    let mut cells = [Rect::new(0.0, 0.0, 0.0, 0.0); 6];
    let mut buttons = cells;
    let bar;
    if wide {
        let cw = inner.w / 6.0;
        for (i, c) in cells.iter_mut().enumerate() { *c = Rect::new(inner.x + i as f32 * cw, y0 + 40.0, cw, 54.0); }
        bar = Rect::new(inner.x, y0 + 98.0, inner.w, 14.0);
        let bw = (inner.w - 5.0 * 10.0) / 6.0;
        for (i, b) in buttons.iter_mut().enumerate() { *b = Rect::new(inner.x + i as f32 * (bw + 10.0), y0 + 134.0, bw, 26.0); }
    } else {
        let cw = inner.w / 3.0;
        for (i, c) in cells.iter_mut().enumerate() { *c = Rect::new(inner.x + (i % 3) as f32 * cw, y0 + 38.0 + (i / 3) as f32 * 52.0, cw, 48.0); }
        bar = Rect::new(inner.x, y0 + 150.0, inner.w, 14.0);
        let bw = (inner.w - 2.0 * 8.0) / 3.0;
        for (i, b) in buttons.iter_mut().enumerate() { *b = Rect::new(inner.x + (i % 3) as f32 * (bw + 8.0), y0 + 184.0 + (i / 3) as f32 * 32.0, bw, 26.0); }
    }
    Layout { panel, cells, bar, buttons }
}

fn text_db(v: f32, relative_to: Option<f32>) -> String {
    if v <= -100.0 { return "−−.−".into(); }
    let v = v - relative_to.unwrap_or(0.0);
    let s = format!("{:.1}", v.abs());
    if v < -0.049 { format!("−{s}") } else if relative_to.is_some() && v > 0.049 { format!("+{s}") } else { s }
}

fn level_colour(v: f32, target: f32) -> Color {
    if v <= -100.0 { return Color(0xff6c777d); }
    let rel = v - target;
    if rel > 3.0 { Color(0xfffa4a3c) } else if rel > 1.0 { Color(0xfff0c83c) } else if rel >= -1.0 { Color(0xff46e16e) } else { Color(0xff8ab4c8) }
}

fn peak_colour(v: f32, ceiling: f32) -> Color {
    if v <= -100.0 { Color(0xff6c777d) } else if v > ceiling { Color(0xfffa4a3c) } else if v > ceiling - 1.0 { Color(0xfff0c83c) } else { Color(0xff46e16e) }
}

pub fn scale_span(scale: i32) -> (f32, f32) { if scale == 1 { (-36.0, 18.0) } else { (-18.0, 9.0) } }

pub fn paint_panel(scene: &mut Scene, meter: &Meter, face_w: f32, y0: f32) {
    let lay = layout(face_w, y0);
    let r = meter.loud.readings();
    let target = meter.loud_target;
    let relative = meter.loud_unit == 1;
    let unit = if relative { "LU" } else { "LUFS" };
    scene.fill_round(lay.panel, 8.0, Color(0xff0c1013), Color(0xff0c1013), true);
    scene.stroke_round(lay.panel, 8.0, Color(0xff465058), 1.5);
    scene.text(Point::new(lay.panel.x + 16.0, y0 + 26.0), "LOUDNESS", Color(0xffd2d8dc), 13.0, false);
    let (scale_lo, scale_hi) = scale_span(meter.loud_scale);
    // A paint text holds 47 bytes, so the header is split in two.
    let info = format!("target {} LUFS   ceiling {} dBTP", text_db(target, None), text_db(meter.loud_ceiling, None));
    scene.text(Point::new(lay.panel.x + 110.0, y0 + 26.0), info, Color(0xff7d878e), 11.0, false);
    let legend = format!("bar: EBU +{}   fill M   white S   cyan I", scale_hi as i32);
    if !narrow(face_w) { scene.text(Point::new(lay.panel.right() - 14.0 - legend.chars().count() as f32 * 5.6, y0 + 26.0), legend, Color(0xff6c777d), 10.5, false); }

    let window = format!("SLIDING {} s", meter.loud_window.round() as i32);
    let reference = if relative { Some(target) } else { None };
    let items: [(String, String, Color); 6] = [
        (format!("MOMENTARY {unit}"), text_db(r.momentary, reference), level_colour(r.momentary, target)),
        (format!("SHORT-TERM 3 s {unit}"), text_db(r.short_term, reference), level_colour(r.short_term, target)),
        (format!("{window} {unit}"), text_db(r.sliding, reference), level_colour(r.sliding, target)),
        (format!("INTEGRATED {unit}"), text_db(r.integrated, reference), level_colour(r.integrated, target)),
        (if r.range_stable == 1 { "LOUDNESS RANGE LU".into() } else { "RANGE LU (settling)".into() }, if r.integrated <= -100.0 { "−−.−".into() } else { format!("{:.1}", r.range) }, if r.range_stable == 1 { Color(0xffc8d0d4) } else { Color(0xff6c777d) }),
        ("TRUE PEAK dBTP".into(), text_db(r.true_peak, None), peak_colour(r.true_peak, meter.loud_ceiling)),
    ];
    let size = if narrow(face_w) { 26.0 } else { 34.0 };
    for (cell, (label, value, colour)) in lay.cells.iter().zip(items.iter()) {
        scene.text(Point::new(cell.x, cell.y + 12.0), label.as_str(), Color(0xff7d878e), 10.5, false);
        scene.text(Point::new(cell.x, cell.y + 12.0 + size * 0.95), value.as_str(), *colour, size, false);
    }

    // The bar: relative to the target on the chosen EBU scale. Fill = momentary, white tick = short-term, cyan tick = integrated.
    let bar = lay.bar;
    let x_of = |lu: f32| bar.x + ((lu - scale_lo) / (scale_hi - scale_lo)).clamp(0.0, 1.0) * bar.w;
    scene.fill_rect(bar, Color(0xff14181b), Color(0xff14181b), true);
    scene.stroke_round(bar, 2.0, Color(0xff3c454b), 1.0);
    let mom = r.momentary - target;
    if r.momentary > -100.0 {
        let split = x_of(0.0).min(x_of(mom));
        scene.fill_rect(Rect::new(bar.x + 1.0, bar.y + 2.0, (split - bar.x - 1.0).max(0.0), bar.h - 4.0), Color(0xff3cc864), Color(0xff2a9a4a), true);
        if mom > 0.0 {
            let end = x_of(mom.min(scale_hi));
            let amber = x_of(1.0).min(end);
            scene.fill_rect(Rect::new(x_of(0.0), bar.y + 2.0, amber - x_of(0.0), bar.h - 4.0), Color(0xfff0c83c), Color(0xffc09a20), true);
            if end > amber { scene.fill_rect(Rect::new(amber, bar.y + 2.0, end - amber, bar.h - 4.0), Color(0xfffa4a3c), Color(0xffc03028), true); }
        }
    }
    let step = if meter.loud_scale == 1 { 6.0 } else { 3.0 };
    let mut lu = (scale_lo / step).ceil() * step;
    while lu <= scale_hi + 0.01 {
        let x = x_of(lu);
        scene.fill_rect(Rect::new(x - 0.5, bar.bottom(), 1.0, 5.0), Color(0xff7d878e), Color(0xff7d878e), true);
        let t = if lu.abs() < 0.01 { "0".to_string() } else { format!("{:+}", lu as i32) };
        scene.text(Point::new(x, bar.bottom() + 16.0), t, Color(0xff7d878e), 10.0, true);
        lu += step;
    }
    if r.short_term > -100.0 { scene.fill_rect(Rect::new(x_of(r.short_term - target) - 1.0, bar.y - 3.0, 2.0, bar.h + 6.0), Color(0xffffffff), Color(0xffffffff), true); }
    if r.integrated > -100.0 { scene.fill_rect(Rect::new(x_of(r.integrated - target) - 1.0, bar.y - 3.0, 2.0, bar.h + 6.0), Color(0xff4cd2ff), Color(0xff4cd2ff), true); }
    scene.fill_rect(Rect::new(x_of(0.0) - 1.5, bar.y - 5.0, 3.0, bar.h + 10.0), Color(0xfff2f6f3), Color(0xfff2f6f3), true);

    let captions = [
        ("Reset I / LRA".to_string(), false),
        (format!("Target {}", text_db(target, None)), true),
        (format!("Ceiling {}", text_db(meter.loud_ceiling, None)), true),
        (format!("Window {} s", meter.loud_window.round() as i32), true),
        (format!("EBU +{}", scale_hi as i32), true),
        (if relative { "LU".to_string() } else { "LUFS".to_string() }, relative),
    ];
    for (rect, (caption, on)) in lay.buttons.iter().zip(captions.iter()) { value_chip(scene, *rect, caption, *on); }
}

/// 0 = not on the panel, 1 = reset (done here), 2 = a setting changed.
pub fn click(meter: &mut Meter, x: f32, y: f32) -> i32 {
    let (face_w, face_h) = view_size_for(meter.face, meter.skin);
    let lay = layout(face_w, face_h);
    let p = Point::new(x, y);
    let Some(index) = lay.buttons.iter().position(|b| b.contains(p)) else { return 0 };
    let next = |list: &[f32], v: f32| -> f32 {
        match list.iter().position(|t| (t - v).abs() < 0.05) { Some(i) => list[(i + 1) % list.len()], None => list[0] }
    };
    match index {
        0 => { meter.loud.reset(); return 1; }
        1 => meter.loud_target = next(&TARGETS, meter.loud_target),
        2 => meter.loud_ceiling = next(&CEILINGS, meter.loud_ceiling),
        3 => { meter.loud_window = next(&WINDOWS, meter.loud_window); meter.loud.set_window(meter.loud_window); }
        4 => meter.loud_scale = 1 - meter.loud_scale.clamp(0, 1),
        _ => meter.loud_unit = 1 - meter.loud_unit.clamp(0, 1),
    }
    2
}

pub fn floor_reading() -> f32 { FLOOR }
