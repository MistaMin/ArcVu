#include "PluginEditor.h"
#include "arcvu.h"
#include <cstring>
#include <vector>

namespace {
juce::Colour ink(unsigned int argb) { return juce::Colour(argb); }

void fillGradient(juce::Graphics& g, juce::Rectangle<float> r, unsigned int c0, unsigned int c1, bool diagonal, bool vertical) {
    if (c0 == c1) { g.setColour(ink(c0)); return; }
    auto a = ink(c0);
    auto b = ink(c1);
    if (diagonal)
        g.setGradientFill(juce::ColourGradient(a, r.getX(), r.getY(), b, r.getRight(), r.getBottom(), false));
    else if (vertical)
        g.setGradientFill(juce::ColourGradient(a, r.getX(), r.getY(), b, r.getX(), r.getBottom(), false));
    else
        g.setGradientFill(juce::ColourGradient(a, r.getX(), r.getY(), b, r.getRight(), r.getY(), false));
}

void replay(juce::Graphics& g, const ArcVuPaint& cmd) {
    auto rect = juce::Rectangle<float>(cmd.x, cmd.y, cmd.w, cmd.h);
    switch (cmd.kind) {
        case 1:
            fillGradient(g, rect, cmd.c0, cmd.c1, cmd.flags != 0, false);
            g.fillEllipse(rect);
            break;
        case 2:
            g.setColour(ink(cmd.c0));
            g.drawEllipse(rect, cmd.width);
            break;
        case 3:
            fillGradient(g, rect, cmd.c0, cmd.c1, false, cmd.flags != 0);
            g.fillRoundedRectangle(rect, cmd.radius);
            break;
        case 4:
            g.setColour(ink(cmd.c0));
            g.drawRoundedRectangle(rect, cmd.radius, cmd.width);
            break;
        case 5:
            fillGradient(g, rect, cmd.c0, cmd.c1, false, cmd.flags != 0);
            g.fillRect(rect);
            break;
        case 6:
            g.setColour(ink(cmd.c0));
            g.drawLine(cmd.x, cmd.y, cmd.x2, cmd.y2, cmd.width);
            break;
        case 7:
        case 8: {
            juce::Path path;
            if (cmd.npts > 0) path.startNewSubPath(cmd.pts[0], cmd.pts[1]);
            for (int i = 1; i < cmd.npts; ++i) path.lineTo(cmd.pts[i * 2], cmd.pts[i * 2 + 1]);
            if (cmd.kind == 7 || cmd.flags) path.closeSubPath();
            if (cmd.kind == 7) {
                auto bounds = path.getBounds();
                fillGradient(g, bounds, cmd.c0, cmd.c1, cmd.flags != 0, false);
                g.fillPath(path);
            } else {
                g.setColour(ink(cmd.c0));
                g.strokePath(path, juce::PathStrokeType(cmd.width));
            }
            break;
        }
        case 9: {
            juce::Path path;
            const int steps = 24;
            for (int i = 0; i <= steps; ++i) {
                const float t = (float) i / (float) steps;
                const float a = cmd.x2 + (cmd.y2 - cmd.x2) * t;
                const float px = cmd.x + std::cos(a) * cmd.radius;
                const float py = cmd.y + std::sin(a) * cmd.radius;
                if (i == 0) path.startNewSubPath(px, py);
                else path.lineTo(px, py);
            }
            g.setColour(ink(cmd.c0));
            g.strokePath(path, juce::PathStrokeType(cmd.width));
            break;
        }
        case 10: {
            g.setColour(ink(cmd.c0));
            const char* face = cmd.w >= 15.0f ? "Avenir Next" : "Avenir Next Condensed";
            g.setFont(juce::Font(juce::FontOptions(face, cmd.w, juce::Font::plain)));
            auto text = juce::String::fromUTF8((const char*) cmd.text);
            const float box = juce::jmax(cmd.w * 5.0f, 200.0f);
            if (cmd.flags)
                g.drawText(text, juce::Rectangle<float>(cmd.x - box * 0.5f, cmd.y - cmd.w * 0.65f, box, cmd.w * 1.35f),
                           juce::Justification::centred);
            else
                g.drawText(text, juce::Rectangle<float>(cmd.x, cmd.y - cmd.w, 220.0f, cmd.w * 1.45f),
                           juce::Justification::left);
            break;
        }
        case 11: {
            juce::Path clip;
            clip.addRoundedRectangle(rect, cmd.radius);
            g.saveState();
            g.reduceClipRegion(clip);
            break;
        }
        case 12:
            g.restoreState();
            break;
        default:
            break;
    }
}
}

