//! Wiener Blut — Interactive music sheet with Bravura SMuFL glyphs.
//!
//! Displays a simplified arrangement of Johann Strauss II's "Wiener Blut"
//! Op. 354 using Bravura font glyphs rendered via ab_glyph on a Cairo surface.
//!
//! Run with:
//! ```sh
//! cargo run --example wiener_blut --features gtk,render,embed-fonts
//! ```

use std::cell::RefCell;
use std::rc::Rc;

use ab_glyph::Font;
use ab_glyph::FontVec;
use ab_glyph::OutlineCurve;

use fonts_rs_bravura::fonts;
use fonts_rs_bravura::register_glyphs;

use gtk4::cairo;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, DrawingArea, Label, Orientation, Scale};

use miette::Result;

const APP_ID: &str = "io.smearor.fonts_rs.wiener_blut";

/// SMuFL codepoints for the glyphs we need.
mod cp {
    pub const GCLEF: char = '\u{E050}';
    pub const TIMESIG_3: char = '\u{E083}';
    pub const TIMESIG_4: char = '\u{E084}';
    pub const NOTEHEAD_BLACK: char = '\u{E0A4}';
    pub const NOTEHEAD_HALF: char = '\u{E0A3}';
    pub const NOTEHEAD_WHOLE: char = '\u{E0A2}';
    pub const REST_QUARTER: char = '\u{E4E5}';
    pub const REST_HALF: char = '\u{E4E4}';
    pub const REST_WHOLE: char = '\u{E4E3}';
    pub const REST_8TH: char = '\u{E4E6}';
    pub const AUGMENTATION_DOT: char = '\u{E1E7}';
    pub const FLAG_8TH_UP: char = '\u{E240}';
    pub const FLAG_8TH_DOWN: char = '\u{E241}';
}

/// Note duration types.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Duration {
    Whole,
    Half,
    Quarter,
    Eighth,
}

/// A note or rest in the score.
#[derive(Clone, Debug, PartialEq)]
enum Event {
    Note {
        pitch: i32,
        duration: Duration,
        dotted: bool,
    },
    Rest {
        duration: Duration,
    },
    Barline,
    DoubleBar,
    FinalBar,
}

/// A system is one staff line with events.
type System = Vec<Event>;

/// A page contains multiple systems.
type Page = Vec<System>;

/// Helper to build a measure of 3 quarter notes.
fn m3(a: i32, b: i32, c: i32) -> Vec<Event> {
    vec![
        Event::Note { pitch: a, duration: Duration::Quarter, dotted: false },
        Event::Note { pitch: b, duration: Duration::Quarter, dotted: false },
        Event::Note { pitch: c, duration: Duration::Quarter, dotted: false },
        Event::Barline,
    ]
}

/// Helper: half note + quarter note.
fn hq(a: i32, b: i32) -> Vec<Event> {
    vec![
        Event::Note { pitch: a, duration: Duration::Half, dotted: false },
        Event::Note { pitch: b, duration: Duration::Quarter, dotted: false },
        Event::Barline,
    ]
}

/// Helper: dotted half + quarter.
fn dhq(a: i32, b: i32) -> Vec<Event> {
    vec![
        Event::Note { pitch: a, duration: Duration::Half, dotted: true },
        Event::Note { pitch: b, duration: Duration::Quarter, dotted: false },
        Event::Barline,
    ]
}

/// Helper: rest + 2 quarters.
fn rqq(a: i32, b: i32) -> Vec<Event> {
    vec![
        Event::Rest { duration: Duration::Quarter },
        Event::Note { pitch: a, duration: Duration::Quarter, dotted: false },
        Event::Note { pitch: b, duration: Duration::Quarter, dotted: false },
        Event::Barline,
    ]
}

/// Build a system from measure fragments.
fn sys(measures: &[&[Event]]) -> System {
    let mut events = Vec::new();
    for m in measures {
        events.extend(m.iter().cloned());
    }
    // Remove trailing barline from last measure if we'll add DoubleBar/FinalBar
    if events.last() == Some(&Event::Barline) {
        events.pop();
    }
    events
}

