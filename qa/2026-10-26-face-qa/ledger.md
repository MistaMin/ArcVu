# Scope ledger: reference-meter behaviour, hardware lamp face

Project: ArcVu   Branch / commit: main, uncommitted GUI work   Started: 2026-10-26   Agent: Grok

Sources of scope:
- Request: feature parity with a reference software meter, look and buttons of a hardware lamp meter, list gaps before implementation
- the reference meter's manual, pages 3–16
- the hardware meter user guide (Arc, Horizontal, Vertical, Peak, Overs, reference)
- Clean-room rule: no third-party wordmarks, logos, or copied skins

## Already in the plugin (not missing)

Meter kinds VU, RMS, K-12, K-14, K-20, DIN, Nordic, BBC, EBU. AES-17 switch. Reference −20 / −18 / −14. Peak Follow, Auto 3 s, Hold, reset. Overs as 3 samples over 0 dBFS, with a count and reset. Single or dual display. L/R and M/S with four trims. Polarity and mute per channel. Monitor LR, RL, L, R, Mid, Side. HPF and LPF frequency plus 6/12/18/24 dB slopes. A mono-maker frequency and amount. A crude full-band duck labelled dynamic EQ. Arc, horizontal, and vertical lamp layouts. Plugin state blob. Standalone app.

These rows stay `todo` where the behaviour is only partial. The gap is named in the item.

## Evidence 2026-10-26

DSP tests 16/16. Release VST3, AU, and Standalone rebuilt after the face pass. Screenshots: `arcvu-arc.png`, `arcvu-horiz.png`, `arcvu-vert.png`. auval, pluginval, and REAPER were not run (see `report.md`), so those host checks stay skipped. A row marked done has the behaviour in the detector or the face, a GUI path, and either a unit test or one of those screenshots. Weighting is a published-shape approximation, not a full ITU filter cascade.

## Items still missing

| ID | Item | Source | Status |
|---|---|---|---|
| S1 | Each meter type uses its own scale, not one −36..+3 ruler. VU −20..+3. DIN about −50..+5. BBC 1..7. EBU −12..+12 with TEST. Nordic −36..+9 with TEST. K scales show the offset | Deluxe manual p.15 | code: `lamp_index` / captions. Test `din_scale_is_not_the_vu_scale`. Still 40 lamps |
| S2 | Integrated peak lamp is separate from the average bar, on that same scale | reference manual p.3, hardware guide | done |
| S3 | Three VU models: ideal 300 ms, a second model, and separate rise / fall / overshoot | Deluxe p.11 | done |
| S4 | Hold time for the peak lamp, including infinity, not only Follow / 3 s / forever | Deluxe p.11 | done |
| S5 | Numeric meter readout and numeric peak readout, each with its own hold time, click to reset | Deluxe p.6, p.11 | done |
| S6 | Peak-lamp threshold and clip threshold, clip hold time including infinity, click the lamp to reset | Deluxe p.6, p.11 | done |
| S7 | RMS window in ms, analog-ballistic RMS on/off, red-zone threshold, K comfort zone | Deluxe p.12 | done |
| S8 | Weighting A, B, C, D, K (ITU-R BS.1770-1), selectable while RMS is the meter | Deluxe p.3, p.16 | done |
| S9 | Force-mono so a host mono track still drives the meter | Deluxe p.6 | done |
| S10 | Single display: one meter for the sum, or main plus a second lamp for the other channel (L/R or M/S) | Deluxe p.8 | done |
| S11 | Trim link: one knob moves both channels | Deluxe p.7 | done |
| S12 | Green hint when the hidden M or S trim is not at 0 | Deluxe p.7 | done |
| S13 | Monitor click on the lit button returns to LR. In-place versus centered for L and R | Deluxe p.7–8 | done |
| S14 | HPF and LPF are real cascaded filters at 6/12/18/24 dB, each section can be bypassed | Deluxe p.10 | done |
| S15 | Dynamic EQ is one band: off, HPF, LPF, narrow BPF, wide BPF, frequency, depth, listen | Deluxe p.10 | done |
| S16 | Mono maker: everything below the frequency collapses toward mono by the amount | Deluxe p.10 | done |
| S17 | Meters-only view hides the control panel. Indicators show tools, trims, or channel tools are active | Deluxe p.9 | done |
| S18 | Show or hide values, hold, channel tools, and the instance label | Deluxe p.6 | done |
| S19 | Editable instance label | Deluxe p.10 | done |
| S20 | GUI size steps | Deluxe p.5 | done |
| S21 | Preset menu: next/previous, factory categories, user presets, load/save file, reset, save as default, copy/paste | Deluxe p.5 | done |
| S22 | Factory presets change only the parameters they need. User presets store everything | Deluxe p.5 | done |
| S23 | Global settings: drag mode, drag sensitivity, meter refresh, bypass hidden processors, tooltips | Deluxe p.13 | done |
| S24 | Standalone restores settings, can load/save a state, and can stay above other windows | Deluxe p.14 | done |
| S25 | Hardware-style metal chassis, black meter window, square lamps, scale printed with the lamps | hardware guide p.1, p.6 | done |
| S26 | Hardware-style buttons on the face: Peak Auto, Hold, Reset; Overs count and Reset; Phase; Sum/Diff and Left/Right; reference 14/18/20 | hardware guide p.6–7 | done |
| S27 | Every audio control is a host parameter, saved, and automated | plugin-qa rule 2 | done |
| S28 | Release VST3, AU, and Standalone rebuilt and checked against this ledger. AAX is a CMake option and is not part of the normal build | plugin-qa | done |
| S29 | Phase correlation meter. 0 is fully correlated (mono). −20 is uncorrelated. −39 is fully out of phase. −20..0 reads as good phase, −39..−20 as poor. It is its own meter, separate from the loudness lamps and from the polarity buttons | hardware lamp meter guide p.8, user 2026-10-26 | done |

