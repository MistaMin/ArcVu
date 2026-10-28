#pragma once
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Meter Meter;

typedef struct ArcVuLevels {
    float vu_db;
    float peak_db;
    float vu_pos;
    float peak_pos;
} ArcVuLevels;

Meter* arcvu_create(double sample_rate);
void arcvu_destroy(Meter* meter);
void arcvu_set_sample_rate(Meter* meter, double sample_rate);
void arcvu_set_reference_db(Meter* meter, float db);
void arcvu_set_slow_mode(Meter* meter, int on);
void arcvu_set_theme(Meter* meter, int theme);
void arcvu_set_skin(Meter* meter, int skin);
void arcvu_set_peak_scale(Meter* meter, int scale);
typedef struct ArcVuLoudSettings {
    int on;
    float target, ceiling;
    int scale, unit;
    float window;
} ArcVuLoudSettings;
typedef struct ArcVuLoudness {
    float momentary, short_term, sliding, integrated, range, true_peak, max_momentary, max_short_term;
    int range_stable;
    float elapsed;
    float sample_peak;
} ArcVuLoudness;
void arcvu_set_loudness(Meter* meter, const ArcVuLoudSettings* settings);
void arcvu_get_loudness(const Meter* meter, ArcVuLoudSettings* out);
void arcvu_loudness(const Meter* meter, ArcVuLoudness* out);
void arcvu_reset_loudness(Meter* meter);
float arcvu_loudness_height(int face);
int arcvu_loudness_click(Meter* meter, float x, float y);
// Offline bounce log: measure the output while on, then read the totals and one row per second.
void arcvu_bounce(Meter* meter, int on);
void arcvu_bounce_readings(const Meter* meter, ArcVuLoudness* out, uint64_t* frames);
int arcvu_bounce_rows(const Meter* meter, float* out, int cap);
typedef struct ArcVuSetup {
    int kind, face, single, mid_side, aes17, peak_mode;
    float rms_ms, clip_db, reference_db;
    float trim_l, trim_r, trim_m, trim_s;
    int pol_l, pol_r, mute_l, mute_r, monitor;
    float hpf_hz;
    int hpf_poles;
    float lpf_hz;
    int lpf_poles;
    float mono_hz, mono_amt, dyn_hz, dyn_depth;
    int dyn_on;
    int panel;
    int vu_model;
    float rise_ms, fall_ms, overshoot;
    float peak_hold_s, value_hold_s, clip_hold_s;
    float peak_thresh_db, red_db, comfort_db;
    int analog_rms, weight, force_mono, trim_link, in_place;
    int show_values, show_hold, show_tools, show_label, meters_only;
    int dyn_type, dyn_listen, gui_scale;
    unsigned char label[32];
    float drag_sens;
    int refresh_hz, bypass_hidden, tooltips, always_on_top;
} ArcVuSetup;

void arcvu_setup(Meter* meter, const ArcVuSetup* setup);
void arcvu_set_peak_mode(Meter* meter, int mode);
void arcvu_reset_peaks(Meter* meter);
void arcvu_reset_overs(Meter* meter);
void arcvu_reset_clip(Meter* meter);
void arcvu_view_size(int face, float* w, float* h);
int arcvu_overs(const Meter* meter);
void arcvu_process(Meter* meter, const float* interleaved, int frames, int channels);
void arcvu_levels(const Meter* meter, int channel, ArcVuLevels* out);

typedef struct ArcVuPaint {
    int kind;
    float x, y, w, h, x2, y2, radius, width;
    unsigned int c0, c1;
    int flags, npts;
    float pts[64];
    unsigned char text[48];
} ArcVuPaint;

int arcvu_paint(const Meter* meter, float reference_norm, ArcVuPaint* out, int cap);

#ifdef __cplusplus
}
#endif