/// Complete arrangement of "Wiener Blut" Walzer Op. 354 by Johann Strauss II.
/// Simplified melody in 3/4 time, 5 sections: Intro, Waltz 1-4, Coda.
fn wiener_blut_score() -> Vec<Page> {
    // Section 1: Introduction (4 measures)
    let intro = vec![
        sys(&[
            &rqq(2, 4),
            &hq(6, 4),
        ]),
        sys(&[
            &m3(2, 4, 6),
            &[Event::Note { pitch: 7, duration: Duration::Half, dotted: false }, Event::Rest { duration: Duration::Quarter }, Event::DoubleBar],
        ]),
    ];

    // Waltz 1: Theme A (8 measures) + Theme A' (8 measures)
    let waltz1 = vec![
        sys(&[
            &m3(2, 4, 6),
            &hq(7, 6),
            &m3(4, 6, 7),
            &hq(9, 7),
        ]),
        sys(&[
            &m3(6, 4, 2),
            &hq(0, 2),
            &m3(4, 6, 7),
            &dhq(9, 7),
        ]),
        sys(&[
            &hq(11, 9),
            &m3(7, 6, 4),
            &hq(2, 4),
            &[Event::Rest { duration: Duration::Quarter }, Event::Note { pitch: 2, duration: Duration::Quarter, dotted: false }, Event::Note { pitch: 4, duration: Duration::Quarter, dotted: false }, Event::Barline],
        ]),
        sys(&[
            &m3(6, 7, 9),
            &dhq(11, 9),
            &m3(7, 6, 4),
            &[Event::Note { pitch: 2, duration: Duration::Whole, dotted: false }, Event::DoubleBar],
        ]),
    ];

    // Waltz 2: Theme B (8 measures) + Theme B' (8 measures)
    let waltz2 = vec![
        sys(&[
            &m3(7, 9, 11),
            &hq(12, 11),
            &m3(9, 7, 6),
            &hq(4, 6),
        ]),
        sys(&[
            &m3(7, 9, 11),
            &dhq(12, 11),
            &m3(9, 7, 6),
            &hq(4, 2),
        ]),
        sys(&[
            &m3(4, 6, 7),
            &dhq(9, 7),
            &m3(6, 4, 2),
            &hq(0, 2),
        ]),
        sys(&[
            &m3(4, 6, 7),
            &dhq(9, 11),
            &m3(12, 11, 9),
            &[Event::Note { pitch: 7, duration: Duration::Whole, dotted: false }, Event::DoubleBar],
        ]),
    ];

    // Waltz 3: Theme C (8 measures) + Theme C' (8 measures)
    let waltz3 = vec![
        sys(&[
            &m3(9, 11, 12),
            &hq(14, 12),
            &m3(11, 9, 7),
            &hq(6, 9),
        ]),
        sys(&[
            &m3(11, 12, 14),
            &dhq(16, 14),
            &m3(12, 11, 9),
            &hq(7, 6),
        ]),
        sys(&[
            &m3(4, 6, 7),
            &dhq(9, 11),
            &m3(12, 14, 16),
            &hq(14, 12),
        ]),
        sys(&[
            &m3(11, 9, 7),
            &dhq(6, 4),
            &m3(2, 4, 6),
            &[Event::Note { pitch: 7, duration: Duration::Whole, dotted: false }, Event::DoubleBar],
        ]),
    ];

    // Waltz 4: Theme D (8 measures) + Theme D' (8 measures)
    let waltz4 = vec![
        sys(&[
            &m3(6, 7, 9),
            &dhq(11, 9),
            &m3(7, 6, 4),
            &hq(2, 4),
        ]),
        sys(&[
            &m3(6, 7, 9),
            &dhq(11, 14),
            &m3(12, 11, 9),
            &hq(7, 6),
        ]),
        sys(&[
            &m3(4, 6, 7),
            &dhq(9, 11),
            &m3(12, 14, 16),
            &dhq(14, 12),
        ]),
        sys(&[
            &m3(11, 9, 7),
            &dhq(6, 4),
            &m3(2, 4, 6),
            &[Event::Note { pitch: 7, duration: Duration::Whole, dotted: false }, Event::DoubleBar],
        ]),
    ];

    // Coda (8 measures)
    let coda = vec![
        sys(&[
            &m3(2, 4, 6),
            &hq(7, 6),
            &m3(4, 6, 7),
            &dhq(9, 7),
        ]),
        sys(&[
            &m3(6, 4, 2),
            &hq(0, 2),
            &m3(4, 6, 7),
            &[Event::Note { pitch: 2, duration: Duration::Whole, dotted: false }, Event::Barline, Event::Rest { duration: Duration::Whole }, Event::FinalBar],
        ]),
    ];

    // Combine into pages (4 systems per page)
    let all_systems: Vec<System> = intro
        .into_iter()
        .chain(waltz1)
        .chain(waltz2)
        .chain(waltz3)
        .chain(waltz4)
        .chain(coda)
        .collect();

    let systems_per_page = 4;
    all_systems
        .chunks(systems_per_page)
        .map(|chunk| chunk.to_vec())
        .collect()
}