ArcVuEditor::ArcVuEditor(ArcVuProcessor& p) : AudioProcessorEditor(p), arcvu(p) {
    // Free resizing: the window keeps the face's aspect ratio and everything scales with it.
    constrainer.setMinimumSize(380, 200);
    constrainer.setMaximumSize(4000, 4000);
    setConstrainer(&constrainer);
    setResizable(true, true);
    userScale = arcvu.zoom() > 0.0f ? arcvu.zoom() : 1.0f;
    float vw = 1100.0f, vh = 500.0f;
    arcvu_view_size(arcvu.setup.face, &vw, &vh);
    lastFace = arcvu.setup.face;
    lastSkin = arcvu.skin();
    lastExtra = arcvu.loudnessHeight(arcvu.setup.face);
    lastStep = juce::jlimit(0, 2, arcvu.setup.gui_scale);
    constrainer.setFixedAspectRatio((double) vw / (double) vh);
    setSize((int) std::lround(vw * userScale), (int) std::lround(vh * userScale));
    addChildComponent(labelBox);
    labelBox.setFont(juce::Font(juce::FontOptions(14.0f)));
    labelBox.onReturnKey = [this] { commitLabel(); };
    labelBox.onFocusLost = [this] { commitLabel(); };
    startTimerHz(30);
}

void ArcVuEditor::commitLabel() {
    const auto text = ArcVuProcessor::fitLabel(labelBox.getText());
    std::memset(arcvu.setup.label, 0, sizeof arcvu.setup.label);
    std::memcpy(arcvu.setup.label, text.toRawUTF8(), (size_t) text.getNumBytesAsUTF8());
    arcvu.noteSetup();
    labelBox.setVisible(false);
}

void ArcVuEditor::resized() {
    float vw = 1100.0f, vh = 220.0f;
    arcvu_view_size(arcvu.setup.face, &vw, &vh);
    const int top = arcvu.setup.face == 2 ? 76 : 40;
    userScale = (float) getWidth() / vw;
    zoomDirty = true;
    if (controls) {
        controls->setTransform(juce::AffineTransform::scale(getWidth() / vw));
        if (controls->getComponentID() == "6") {
            // The Tools dropdown hangs under its button and is only as large as its knobs.
            const int panelW = arcvu.setup.face == 2 ? (int) vw - 24 : 548;
            controls->setSize(panelW, 400);
            controls->setBounds(arcvu.setup.face == 2 ? 12 : (int) vw - 12 - panelW, top - (arcvu.setup.face == 2 ? 0 : 2), panelW, controls->toolsHeight());
        } else {
            controls->setBounds(12, top, (int)vw - 24, (int)vh - top - 8);
        }
    }
}

void ArcVuEditor::showPanel(int group) {
    arcvu.setup.panel = 0;
    if (controls && controls->getComponentID() == juce::String(group)) {
        controls.reset(); return;
    }
    controls = std::make_unique<ArcVuParameterPanel>(arcvu, group);
    controls->setComponentID(juce::String(group));
    controls->onClose = [safe = juce::Component::SafePointer<ArcVuEditor>(this)] {
        juce::MessageManager::callAsync([safe] { if (safe) safe->controls.reset(); });
    };
    addAndMakeVisible(*controls);
    resized();
}