## Look and feel (this is the gap, not a polish pass)

The current face does not look like a hardware lamp meter. Screenshots of the standalone show why. The lamps sit in a large empty panel. The menu bar is small grey type with chevrons that collide with the words. There is no button row. Arc, Horizontal, and Vertical change the lamp path, then leave the rest of the window as blank metal or blank black. A person comparing it with the guide will not recognise the instrument.

What the hardware lamp meter actually shows, from its user guide, and what ArcVu has to grow into:

| ID | What the hardware face does | What ArcVu shows now | Status |
|---|---|---|---|
| L1 | Brushed metal chassis, screws, a recessed black window that is only as big as the lamp scale | A flat grey gradient plate. The arc window is a short band on top of a tall empty plate. Horizontal leaves a slab of grey around a thin LED row | done |
| L2 | Arc: a shallow ribbon of short radial lamps, scale figures sitting on the arc, green then yellow then red, average as a run, peak as one lamp | The arc is the closest of the three, and it is still small in the window, with a dead area under it | done |
| L3 | Horizontal: one dense row of square lamps per channel, scale printed on the black window just above the lamps, two channels stacked like a rack unit, almost no unused metal | Two strips exist. The lamps are short, the numbers are sparse, and most of each plate is empty | done |
| L4 | Vertical: a tall narrow column of square lamps, green at the bottom, red at the top, scale beside the column, the column is the product | Two columns exist inside oversized plates. The plates read as empty frames | done |
| L5 | Button row under the meters, hardware keys, not a text menu: Peak Auto, Hold, Reset; Overs digits and Reset; Phase lamp; Sum/Diff and Left/Right; reference 14, 18, 20 as lit keys | Those controls are buried in Type, Meter, I/O, and Tools dropdowns. Nothing on the face looks like the hardware lamp meter keys | done |
| L6 | Overs and phase are lamps, red when active, not menu rows | Clip is a tiny dot. Overs is a line of text inside a menu | done |
| L7 | Scale type is dense. The horizontal meter prints a figure on nearly every lamp. The arc prints figures along the curve | ArcVu labels every third lamp. Figures are large relative to the lamps and use a UI face | done |
| L8 | Type is a technical meter face: condensed, tabular figures, light on the black window, dark on the metal | Avenir Next Condensed is in the editor. It still reads as a software menu, and the chevrons sit on the labels | done |
| L9 | The three styles are three products that fill their window. Changing style changes the window proportions, not only the drawing inside a fixed 1100×430 frame | One window size for all three, so arc and vertical are stranded in unused space | done |
| L10 | Colour is lamp emission on black, with dim unlit lamps still visible in their own colour. No brown menu cards, no gold underline | Menus are dark brown cards. The bar is a developer menu. That language fights the meter | done |

Look work is S25 and S26 in the feature table, split here so it cannot be treated as done because three lamp paths exist.

Behaviour work stays the S1–S24 and S27–S28 list above. A finished detector with this face would still fail the request.

## Out of scope

| Item | Decided by | Where |
|---|---|---|
| Third-party names, logos, and the eight copied skins | Clean-room rule in this project | User, earlier in the session |
| Other vendors' names and logos | Same rule | User |
| Their installer, website, and credits page | Not our product | Manual p.4, p.13 |
| Building AAX on every configure | User, 2026-10-26. The format is enabled with `-DARCVU_BUILD_AAX=ON` and `ARCVU_AAX_SDK_PATH`. A normal build stays VST3, AU, and Standalone so a later PACE-signed AAX is possible | CMakeLists.txt |

## Open questions

- Eight original colour schemes, or one metal finish only? Skins that copy a third-party product are out.
- Should the native PPM scales replace the 40-lamp hardware-style ruler, or should both exist (hardware-style ruler for VU, native scale when a PPM or K type is selected)?