struct AppState {
    pages: Vec<Page>,
    current_page: usize,
    current_note: usize,
    playing: bool,
    tempo_bpm: f64,
}

/// Count total events across all systems on a page.
fn page_event_count(page: &Page) -> usize {
    page.iter().map(|sys| sys.len()).sum()
}

/// Duration of an event in milliseconds, scaled by tempo (BPM = beats per minute).
/// At 180 BPM (60 bars/min in 3/4), one quarter note = 333ms.
fn event_duration_ms_tempo(event: &Event, bpm: f64) -> u32 {
    // One beat = 60_000 / bpm ms. Quarter note = 1 beat.
    let beat_ms = 60_000.0 / bpm;
    match event {
        Event::Note { duration, dotted, .. } => {
            let base = match duration {
                Duration::Whole => beat_ms * 4.0,
                Duration::Half => beat_ms * 2.0,
                Duration::Quarter => beat_ms,
                Duration::Eighth => beat_ms * 0.5,
            } as u32;
            if *dotted { base + base / 2 } else { base }
        }
        Event::Rest { duration } => {
            let base = match duration {
                Duration::Whole => beat_ms * 4.0,
                Duration::Half => beat_ms * 2.0,
                Duration::Quarter => beat_ms,
                Duration::Eighth => beat_ms * 0.5,
            } as u32;
            base
        }
        Event::Barline | Event::DoubleBar | Event::FinalBar => 0,
    }
}

fn main() -> Result<()> {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
    Ok(())
}