void ArcVuEditor::timerCallback() {
    float vw = 1100.0f, vh = 500.0f;
    arcvu_view_size(arcvu.setup.face, &vw, &vh);
    const float scales[] = { 1.0f, 1.25f, 1.5f };
    const int step = juce::jlimit(0, 2, arcvu.setup.gui_scale);
    // The 100/125/150% choice jumps to that size; dragging the window corner sets any other size and is kept.
    if (lastStep >= 0 && step != lastStep) userScale = scales[step];
    const float extra = arcvu.loudnessHeight(arcvu.setup.face);
    if (step != lastStep || arcvu.setup.face != lastFace || arcvu.skin() != lastSkin || extra != lastExtra) {
        lastExtra = extra;
        lastSkin = arcvu.skin();
        lastStep = step;
        lastFace = arcvu.setup.face;
        const int nw = (int) std::lround(vw * userScale);
        const int nh = (int) std::lround(vh * userScale);
        constrainer.setFixedAspectRatio((double) vw / (double) vh);
        if (getWidth() != nw || getHeight() != nh) setSize(nw, nh);
    }
    if (zoomDirty && !juce::ModifierKeys::currentModifiers.isAnyMouseButtonDown()) {
        zoomDirty = false;
        arcvu.setZoom(userScale);
    }
    setAlwaysOnTop(arcvu.setup.always_on_top != 0);
    // The standalone window is the editor's parent, so only it can float above other apps; in a host the host owns the window.
    if (juce::JUCEApplicationBase::isStandaloneApp())
        if (auto* window = getTopLevelComponent(); window != nullptr && window != this)
            window->setAlwaysOnTop(arcvu.setup.always_on_top != 0);
    const int hz = juce::jlimit(10, 60, arcvu.setup.refresh_hz > 0 ? arcvu.setup.refresh_hz : 30);
    if (hz != lastHz) { lastHz = hz; startTimerHz(hz); }
    arcvu.pullSetup();
    if (controls) for (auto& row : controls->rows)
        row->slider.setMouseDragSensitivity(juce::jmax(1, (int)(250.0f / juce::jmax(0.25f, arcvu.setup.drag_sens))));
    tooltip.setEnabled(arcvu.setup.tooltips != 0);
    repaint();
}

namespace {
// Preset menu row rectangles in view coordinates. They match preset_menu() in dsp/src/lib.rs.
juce::Rectangle<float> presetRow(int row, bool compact) {
    if (compact) return { 20.0f, 78.0f + row * 21.0f, 264.0f, 20.0f };
    const int column = row / 6;
    return { 358.0f + column * 240.0f, 42.0f + (row % 6) * 21.0f, 226.0f, 20.0f };
}
}

void ArcVuEditor::paint(juce::Graphics& g) {
    float vw = 1100.0f, vh = 500.0f;
    arcvu_view_size(arcvu.setup.face, &vw, &vh);
    g.addTransform(juce::AffineTransform::scale(getWidth() / vw, getHeight() / vh));
    std::vector<ArcVuPaint> cmds(8192);
    const int n = arcvu.paint(cmds.data(), (int) cmds.size());
    for (int i = 0; i < n; ++i) replay(g, cmds[(size_t) i]);
    if (arcvu.setup.panel == 8) {
        // Live values for the rows the menu draws as labels only.
        const bool compact = arcvu.setup.face == 2;
        g.setColour(juce::Colour(0xffb7b0a4));
        g.setFont(13.0f);
        const char* category = arcvu.presetCategory() == 0 ? "Factory" : "User";
        const char* drag = arcvu.dragMode() == 0 ? "Vertical" : "Horizontal";
        g.drawText(category, presetRow(2, compact).withTrimmedRight(8), juce::Justification::centredRight, false);
        g.drawText(drag, presetRow(15, compact).withTrimmedRight(8), juce::Justification::centredRight, false);
        g.drawText(arcvu.slowMode() ? "On" : "Off", presetRow(17, compact).withTrimmedRight(8), juce::Justification::centredRight, false);
    }
}

bool ArcVuEditor::runPresetRow(int row) {
    switch (row) {
        case 0: arcvu.stepPreset(-1); break;
        case 1: arcvu.stepPreset(1); break;
        case 2: arcvu.toggleCategory(); return true;
        case 3: case 4: case 5: case 6: arcvu.applyFactory(row - 3); break;
        case 7: arcvu.applyFactory(4); break;
        case 8: arcvu.saveDefault(); break;
        case 9: arcvu.loadDefault(); break;
        case 10: arcvu.copySetup(); break;
        case 11: arcvu.pasteSetup(); break;
        case 12: choosePresetFile(true); break;
        case 13: choosePresetFile(false); break;
        case 14: showUserPresetMenu(); break;
        case 15: arcvu.toggleDragMode(); return true;
        case 16: arcvu.saveGlobalSettings(); break;
        case 17: arcvu.toggleSlowMode(); return true;
        default: break;
    }
    return false;
}