fn build_ui(app: &Application) {
    if let Err(e) = register_glyphs() {
        eprintln!("Failed to register glyphs: {e}");
    }

    let state = Rc::new(RefCell::new(AppState {
        pages: wiener_blut_score(),
        current_page: 0,
        current_note: 0,
        playing: false,
        tempo_bpm: 180.0,
    }));

    let drawing_area = DrawingArea::builder()
        .width_request(900)
        .height_request(500)
        .build();

    let state_for_draw = state.clone();
    drawing_area.set_draw_func(move |_area, cr, width, height| {
        draw_score(&state_for_draw, cr, width, height);
    });

    let title_label = Label::builder()
        .label("<span size=\"large\" weight=\"bold\">Wiener Blut</span>\n<span size=\"medium\">Walzer, Op. 354 — Johann Strauss II</span>")
        .use_markup(true)
        .build();

    let page_label = Label::builder().build();
    let page_label = Rc::new(page_label);

    let update_page_label = {
        let state = state.clone();
        let label = page_label.clone();
        move || {
            let s = state.borrow();
            label.set_label(&format!(
                "Seite {} von {}",
                s.current_page + 1,
                s.pages.len()
            ));
        }
    };
    update_page_label();
    let update_page_label = Rc::new(update_page_label);

    let prev_btn = gtk4::Button::with_label("◀ Vorige Seite");
    let next_btn = gtk4::Button::with_label("Nächste Seite ▶");
    let play_btn = gtk4::Button::with_label("▶ Abspielen");
    let play_btn = Rc::new(play_btn);

    {
        let state = state.clone();
        let da = drawing_area.clone();
        let update = update_page_label.clone();
        prev_btn.connect_clicked(move |_| {
            {
                let mut s = state.borrow_mut();
                if s.current_page > 0 {
                    s.current_page -= 1;
                    s.current_note = 0;
                }
            }
            da.queue_draw();
            update();
        });
    }

    {
        let state = state.clone();
        let da = drawing_area.clone();
        let update = update_page_label.clone();
        next_btn.connect_clicked(move |_| {
            {
                let mut s = state.borrow_mut();
                if s.current_page < s.pages.len() - 1 {
                    s.current_page += 1;
                    s.current_note = 0;
                }
            }
            da.queue_draw();
            update();
        });
    }

    // Play/Stop button
    {
        let state = state.clone();
        let da = drawing_area.clone();
        let update = update_page_label.clone();
        let btn = play_btn.clone();
        let btn_for_closure = play_btn.clone();
        btn.connect_clicked(move |_| {
            let mut s = state.borrow_mut();
            s.playing = !s.playing;
            let playing = s.playing;
            drop(s);
            if playing {
                btn_for_closure.set_label("⏸ Stop");
                schedule_next_note(state.clone(), da.clone(), update.clone(), btn_for_closure.clone());
            } else {
                btn_for_closure.set_label("▶ Abspielen");
            }
        });
    }

    // Tempo slider: 174–186 BPM (58–62 bars/min in 3/4)
    // Upper marks: Takte/Min, lower marks: BPM
    let tempo_label = Label::builder()
        .label("Tempo: 180 BPM (60 Takte/Min)")
        .halign(gtk4::Align::Center)
        .build();
    let tempo_label = Rc::new(tempo_label);

    let tempo_marks_top = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(0)
        .hexpand(true)
        .margin_start(8)
        .margin_end(8)
        .build();
    for bars in [58, 59, 60, 61, 62] {
        let lbl = Label::builder()
            .label(&format!("{bars}"))
            .hexpand(true)
            .halign(gtk4::Align::Center)
            .css_classes(["caption"])
            .build();
        tempo_marks_top.append(&lbl);
    }

    let tempo_scale = Scale::with_range(gtk4::Orientation::Horizontal, 174.0, 186.0, 1.0);
    tempo_scale.set_value(180.0);
    tempo_scale.set_hexpand(true);
    tempo_scale.set_draw_value(false);

    // Add tick marks at each BPM value
    for bpm in [174, 177, 180, 183, 186] {
        tempo_scale.add_mark(bpm as f64, gtk4::PositionType::Bottom, None);
    }

    {
        let state = state.clone();
        let label = tempo_label.clone();
        tempo_scale.connect_value_changed(move |scale| {
            let bpm = scale.value();
            let bars = bpm / 3.0;
            {
                let mut s = state.borrow_mut();
                s.tempo_bpm = bpm;
            }
            label.set_label(&format!("Tempo: {:.0} BPM ({:.0} Takte/Min)", bpm, bars));
        });
    }

    let tempo_marks_bottom = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(0)
        .hexpand(true)
        .margin_start(8)
        .margin_end(8)
        .build();
    for bpm in [174, 177, 180, 183, 186] {
        let lbl = Label::builder()
            .label(&format!("{bpm}"))
            .hexpand(true)
            .halign(gtk4::Align::Center)
            .css_classes(["caption"])
            .build();
        tempo_marks_bottom.append(&lbl);
    }

    let tempo_container = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(2)
        .margin_start(40)
        .margin_end(40)
        .build();
    tempo_container.append(&*tempo_label);
    tempo_container.append(&tempo_marks_top);
    tempo_container.append(&tempo_scale);
    tempo_container.append(&tempo_marks_bottom);

    let nav_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .halign(gtk4::Align::Center)
        .build();
    nav_box.append(&prev_btn);
    nav_box.append(&*page_label);
    nav_box.append(&next_btn);
    nav_box.append(&*play_btn);

    let main_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .margin_start(12)
        .margin_end(12)
        .margin_top(8)
        .margin_bottom(8)
        .build();
    main_box.append(&title_label);
    main_box.append(&drawing_area);
    main_box.append(&nav_box);
    main_box.append(&tempo_container);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Wiener Blut — Bravura SMuFL Demo")
        .default_width(950)
        .default_height(600)
        .child(&main_box)
        .build();

    window.present();
}

/// Get the event at a global index on a page.
fn get_event_at(page: &Page, global_idx: usize) -> Option<&Event> {
    let mut idx = global_idx;
    for system in page {
        if idx < system.len() {
            return Some(&system[idx]);
        }
        idx -= system.len();
    }
    None
}

/// Map a diatonic pitch index to a frequency in Hz.
/// pitch 0 = C4 (261.63 Hz), each increment = one diatonic step up.
fn pitch_to_freq(pitch: i32) -> f32 {
    let semitones_from_c = [0_i32, 2, 4, 5, 7, 9, 11];
    let octave = pitch.div_euclid(7);
    let step = pitch.rem_euclid(7) as usize;
    let midi_note = 60 + octave * 12 + semitones_from_c[step];
    440.0_f32 * 2.0_f32.powf((midi_note - 69) as f32 / 12.0)
}

/// Generate a violin-like tone using additive synthesis with harmonics,
/// vibrato, and a bowing envelope. Plays via `aplay`.
fn play_tone(freq: f32, duration_ms: u32) {
    let sample_rate = 44100_u32;
    let num_samples = (sample_rate * duration_ms / 1000) as usize;
    let duration_s = duration_ms as f32 / 1000.0;

    let mut wav = Vec::with_capacity(44 + num_samples * 2);
    let data_size = (num_samples * 2) as u32;
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36 + data_size).to_le_bytes());
    wav.extend_from_slice(b"WAVE");
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16_u32.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes());
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&(sample_rate * 2).to_le_bytes());
    wav.extend_from_slice(&2_u16.to_le_bytes());
    wav.extend_from_slice(&16_u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_size.to_le_bytes());

    // Violin-like harmonic amplitudes (odd + even harmonics, decreasing)
    let harmonics: [(f32, f32); 7] = [
        (1.0, 1.0),
        (2.0, 0.45),
        (3.0, 0.28),
        (4.0, 0.18),
        (5.0, 0.12),
        (6.0, 0.08),
        (7.0, 0.05),
    ];

    // Bowing envelope: slow attack, gentle sustain, soft release
    let attack = (0.04_f32).min(duration_s * 0.3);
    let release = (0.08_f32).min(duration_s * 0.3);
    let vibrato_rate = 5.5_f32; // Hz
    let vibrato_depth = 0.006_f32; // 0.6% frequency modulation

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;

        // Envelope
        let envelope = if t < attack {
            (t / attack).powi(2)
        } else if t > duration_s - release {
            let r = ((duration_s - t) / release).max(0.0);
            r * r
        } else {
            1.0
        };

        // Vibrato: smooth frequency modulation
        let vibrato = 1.0 + vibrato_depth * (2.0 * std::f32::consts::PI * vibrato_rate * t).sin();
        let modulated_freq = freq * vibrato;

        // Additive synthesis: sum of harmonics
        let mut sample = 0.0_f32;
        for (n, amp) in harmonics {
            sample += amp * (n * modulated_freq * 2.0 * std::f32::consts::PI * t).sin();
        }
        sample *= envelope * 0.12;

        let sample_i16 = (sample * 32767.0).clamp(-32767.0, 32767.0) as i16;
        wav.extend_from_slice(&sample_i16.to_le_bytes());
    }

    std::thread::spawn(move || {
        use std::io::Write;
        use std::process::{Command, Stdio};
        let result = Command::new("aplay")
            .arg("-q")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
        if let Ok(mut child) = result {
            if let Some(mut stdin) = child.stdin.take() {
                let _ = stdin.write_all(&wav);
            }
            let _ = child.wait();
        }
    });
}