void ArcVuEditor::showUserPresetMenu() {
    const auto files = ArcVuProcessor::userPresetFiles();
    juce::PopupMenu menu;
    for (int i = 0; i < files.size(); ++i)
        menu.addItem(i + 1, files.getReference(i).getFileNameWithoutExtension());
    if (files.isEmpty()) menu.addItem(1000, "No user presets yet", false);
    menu.showMenuAsync(juce::PopupMenu::Options().withTargetComponent(this), [this, files](int result) {
        if (result > 0 && result <= files.size()) {
            arcvu.loadPresetFile(files.getReference(result - 1));
            repaint();
        }
    });
}

void ArcVuEditor::choosePresetFile(bool save) {
    auto dir = ArcVuProcessor::userPresetDirectory();
    dir.createDirectory();
    presetChooser = std::make_unique<juce::FileChooser>(save ? "Save ArcVu preset" : "Load ArcVu preset", dir, "*.arcvu");
    const int flags = save ? juce::FileBrowserComponent::saveMode | juce::FileBrowserComponent::warnAboutOverwriting
                           : juce::FileBrowserComponent::openMode | juce::FileBrowserComponent::canSelectFiles;
    presetChooser->launchAsync(flags, [this, save](const juce::FileChooser& chooser) {
        const auto file = chooser.getResult();
        if (file == juce::File()) return;
        if (save) arcvu.savePresetFile(file.withFileExtension(".arcvu"));
        else arcvu.loadPresetFile(file);
        repaint();
    });
}

void ArcVuEditor::mouseDown(const juce::MouseEvent& e) {
    auto& s = arcvu.setup;
    dragTrim = -1;
    float vw = 1100.0f, vh = 500.0f;
    arcvu_view_size(s.face, &vw, &vh);
    auto p = e.position;
    p.x *= vw / (float) getWidth();
    p.y *= vh / (float) getHeight();
    const bool compact = s.face == 2;
    // The hardware row sits at the bottom of the face; the loudness panel, when on, is under it.
    const float faceH = vh - arcvu.loudnessHeight(s.face);
    const float row = faceH - (compact ? 100.0f : 64.0f);
    if (p.y >= faceH && arcvu.loudnessHeight(s.face) > 0.0f) {
        arcvu.loudnessClick(p.x, p.y);
        return;
    }
    if (compact) {
        int group = 0;
        if (p.y >= 5 && p.y <= 31) {
            group = p.x >= 88 && p.x < 156 ? 1 : p.x >= 162 && p.x < 254 ? 2 : p.x >= 260 && p.x < 308 ? 3 : p.x >= 314 && p.x < 374 ? 7 : 0;
        } else if (p.y >= 41 && p.y <= 67) {
            group = p.x >= 12 && p.x < 102 ? 8 : p.x >= 114 && p.x < 194 ? 4 : p.x >= 206 && p.x < 266 ? 5 : p.x >= 278 && p.x < 358 ? 6 : 0;
        }
        if (group == 8) { controls.reset(); s.panel = s.panel == 8 ? 0 : 8; return; }
        if (group) { showPanel(group); return; }
        if (p.y >= row + 32 && p.y < row + 60 && !s.meters_only) {
            if (p.x >= 8 && p.x < 56) s.mid_side = 0;
            else if (p.x >= 60 && p.x < 108) s.mid_side = 1;
            else if (p.x >= 112 && p.x < 226) { const int i = (int)((p.x - 112) / 38); s.reference_db = i == 0 ? -20 : i == 1 ? -18 : -14; }
            arcvu.noteSetup(); return;
        }
        if (s.panel == 8 && p.x >= 20 && p.x < 284 && p.y >= 78 && p.y < 78 + 18 * 21) {
            if (!runPresetRow((int)((p.y - 78) / 21))) s.panel = 0;
            return;
        }
    }
    if (p.y >= row && p.y < row + 30.0f && s.meters_only == 0) {
        if (p.x < 68.0f) s.peak_mode = s.peak_mode == 1 ? 0 : 1;
        else if (p.x < 128.0f) s.peak_mode = s.peak_mode == 2 ? 0 : 2;
        else if (p.x < 188.0f) arcvu.resetPeaks();
        else if (p.x >= 248.0f && p.x < 292.0f) arcvu.resetOvers();
        else if (!compact && p.x >= vw - 70.0f) { showPanel(7); return; }
        else if (p.x >= 296.0f && p.x < 332.0f) s.pol_l ^= 1;
        else if (p.x >= 336.0f && p.x < 372.0f) s.pol_r ^= 1;
        else if (p.x >= 376.0f && p.x < 424.0f) s.mid_side = 0;
        else if (p.x >= 428.0f && p.x < 476.0f) s.mid_side = 1;
        else if (p.x >= 480.0f && p.x < 594.0f) {
            const int index = juce::jlimit(0, 2, (int) ((p.x - 480.0f) / 38.0f));
            s.reference_db = index == 0 ? -20.0f : index == 2 ? -14.0f : -18.0f;
        }
        arcvu.noteSetup();
        return;
    }
    if (p.x < 220.0f && p.y >= row + (compact ? 66.0f : 32.0f) && p.y < row + (compact ? 90.0f : 52.0f) && s.meters_only == 0) {
        const float scale = getWidth() / vw;
        labelBox.setBounds((int)(16 * scale), (int) ((row + (compact ? 66.0f : 32.0f)) * scale), (int)(180 * scale), (int)(22 * scale));
        labelBox.setText(juce::String((const char*) s.label), false);
        labelBox.setVisible(true);
        labelBox.grabKeyboardFocus();
        return;
    }
    const int bar = !compact && p.y >= 5.0f && p.y <= 31.0f
        ? (p.x >= 90.0f && p.x < 170.0f ? 1
        : p.x >= 178.0f && p.x < 278.0f ? 2
        : p.x >= 286.0f && p.x < 342.0f ? 3
        : p.x >= 350.0f && p.x < 428.0f ? 8
        : p.x >= 436.0f && p.x < 506.0f ? 4
        : p.x >= 514.0f && p.x < 566.0f ? 5
        : p.x >= 574.0f && p.x < 644.0f ? 6 : 0) : 0;
    if (bar != 0 && bar != 8) { showPanel(bar); return; }
    if (bar == 8) { controls.reset(); s.panel = s.panel == 8 ? 0 : 8; return; }
    if (!compact && p.y >= 5.0f && p.y <= 31.0f && p.x >= 652.0f && p.x < 720.0f) { showPanel(7); return; }

    if (s.panel == 8 && !compact && p.x >= 358.0f && p.x < 1078.0f && p.y >= 42.0f && p.y < 42.0f + 6 * 21.0f) {
        const int column = (int) ((p.x - 358.0f) / 240.0f);
        const bool onRow = p.x - 358.0f - column * 240.0f <= 226.0f;
        const int index = column * 6 + (int) ((p.y - 42.0f) / 21.0f);
        if (onRow && index < 18 && !runPresetRow(index)) s.panel = 0;
        return;
    } else if (s.panel == 0 && p.y >= (compact ? 80.0f : 40.0f) && p.y < row - 6.0f) {
        arcvu.resetPeaks();
        arcvu.resetClip();
    } else if (s.panel != 0) s.panel = 0;
    arcvu.noteSetup();
}