/// Play the audio for an event if it is a note.
fn play_event_sound(event: &Event, bpm: f64) {
    if let Event::Note { pitch, duration: _, dotted: _ } = event {
        let base_ms = event_duration_ms_tempo(event, bpm);
        let freq = pitch_to_freq(*pitch);
        play_tone(freq, base_ms.max(80));
    }
}

/// Schedule the next note advance based on the current event's duration.
fn schedule_next_note(
    state: Rc<RefCell<AppState>>,
    da: DrawingArea,
    update: Rc<dyn Fn()>,
    btn: Rc<gtk4::Button>,
) {
    // Get the current event and its duration
    let (duration_ms, at_end, current_event, tempo) = {
        let s = state.borrow();
        let page = &s.pages[s.current_page];
        let count = page_event_count(page);
        let event = get_event_at(page, s.current_note).cloned();
        let ms = event.as_ref().map(|e| event_duration_ms_tempo(e, s.tempo_bpm)).unwrap_or(500);
        (ms, s.current_note >= count, event, s.tempo_bpm)
    };

    // Play the current note's tone
    if let Some(ref event) = current_event {
        play_event_sound(event, tempo);
    }

    // If we're at the end of the page, advance to the next page
    if at_end {
        let mut s = state.borrow_mut();
        if s.current_page < s.pages.len() - 1 {
            s.current_page += 1;
            s.current_note = 0;
        } else {
            // End of piece — stop playback
            s.playing = false;
            drop(s);
            btn.set_label("▶ Abspielen");
            da.queue_draw();
            update();
            return;
        }
        drop(s);
        da.queue_draw();
        update();
    }

    glib::timeout_add_local_once(std::time::Duration::from_millis(duration_ms.max(50) as u64), move || {
        let (still_playing, next_event) = {
            let mut s = state.borrow_mut();
            if !s.playing {
                (false, None)
            } else {
                s.current_note += 1;
                let page = &s.pages[s.current_page];
                let count = page_event_count(page);
                if s.current_note >= count {
                    if s.current_page < s.pages.len() - 1 {
                        s.current_page += 1;
                        s.current_note = 0;
                    } else {
                        s.playing = false;
                    }
                }
                let event = get_event_at(&s.pages[s.current_page], s.current_note).cloned();
                (s.playing, event)
            }
        };

        da.queue_draw();
        update();

        if still_playing {
            // Play the next note's tone immediately
            if let Some(ref event) = next_event {
                let bpm = state.borrow().tempo_bpm;
                play_event_sound(event, bpm);
            }
            schedule_next_note(state, da, update, btn);
        } else {
            btn.set_label("▶ Abspielen");
        }
    });
}

fn draw_score(state: &Rc<RefCell<AppState>>, cr: &cairo::Context, width: i32, height: i32) {
    let s = state.borrow();
    let page = &s.pages[s.current_page];

    let font = match fonts::font() {
        Some(f) => f,
        None => return,
    };

    let w = width as f64;
    let h = height as f64;

    // Background — warm paper color
    cr.set_source_rgb(0.98, 0.96, 0.92);
    cr.paint().unwrap_or(());

    // Layout parameters
    let staff_space = 8.0_f64;
    let staff_line_height = staff_space * 4.0;
    let left_margin = 50.0_f64;
    let right_margin = w - 50.0;
    let top_margin = 40.0_f64;
    let system_spacing = staff_line_height + 50.0;

    let mut global_event_idx = 0usize;

    for (sys_idx, system) in page.iter().enumerate() {
        let staff_y = top_margin + (sys_idx as f64) * system_spacing;

        // Draw staff lines (5 lines)
        cr.set_source_rgb(0.0, 0.0, 0.0);
        cr.set_line_width(0.6);
        for i in 0..5 {
            let y = staff_y + (i as f64) * staff_space;
            cr.move_to(left_margin, y);
            cr.line_to(right_margin, y);
            cr.stroke().unwrap_or(());
        }

        // Draw clef at the start of each system
        draw_glyph(cr, font, cp::GCLEF, left_margin, staff_y, staff_space * 4.0);

        // Draw time signature (3/4) only on first system of first page
        if s.current_page == 0 && sys_idx == 0 {
            let ts_x = left_margin + 50.0;
            draw_glyph(cr, font, cp::TIMESIG_3, ts_x, staff_y, staff_space * 4.0);
            draw_glyph(cr, font, cp::TIMESIG_4, ts_x, staff_y + staff_space * 2.0, staff_space * 4.0);
        }

        // Draw events for this system
        let mut x = left_margin + 90.0;
        let note_spacing = 34.0_f64;

        for event in system {
            let is_current = global_event_idx == s.current_note;
            let color = if is_current {
                (0.8, 0.2, 0.2)
            } else {
                (0.0, 0.0, 0.0)
            };

            match event {
                Event::Barline => {
                    cr.set_source_rgb(color.0, color.1, color.2);
                    cr.set_line_width(0.8);
                    cr.move_to(x, staff_y);
                    cr.line_to(x, staff_y + staff_line_height);
                    cr.stroke().unwrap_or(());
                    x += 12.0;
                }
                Event::DoubleBar => {
                    cr.set_source_rgb(0.0, 0.0, 0.0);
                    cr.set_line_width(0.8);
                    cr.move_to(x, staff_y);
                    cr.line_to(x, staff_y + staff_line_height);
                    cr.stroke().unwrap_or(());
                    cr.move_to(x + 4.0, staff_y);
                    cr.line_to(x + 4.0, staff_y + staff_line_height);
                    cr.stroke().unwrap_or(());
                    x += 20.0;
                }
                Event::FinalBar => {
                    cr.set_source_rgb(0.0, 0.0, 0.0);
                    cr.set_line_width(0.8);
                    cr.move_to(x, staff_y);
                    cr.line_to(x, staff_y + staff_line_height);
                    cr.stroke().unwrap_or(());
                    cr.set_line_width(2.5);
                    cr.move_to(x + 5.0, staff_y);
                    cr.line_to(x + 5.0, staff_y + staff_line_height);
                    cr.stroke().unwrap_or(());
                    x += 20.0;
                }
                Event::Note { pitch, duration, dotted } => {
                    cr.set_source_rgb(color.0, color.1, color.2);
                    let note_y = staff_y + staff_line_height
                        - (*pitch as f64) * 0.5 * staff_space;
                    let glyph = match duration {
                        Duration::Whole => cp::NOTEHEAD_WHOLE,
                        Duration::Half => cp::NOTEHEAD_HALF,
                        Duration::Quarter | Duration::Eighth => cp::NOTEHEAD_BLACK,
                    };
                    draw_glyph_colored(cr, font, glyph, x, note_y, staff_space * 4.0, color);

                    // Stem for half/quarter/eighth notes
                    if *duration != Duration::Whole {
                        let stem_up = *pitch < 6;
                        let stem_x = if stem_up { x + 14.0 } else { x - 2.0 };
                        let stem_y1 = note_y;
                        let stem_y2 = if stem_up { note_y - 28.0 } else { note_y + 28.0 };
                        cr.set_source_rgb(color.0, color.1, color.2);
                        cr.set_line_width(1.2);
                        cr.move_to(stem_x, stem_y1);
                        cr.line_to(stem_x, stem_y2);
                        cr.stroke().unwrap_or(());

                        // Flag for eighth notes
                        if *duration == Duration::Eighth {
                            let flag_glyph = if stem_up { cp::FLAG_8TH_UP } else { cp::FLAG_8TH_DOWN };
                            draw_glyph_colored(cr, font, flag_glyph, stem_x - 4.0, stem_y2, staff_space * 4.0, color);
                        }
                    }

                    // Augmentation dot
                    if *dotted {
                        draw_glyph_colored(cr, font, cp::AUGMENTATION_DOT, x + 18.0, note_y, staff_space * 4.0, color);
                    }

                    x += note_spacing;
                }
                Event::Rest { duration } => {
                    cr.set_source_rgb(color.0, color.1, color.2);
                    let rest_glyph = match duration {
                        Duration::Whole => cp::REST_WHOLE,
                        Duration::Half => cp::REST_HALF,
                        Duration::Quarter => cp::REST_QUARTER,
                        Duration::Eighth => cp::REST_8TH,
                    };
                    let rest_y = staff_y + staff_line_height * 0.5;
                    draw_glyph_colored(cr, font, rest_glyph, x, rest_y, staff_space * 4.0, color);
                    x += note_spacing;
                }
            }
            global_event_idx += 1;
        }
    }

    // Footer text
    cr.set_source_rgb(0.4, 0.4, 0.4);
    cr.select_font_face("Serif", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
    cr.set_font_size(11.0);
    cr.move_to(left_margin, h - 15.0);
    cr.show_text("Bravura SMuFL — Demo mit ab_glyph Rendering").unwrap_or(());
}

/// Draw a Bravura glyph at (x, y) where y is the top of the glyph.
/// `em_size` is the font size in pixels (staff_space * 4 = one em).
fn draw_glyph(cr: &cairo::Context, font: &FontVec, ch: char, x: f64, y: f64, em_size: f64) {
    draw_glyph_colored(cr, font, ch, x, y, em_size, (0.0, 0.0, 0.0));
}

fn draw_glyph_colored(
    cr: &cairo::Context,
    font: &FontVec,
    ch: char,
    x: f64,
    y: f64,
    em_size: f64,
    color: (f64, f64, f64),
) {
    let glyph_id = font.glyph_id(ch);

    let outline = match font.outline(glyph_id) {
        Some(o) => o,
        None => return,
    };

    let upem = font.units_per_em().unwrap_or(1000.0);
    let scale = em_size as f32 / upem;
    let bounds = &outline.bounds;
    let scale_x = scale;
    let scale_y = -scale;

    cr.save().unwrap_or(());
    cr.set_source_rgb(color.0, color.1, color.2);

    let offset_x = x - bounds.min.x as f64 * scale_x as f64;
    let offset_y = y - bounds.min.y as f64 * scale_y as f64;
    cr.translate(offset_x, offset_y);

    let mut first = true;
    for curve in outline.curves.iter() {
        match curve {
            OutlineCurve::Line(p1, p2) => {
                let x1 = p1.x as f64 * scale_x as f64;
                let y1 = p1.y as f64 * scale_y as f64;
                let x2 = p2.x as f64 * scale_x as f64;
                let y2 = p2.y as f64 * scale_y as f64;
                if first {
                    cr.move_to(x1, y1);
                    first = false;
                }
                cr.line_to(x2, y2);
            }
            OutlineCurve::Quad(p1, p2, p3) => {
                let x1 = p1.x as f64 * scale_x as f64;
                let y1 = p1.y as f64 * scale_y as f64;
                let x2 = p2.x as f64 * scale_x as f64;
                let y2 = p2.y as f64 * scale_y as f64;
                let x3 = p3.x as f64 * scale_x as f64;
                let y3 = p3.y as f64 * scale_y as f64;
                if first {
                    cr.move_to(x1, y1);
                    first = false;
                }
                let c1x = x1 + (2.0 / 3.0) * (x2 - x1);
                let c1y = y1 + (2.0 / 3.0) * (y2 - y1);
                let c2x = x3 + (2.0 / 3.0) * (x2 - x3);
                let c2y = y3 + (2.0 / 3.0) * (y2 - y3);
                cr.curve_to(c1x, c1y, c2x, c2y, x3, y3);
            }
            OutlineCurve::Cubic(p1, p2, p3, p4) => {
                let x1 = p1.x as f64 * scale_x as f64;
                let y1 = p1.y as f64 * scale_y as f64;
                let x2 = p2.x as f64 * scale_x as f64;
                let y2 = p2.y as f64 * scale_y as f64;
                let x3 = p3.x as f64 * scale_x as f64;
                let y3 = p3.y as f64 * scale_y as f64;
                let x4 = p4.x as f64 * scale_x as f64;
                let y4 = p4.y as f64 * scale_y as f64;
                if first {
                    cr.move_to(x1, y1);
                    first = false;
                }
                cr.curve_to(x2, y2, x3, y3, x4, y4);
            }
        }
    }

    cr.fill().unwrap_or(());
    cr.restore().unwrap_or(());
}