// Clicks change state on mouse down; repaint on mouse up so the display follows them even when the transport is stopped.
void ArcVuEditor::mouseUp(const juce::MouseEvent&) {
    repaint();
}

void ArcVuEditor::mouseDrag(const juce::MouseEvent& e) {
    if (dragTrim < 0) return;
    auto& s = arcvu.setup;
    // Drag mode 0 moves up to increase; mode 1 moves right to increase.
    const float travel = arcvu.dragMode() == 1 ? e.getDistanceFromDragStartX() : -e.getDistanceFromDragStartY();
    const float dy = travel * juce::jmax(0.25f, s.drag_sens);
    if (dragTrim < 4) {
        (&s.trim_l)[dragTrim] = juce::jlimit(-24.0f, 24.0f, dragOrigin + dy * 0.05f);
    } else if (dragTrim == 10) s.hpf_hz = juce::jlimit(20.0f, 500.0f, dragOrigin + dy * 0.8f);
    else if (dragTrim == 11) s.lpf_hz = juce::jlimit(1000.0f, 20000.0f, dragOrigin + dy * 12.0f);
    else if (dragTrim == 12) s.mono_hz = juce::jlimit(20.0f, 400.0f, dragOrigin + dy * 0.6f);
    else if (dragTrim == 13) s.mono_amt = juce::jlimit(0.0f, 1.0f, dragOrigin + dy * 0.004f);
    else if (dragTrim == 15) s.dyn_depth = juce::jlimit(0.0f, 1.0f, dragOrigin + dy * 0.004f);
    else if (dragTrim == 16) s.dyn_hz = juce::jlimit(80.0f, 12000.0f, dragOrigin + dy * 20.0f);
    arcvu.noteSetup();
}
