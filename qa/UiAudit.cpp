#include "PluginEditor.h"
#include <iostream>
#include <set>
#include <cstring>
#include <cstdlib>
#include <atomic>
#include <chrono>
#include <new>
#include <thread>
#include <vector>

namespace {
int failures = 0;
void check(bool ok, const juce::String& text) {
    std::cout << (ok ? "PASS " : "FAIL ") << text << std::endl;
    if (!ok) ++failures;
}
void pump() { juce::MessageManager::getInstance()->runDispatchLoopUntil(60); juce::Timer::callPendingTimersSynchronously(); }
void process(ArcVuProcessor& p) {
    juce::AudioBuffer<float> b(2, 512); juce::MidiBuffer midi;
    for (int k = 0; k < 96; ++k) {
        for (int i = 0; i < 512; ++i) {
            b.setSample(0, i, 0.126f * std::sin(juce::MathConstants<float>::twoPi * 1000.0f * (float)(k * 512 + i) / 48000.0f));
            b.setSample(1, i, 0.08f * std::sin(juce::MathConstants<float>::twoPi * 350.0f * (float)(k * 512 + i) / 48000.0f));
        }
        p.processBlock(b, midi);
    }
}
void snapshot(juce::Component& c, const juce::File& file) {
    juce::Image image(juce::Image::ARGB, c.getWidth(), c.getHeight(), true);
    juce::Graphics g(image); c.paintEntireComponent(g, true);
    juce::FileOutputStream stream(file); juce::PNGImageFormat png;
    check(stream.openedOk() && png.writeImageToStream(image, stream), file.getFileName());
}
juce::RangedAudioParameter* parameter(ArcVuProcessor& p, const char* id) {
    for (auto* ap : p.getParameters())
        if (auto* rp = dynamic_cast<juce::RangedAudioParameter*>(ap); rp && rp->paramID == id) return rp;
    return nullptr;
}
}
// Heap allocations made on this thread while counting is on. Used to prove processBlock does not allocate.
static thread_local bool g_countAllocs = false;
static thread_local long g_allocCount = 0;
void* operator new(std::size_t size) {
    if (g_countAllocs) ++g_allocCount;
    if (void* p = std::malloc(size ? size : 1)) return p;
    throw std::bad_alloc();
}
void operator delete(void* p) noexcept { std::free(p); }
void operator delete(void* p, std::size_t) noexcept { std::free(p); }

int main(int argc, char** argv) {
    if (argc > 2 && juce::String(argv[1]) == "--manual-shots") {
        // Two-times screenshots for the user manual. Scratch settings files keep the user's own untouched.
        const juce::File dir = juce::File::getSpecialLocation(juce::File::tempDirectory).getChildFile("arcvu-manual-scratch");
        dir.createDirectory();
        setenv("ARCVU_QA_DEFAULT_FILE", dir.getChildFile("default.arcvu").getFullPathName().toRawUTF8(), 1);
        setenv("ARCVU_QA_GLOBAL_FILE", dir.getChildFile("global.txt").getFullPathName().toRawUTF8(), 1);
        const juce::File out(argv[2]); out.createDirectory();
        ArcVuProcessor p; p.prepareToPlay(48000, 512);
        std::unique_ptr<ArcVuEditor> e(static_cast<ArcVuEditor*>(p.createEditor()));
        auto shoot = [&](const char* name, int face, int skin, int theme, int kind, int, bool single) {
            p.setSkin(skin); p.setTheme(theme);
            parameter(p, "kind")->setValueNotifyingHost(parameter(p, "kind")->convertTo0to1((float) kind));
            parameter(p, "face")->setValueNotifyingHost(parameter(p, "face")->convertTo0to1((float) face));
            parameter(p, "single")->setValueNotifyingHost(single ? 1.0f : 0.0f);
            p.setup.panel = 0;
            process(p); pump();
            float vw = 1100.0f, vh = 500.0f;
            arcvu_view_size(face, &vw, &vh);
            e->setSize((int) (vw * 2), (int) (vh * 2));
            process(p); pump();
            pump();
            snapshot(*e, out.getChildFile(juce::String(name) + ".png"));
            p.setup.panel = 0; p.noteSetup(); pump();
        };
        shoot("arcvu-modern-arc", 0, 0, 0, 0, 0, false);
        shoot("arcvu-classic-arc", 0, 1, 0, 0, 0, false);
        shoot("arcvu-classic-amber", 0, 1, 1, 0, 0, false);
        shoot("arcvu-classic-ice", 0, 1, 2, 0, 0, false);
        shoot("arcvu-modern-horizontal", 1, 0, 0, 0, 0, false);
        p.setLoudness(ArcVuLoudSettings { 1, -14.0f, -1.0f, 0, 0, 10.0f });
        shoot("arcvu-loudness", 1, 0, 0, 0, 0, false);
        shoot("arcvu-loudness-classic", 0, 1, 0, 0, 0, false);
        shoot("arcvu-loudness-vertical", 2, 0, 0, 0, 0, false);
        shoot("arcvu-loudness-ice", 1, 0, 2, 0, 0, false);
        p.setLoudness(ArcVuLoudSettings { 0, -14.0f, -1.0f, 0, 0, 10.0f });
        shoot("arcvu-modern-vertical", 2, 0, 0, 0, 0, false);
        shoot("arcvu-classic-vertical", 2, 1, 0, 0, 0, false);
        shoot("arcvu-single", 0, 0, 0, 0, 0, true);
        shoot("arcvu-k14", 0, 1, 0, 3, 0, false);
        shoot("arcvu-ppm-ebu", 0, 1, 2, 8, 0, false);
        return failures ? 1 : 0;
    }
    if (argc > 2 && juce::String(argv[1]) == "--icon-render") {
        // The Vertical face at high resolution, for the application icon. Scratch settings files keep the user's own untouched.
        const juce::File dir = juce::File::getSpecialLocation(juce::File::tempDirectory).getChildFile("arcvu-icon-scratch");
        dir.createDirectory();
        setenv("ARCVU_QA_DEFAULT_FILE", dir.getChildFile("default.arcvu").getFullPathName().toRawUTF8(), 1);
        setenv("ARCVU_QA_GLOBAL_FILE", dir.getChildFile("global.txt").getFullPathName().toRawUTF8(), 1);
        ArcVuProcessor p; p.prepareToPlay(48000, 512);
        parameter(p, "face")->setValueNotifyingHost(parameter(p, "face")->convertTo0to1(2.0f));
        parameter(p, "showValues")->setValueNotifyingHost(0.0f);
        parameter(p, "showHold")->setValueNotifyingHost(0.0f);
        std::unique_ptr<ArcVuEditor> e(static_cast<ArcVuEditor*>(p.createEditor()));
        float vw = 380.0f, vh = 700.0f;
        arcvu_view_size(2, &vw, &vh);
        process(p); pump();
        e->setSize((int) (vw * 4), (int) (vh * 4));
        pump();
        snapshot(*e, juce::File(argv[2]));
        return failures ? 1 : 0;
    }
    if (argc > 1 && juce::String(argv[1]) == "--keys-test") {
        // S26: overs count and Clear, and phase R inverts the right channel only.
        ArcVuProcessor p; p.prepareToPlay(48000, 512);
        juce::MidiBuffer midi;
        juce::AudioBuffer<float> over(2, 8);
        over.clear();
        for (int i = 0; i < 8; ++i) { over.setSample(0, i, 2.0f); over.setSample(1, i, 2.0f); }
        for (int k = 0; k < 4; ++k) p.processBlock(over, midi);
        check(p.overs() >= 1, "overs count rises on over-full-scale samples (" + juce::String(p.overs()) + ")");
        p.resetOvers();
        check(p.overs() == 0, "Clr resets the overs count");

        parameter(p, "polR")->setValueNotifyingHost(1.0f);
        juce::AudioBuffer<float> tone(2, 4);
        for (int i = 0; i < 4; ++i) { tone.setSample(0, i, 0.25f); tone.setSample(1, i, 0.25f); }
        p.processBlock(tone, midi);
        check(std::abs(tone.getSample(0, 0) - 0.25f) < 1e-4f, "phase R leaves the left channel alone");
        check(std::abs(tone.getSample(1, 0) + 0.25f) < 1e-4f, "phase R inverts the right channel");
        std::cout << "FAILURES " << failures << std::endl;
        return failures ? 1 : 0;
    }
    if (argc > 2 && juce::String(argv[1]) == "--state-file-dump") {
        // Loads a raw state file the way the standalone does and prints the fields it produces.
        juce::MemoryBlock block;
        juce::File(argv[2]).loadFileAsData(block);
        std::cout << "file bytes " << block.getSize() << " sizeof(setup) " << sizeof(ArcVuSetup) << std::endl;
        ArcVuProcessor p;
        p.setStateInformation(block.getData(), (int) block.getSize());
        std::cout << "after set: kind " << p.setup.kind << " face " << p.setup.face << " single " << p.setup.single
                  << " mid_side " << p.setup.mid_side << " ref " << p.setup.reference_db << " meters_only " << p.setup.meters_only << std::endl;
        p.pullSetup();
        std::cout << "after pull: kind " << p.setup.kind << " face " << p.setup.face << " single " << p.setup.single
                  << " mid_side " << p.setup.mid_side << " ref " << p.setup.reference_db << " meters_only " << p.setup.meters_only << std::endl;
        return 0;
    }
    if (argc > 1 && juce::String(argv[1]) == "--state-race-test") {
        // Loading state on the message thread while the audio thread runs must give exactly the loaded values.
        ArcVuProcessor p; p.prepareToPlay(48000, 512);
        juce::MemoryBlock block;
        {
            ArcVuProcessor src;
            src.setup.kind = 6; src.setup.face = 2; src.setup.single = 0; src.setup.meters_only = 0;
            src.setup.reference_db = -14.0f; src.setup.trim_l = 5.0f; src.setup.mid_side = 1;
            src.noteSetup();
            src.getStateInformation(block);
        }
        std::atomic<bool> stop { false };
        std::thread audio([&] { juce::AudioBuffer<float> b(2, 128); juce::MidiBuffer m; b.clear(); while (!stop) p.processBlock(b, m); });
        int bad = 0;
        for (int k = 0; k < 300; ++k) {
            // Start from a different state each time so a stale overwrite is visible.
            p.setup.kind = 0; p.setup.face = 0; p.setup.mid_side = 0; p.setup.reference_db = -18.0f; p.setup.trim_l = 0.0f; p.noteSetup();
            p.setStateInformation(block.getData(), (int) block.getSize());
            std::this_thread::sleep_for(std::chrono::microseconds(300));
            p.pullSetup();
            if (p.setup.kind != 6 || p.setup.face != 2 || p.setup.mid_side != 1 || std::abs(p.setup.reference_db + 14.0f) > 0.01f || std::abs(p.setup.trim_l - 5.0f) > 0.01f) ++bad;
        }
        stop = true; audio.join();
        check(bad == 0, "state load with audio running gave the wrong values " + juce::String(bad) + " of 300 times");
        std::cout << "FAILURES " << failures << std::endl;
        return failures ? 1 : 0;
    }
    if (argc > 1 && juce::String(argv[1]) == "--bad-state-test") {
        // A wrong file (here PNG-like bytes) must not change the state.
        ArcVuProcessor p;
        p.setup.kind = 3; p.setup.face = 1; p.noteSetup();
        unsigned char junk[400];
        for (int i = 0; i < 400; ++i) junk[i] = (unsigned char) (0x89 + i * 37);
        p.setStateInformation(junk, 400);
        p.pullSetup();
        check(p.setup.kind == 3 && p.setup.face == 1, "junk bytes leave the state unchanged");
        juce::MemoryBlock good;
        p.getStateInformation(good);
        p.setup.kind = 0; p.noteSetup();
        p.setStateInformation(good.getData(), (int) good.getSize());
        p.pullSetup();
        check(p.setup.kind == 3, "a real saved state still loads");
        std::cout << "FAILURES " << failures << std::endl;
        return failures ? 1 : 0;
    }
    if (argc > 2 && juce::String(argv[1]) == "--overs-snapshot") {
        // L6: the overs digits and the phase readout, idle and with overs, on the arc face.
        const juce::File out(argv[2]);
        out.createDirectory();
        ArcVuProcessor p; p.prepareToPlay(48000, 512);
        std::unique_ptr<ArcVuEditor> e(static_cast<ArcVuEditor*>(p.createEditor()));
        float vw = 1100.0f, vh = 500.0f; arcvu_view_size(0, &vw, &vh); e->setSize((int) vw, (int) vh);
        process(p); pump();
        snapshot(*e, out.getChildFile("overs-idle.png"));
        juce::MidiBuffer midi; juce::AudioBuffer<float> over(2, 512);
        for (int k = 0; k < 40; ++k) { for (int i = 0; i < 512; ++i) { over.setSample(0, i, 2.0f); over.setSample(1, i, 2.0f); } p.processBlock(over, midi); }
        pump();
        snapshot(*e, out.getChildFile("overs-active.png"));
        std::cout << "overs " << p.overs() << std::endl;
        return 0;
    }
    if (argc > 1 && juce::String(argv[1]) == "--realtime-test") {
        // B1: processBlock must not allocate, and must keep running while the editor-side calls run on another thread.
        ArcVuProcessor p; p.prepareToPlay(48000, 512);
        juce::AudioBuffer<float> buffer(2, 512); juce::MidiBuffer midi;
        for (int k = 0; k < 8; ++k) p.processBlock(buffer, midi);
        g_allocCount = 0; g_countAllocs = true;
        for (int k = 0; k < 200; ++k) p.processBlock(buffer, midi);
        g_countAllocs = false;
        check(g_allocCount == 0, "processBlock allocates: " + juce::String((int) g_allocCount) + " allocations in 200 blocks");
        std::atomic<bool> finished { false };
        std::thread audio([&] { juce::AudioBuffer<float> b(2, 512); juce::MidiBuffer m; for (int k = 0; k < 4000; ++k) p.processBlock(b, m); finished = true; });
        juce::MemoryBlock state;
        p.getStateInformation(state);
        std::vector<ArcVuPaint> cmds(8192);
        for (int k = 0; k < 400; ++k) {
            p.setStateInformation(state.getData(), (int) state.getSize());
            p.getStateInformation(state);
            p.paint(cmds.data(), (int) cmds.size());
        }
        audio.join();
        check(finished.load(), "audio thread completed 4000 blocks while editor-side calls ran");
        std::cout << "FAILURES " << failures << std::endl;
        return failures ? 1 : 0;
    }

    juce::ScopedJuceInitialiser_GUI gui;
    // Default to scratch settings files so audits never read or overwrite the user's own ArcVu settings.
    setenv("ARCVU_QA_DEFAULT_FILE", "/private/tmp/arcvu-qa-audit-default.arcvu", 0);
    setenv("ARCVU_QA_GLOBAL_FILE", "/private/tmp/arcvu-qa-audit-global.txt", 0);
    juce::File("/private/tmp/arcvu-qa-audit-default.arcvu").deleteFile();
    juce::File("/private/tmp/arcvu-qa-audit-global.txt").deleteFile();
    if (argc > 2 && juce::String(argv[1]) == "--categories-global-test") {
        // Everything lives in a scratch folder so the user's real ArcVu settings are never read or written.
        const juce::File dir(argv[2]);
        dir.deleteRecursively();
        dir.createDirectory();
        setenv("ARCVU_QA_DEFAULT_FILE", dir.getChildFile("default.arcvu").getFullPathName().toRawUTF8(), 1);
        setenv("ARCVU_QA_GLOBAL_FILE", dir.getChildFile("global.txt").getFullPathName().toRawUTF8(), 1);
        {
            ArcVuProcessor writer;
            writer.setup.kind = 6;
            writer.noteSetup();
            writer.savePresetFile(ArcVuProcessor::userPresetDirectory().getChildFile("b-second.arcvu"));
            writer.setup.kind = 3;
            writer.noteSetup();
            writer.savePresetFile(ArcVuProcessor::userPresetDirectory().getChildFile("a-first.arcvu"));
        }
        {
            ArcVuProcessor proc;
            check(ArcVuProcessor::userPresetFiles().size() == 2, "user preset browser lists two files");
            check(proc.presetCategory() == 0, "starts in the Factory category");
            proc.toggleCategory();
            check(proc.presetCategory() == 1, "category switches to User");
            proc.stepPreset(1);
            check(proc.setup.kind == 3, "next in User loads the first file by name");
            proc.stepPreset(1);
            check(proc.setup.kind == 6, "next in User loads the second file");
            proc.stepPreset(1);
            check(proc.setup.kind == 3, "next in User wraps to the first file");
            proc.toggleCategory();
            proc.stepPreset(1);
            check(proc.setup.reference_db == -18.0f, "back in Factory, next loads factory preset 0");
        }
        {
            ArcVuProcessor first;
            check(first.dragMode() == 0, "drag mode defaults to vertical");
            first.toggleDragMode();
            first.setup.drag_sens = 2.5f;
            first.setup.refresh_hz = 15;
            first.setup.tooltips = 0;
            first.setSkin(1);
            first.setTheme(2);
            first.setPeakScale(1);
            first.setLoudness(ArcVuLoudSettings { 1, -16.0f, -2.0f, 1, 1, 15.0f });
            first.noteSetup();
            first.saveGlobalSettings();
            first.setZoom(1.75f);
        }
        {
            ArcVuProcessor second;
            check(second.dragMode() == 1, "global settings restore horizontal drag mode");
            check(second.skin() == 1, "global settings restore the Classic skin");
            check(second.theme() == 2, "global settings restore the colour theme");
            check(second.peakScale() == 1, "global settings restore the dBFS peak scale");
            const auto& ls = second.loudness();
            check(ls.on == 1 && ls.scale == 1 && ls.unit == 1 && std::abs(ls.target + 16.0f) < 0.01f && std::abs(ls.ceiling + 2.0f) < 0.01f && std::abs(ls.window - 15.0f) < 0.01f,
                  "global settings restore the loudness panel settings");
            second.setLoudness(ArcVuLoudSettings { 0, -14.0f, -1.0f, 0, 0, 10.0f });
            check(std::abs(second.zoom() - 1.75f) < 0.001f, "global settings restore the window zoom");
            check(std::abs(second.setup.drag_sens - 2.5f) < 0.001f, "global settings restore drag sensitivity");
            check(second.setup.refresh_hz == 15, "global settings restore refresh rate");
            check(second.setup.tooltips == 0, "global settings restore tooltips");
            second.applyFactory(0);
            check(second.dragMode() == 1, "loading a factory preset leaves global drag mode alone");
        }
        {
            // Changing the zoom (or the look) must not rewrite the other global preferences with this session's values.
            ArcVuProcessor other;
            other.setup.drag_sens = 0.5f;
            other.setup.refresh_hz = 20;
            other.noteSetup();
            other.setZoom(1.5f);
            ArcVuProcessor::saveGlobalKey("skin", "0");
            ArcVuProcessor probe;
            check(std::abs(probe.setup.drag_sens - 2.5f) < 0.001f, "changing the zoom keeps the saved drag sensitivity");
            check(probe.setup.refresh_hz == 15, "changing the zoom keeps the saved refresh rate");
            check(std::abs(probe.zoom() - 1.5f) < 0.001f, "the new zoom is stored");
            check(probe.skin() == 0, "a single key change is stored");
        }
        {
            // Slow mode is a non-saved switch in the Preset menu: it toggles the meter and never changes saved state.
            ArcVuProcessor slow;
            check(slow.slowMode() == 0, "slow mode starts off");
            slow.toggleSlowMode();
            check(slow.slowMode() == 1, "slow mode row switches on");
            slow.toggleSlowMode();
            check(slow.slowMode() == 0, "slow mode row switches off");
        }
        dir.deleteRecursively();
        std::cout << "FAILURES " << failures << std::endl;
        return failures ? 1 : 0;
    }
    if (argc > 2 && juce::String(argv[1]) == "--default-load-test") {
        const juce::File file(argv[2]);
        file.deleteFile();
        setenv("ARCVU_QA_DEFAULT_FILE", file.getFullPathName().toRawUTF8(), 1);
        {
            ArcVuProcessor saved;
            saved.setup.kind = 5;
            saved.setup.trim_l = 7.0f;
            std::strncpy(reinterpret_cast<char*>(saved.setup.label), "Saved default", sizeof saved.setup.label - 1);
            saved.noteSetup();
            saved.saveDefault();
        }
        {
            ArcVuProcessor loaded;
            check(loaded.setup.kind == 5, "new instance loads saved meter type");
            check(std::abs(loaded.setup.trim_l - 7.0f) < 0.001f, "new instance loads saved trim");
            check(std::strcmp(reinterpret_cast<const char*>(loaded.setup.label), "Saved default") == 0,
                  "new instance loads saved label");
        }
        file.deleteFile();
        unsetenv("ARCVU_QA_DEFAULT_FILE");
        std::cout << "FAILURES " << failures << std::endl;
        return failures ? 1 : 0;
    }
    if (argc > 2 && juce::String(argv[1]) == "--preset-file-test") {
        const juce::File file(argv[2]);
        file.deleteFile();
        {
            ArcVuProcessor proc;
            proc.setup.kind = 6;
            proc.setup.trim_r = -4.5f;
            proc.setup.dyn_on = 1;
            proc.setup.dyn_hz = 1234.0f;
            std::strncpy(reinterpret_cast<char*>(proc.setup.label), "Preset test", sizeof proc.setup.label - 1);
            proc.noteSetup();
            proc.savePresetFile(file);
        }
        {
            ArcVuProcessor loaded;
            loaded.applyFactory(0);
            check(loaded.loadPresetFile(file), "preset file loads");
            check(loaded.setup.kind == 6, "preset file restores meter type");
            check(std::abs(loaded.setup.trim_r + 4.5f) < 0.001f, "preset file restores trim R");
            check(std::abs(loaded.setup.dyn_hz - 1234.0f) < 0.001f, "preset file restores dynamic frequency");
            check(std::strcmp(reinterpret_cast<const char*>(loaded.setup.label), "Preset test") == 0, "preset file restores label");
        }
        {
            ArcVuProcessor stepper;
            stepper.stepPreset(-1);
            check(stepper.setup.reference_db == -14.0f, "previous from first wraps to last factory preset");
            stepper.stepPreset(1);
            check(stepper.setup.reference_db == -18.0f && stepper.setup.aes17 == 1, "next returns to factory preset 0");
            stepper.stepPreset(1);
            check(stepper.setup.kind == 3, "next moves to factory preset 1");
        }
        {
            juce::File bad = file.getSiblingFile("arcvu-short.arcvu");
            bad.replaceWithText("short");
            ArcVuProcessor rejected;
            check(!rejected.loadPresetFile(bad), "short preset file is rejected");
            bad.deleteFile();
        }
        {
            // S19: labels hold at most 31 UTF-8 bytes and never end inside a character.
            juce::String twoByte;
            for (int i = 0; i < 20; ++i) twoByte << juce::String(juce::CharPointer_UTF8("\xc3\xa9"));
            const auto fitted = ArcVuProcessor::fitLabel(twoByte);
            check(fitted.getNumBytesAsUTF8() <= 31, "label capped at 31 UTF-8 bytes");
            check(fitted.getNumBytesAsUTF8() == 30, "label drops whole characters, not bytes");
            check(ArcVuProcessor::fitLabel("Stereo bus").length() == 10, "short label is unchanged");
        }
        {
            // S22a: a factory preset changes only its own parameters; reset restores the instance defaults.
            ArcVuProcessor p;
            const float defaultHpf = p.setup.hpf_hz;
            p.setup.trim_l = 3.0f;
            p.setup.hpf_hz = 300.0f;
            std::strncpy(reinterpret_cast<char*>(p.setup.label), "Keep me", sizeof p.setup.label - 1);
            p.noteSetup();
            p.applyFactory(0);
            check(std::abs(p.setup.trim_l - 3.0f) < 0.001f, "factory preset keeps trim L");
            check(std::abs(p.setup.hpf_hz - 300.0f) < 0.001f, "factory preset keeps high-pass frequency");
            check(std::strcmp(reinterpret_cast<const char*>(p.setup.label), "Keep me") == 0, "factory preset keeps label");
            p.applyFactory(4);
            check(std::abs(p.setup.trim_l) < 0.001f, "reset restores trim L");
            check(std::abs(p.setup.hpf_hz - defaultHpf) < 0.001f, "reset restores high-pass frequency");
        }
        {
            // S21a: Copy puts the full setup on the clipboard and Paste restores it into any instance.
            ArcVuProcessor source;
            source.setup.kind = 6;
            source.setup.trim_r = -4.5f;
            std::strncpy(reinterpret_cast<char*>(source.setup.label), "Clip me", sizeof source.setup.label - 1);
            source.noteSetup();
            source.copySetup();
            ArcVuProcessor target;
            target.pasteSetup();
            check(target.setup.kind == 6, "paste restores meter type from clipboard");
            check(std::abs(target.setup.trim_r + 4.5f) < 0.001f, "paste restores trim R from clipboard");
            check(std::strcmp(reinterpret_cast<const char*>(target.setup.label), "Clip me") == 0, "paste restores label from clipboard");
        }
        file.deleteFile();
        std::cout << "FAILURES " << failures << std::endl;
        return failures ? 1 : 0;
    }
    if (argc > 1 && juce::String(argv[1]) == "--detector-audit") {
        auto testWeight = [](float weight) {
            ArcVuProcessor proc; proc.prepareToPlay(48000, 512);
            auto* wp = parameter(proc, "weight"); wp->setValueNotifyingHost(wp->convertTo0to1(weight));
            juce::AudioBuffer<float> b(2, 512); juce::MidiBuffer midi;
            for (int k = 0; k < 192; ++k) {
                b.clear();
                for (int i = 0; i < 512; ++i) b.setSample(0, i, 0.2f * std::sin(juce::MathConstants<float>::twoPi * 1000 * (float)(k*512+i)/48000));
                proc.processBlock(b, midi);
            }
            return proc.levels(1).vu_db;
        };
        const auto unweighted = testWeight(0), weighted = testWeight(1);
        std::cout << "F1 right silence, unweighted=" << unweighted << " weighted=" << weighted << " dB" << std::endl;
        check(unweighted < -80 && weighted < -80, "weighting isolates stereo channels");
        ArcVuProcessor kp; kp.prepareToPlay(48000,512);
        parameter(kp, "kind")->setValueNotifyingHost(parameter(kp, "kind")->convertTo0to1(3));
        // process() fixture left channel is a -18 dBFS sine; K-14 must read -4 dB.
        process(kp);
        std::cout << "F2 K14 analog reading=" << kp.levels(0).vu_db << " expected=-4" << std::endl;
        check(std::abs(kp.levels(0).vu_db + 4.0f) < 0.25f, "analog K14 scale calibration");
        auto dynRms = [](float hz) {
            ArcVuProcessor proc; proc.prepareToPlay(48000,512);
            for (auto item : {std::pair<const char*,float>{"dynOn",1}, {"dynType",1}, {"dynHz",1000}, {"dynDepth",0.5f}, {"dynListen",1}}) {
                auto* rp = parameter(proc,item.first); rp->setValueNotifyingHost(rp->convertTo0to1(item.second));
            }
            juce::AudioBuffer<float> b(2,512); juce::MidiBuffer midi; double sum=0;
            for (int k=0;k<192;++k) {
                for (int i=0;i<512;++i) for(int c=0;c<2;++c) b.setSample(c,i,0.2f*std::sin(juce::MathConstants<float>::twoPi*hz*(float)(k*512+i)/48000));
                proc.processBlock(b,midi);
                if(k>=96) for(int i=0;i<512;++i) sum+=b.getSample(0,i)*b.getSample(0,i);
            }
            return std::sqrt(sum/(96*512));
        };
        auto low=dynRms(50), high=dynRms(8000);
        std::cout << "F3 dynamic HPF listen RMS low=" << low << " high=" << high << std::endl;
        check(high > low*10, "dynamic HPF listen passes highs");
        std::cout << "FAILURES " << failures << std::endl;
        return failures ? 1 : 0;
    }
    const juce::File output(argc > 1 ? argv[1] : "/private/tmp/arcvu-ui-audit"); output.createDirectory();
    ArcVuProcessor p; p.prepareToPlay(48000, 512);
    std::unique_ptr<ArcVuEditor> e(static_cast<ArcVuEditor*>(p.createEditor()));
    std::set<juce::String> covered;
    juce::Array<float> defaults;
    for (auto* ap : p.getParameters()) defaults.add(ap->getValue());
    // S1d: every meter type on every face, for readability review.
    for (int kind = 0; kind < 9; ++kind) {
        for (int face = 0; face < 3; ++face) {
            parameter(p, "kind")->setValueNotifyingHost(parameter(p, "kind")->convertTo0to1((float)kind));
            parameter(p, "face")->setValueNotifyingHost(parameter(p, "face")->convertTo0to1((float)face));
            float vw = 1100.0f, vh = 500.0f;
            arcvu_view_size(face, &vw, &vh);
            e->setSize((int) vw, (int) vh);
            process(p); pump();
            snapshot(*e, output.getChildFile("matrix-kind-" + juce::String(kind) + "-face-" + juce::String(face) + ".png"));
        }
    }
    // Colour themes on every face (UI-only setting, not part of the saved state).
    for (int theme = 0; theme < 3; ++theme) {
        p.setTheme(theme);
        for (int face = 0; face < 3; ++face) {
            parameter(p, "kind")->setValueNotifyingHost(0);
            parameter(p, "face")->setValueNotifyingHost(parameter(p, "face")->convertTo0to1((float)face));
            float vw = 1100.0f, vh = 500.0f;
            arcvu_view_size(face, &vw, &vh);
            e->setSize((int) vw, (int) vh);
            process(p); pump();
            snapshot(*e, output.getChildFile("theme-" + juce::String(theme) + "-face-" + juce::String(face) + ".png"));
        }
    }
    // Classic needle skin on every face and colour.
    p.setSkin(1);
    for (int theme = 0; theme < 3; ++theme) {
        p.setTheme(theme);
        for (int face = 0; face < 3; ++face) {
            parameter(p, "kind")->setValueNotifyingHost(0);
            parameter(p, "face")->setValueNotifyingHost(parameter(p, "face")->convertTo0to1((float)face));
            float vw = 1100.0f, vh = 500.0f;
            arcvu_view_size(face, &vw, &vh);
            e->setSize((int) vw, (int) vh);
            process(p); pump();
            snapshot(*e, output.getChildFile("classic-" + juce::String(theme) + "-face-" + juce::String(face) + ".png"));
        }
    }
    // Loudness panel under every face and skin, fed with a -23 dBFS tone, and its buttons.
    for (int skin = 0; skin < 2; ++skin) {
        p.setSkin(skin);
        p.setTheme(0);
        p.setLoudness(ArcVuLoudSettings { 1, -14.0f, -1.0f, 0, 0, 10.0f });
        p.resetLoudness();
        for (int face = 0; face < 3; ++face) {
            parameter(p, "kind")->setValueNotifyingHost(0);
            parameter(p, "face")->setValueNotifyingHost(parameter(p, "face")->convertTo0to1((float)face));
            float vw = 1100.0f, vh = 500.0f;
            arcvu_view_size(face, &vw, &vh);
            check(p.loudnessHeight(face) > 0.0f, "the loudness panel adds height under face " + juce::String(face));
            e->setSize((int) vw * 2, (int) vh * 2);
            process(p); pump();
            snapshot(*e, output.getChildFile("loudness-skin-" + juce::String(skin) + "-face-" + juce::String(face) + ".png"));
            // Click the Target button: the setting changes and is kept.
            const float before = p.loudness().target;
            float lw = 0, lh = 0;
            arcvu_view_size(face, &lw, &lh);
            const float faceH = lh - p.loudnessHeight(face);
            const float bx = face == 2 ? 28.0f + (lw - 56.0f) / 3.0f * 1.5f : 28.0f + (lw - 56.0f) / 6.0f * 1.5f;
            const float by = face == 2 ? faceH + 184.0f + 13.0f : faceH + 134.0f + 13.0f;
            check(p.loudnessClick(bx, by) == 2, "the Target button answers a click on face " + juce::String(face));
            check(p.loudness().target != before, "the Target button changes the target");
            p.setLoudness(ArcVuLoudSettings { 1, -14.0f, -1.0f, 0, 0, 10.0f });
        }
        p.setLoudness(ArcVuLoudSettings { 0, -14.0f, -1.0f, 0, 0, 10.0f });
    }
    p.setSkin(0);
    {
        // Offline bounce log: a real-time render writes nothing, an offline one writes a summary and a per-second CSV.
        const juce::File logs = juce::File::getSpecialLocation(juce::File::tempDirectory).getChildFile("arcvu-bounce-logs-qa");
        logs.deleteRecursively();
        setenv("ARCVU_QA_LOG_DIR", logs.getFullPathName().toRawUTF8(), 1);
        p.prepareToPlay(48000, 512);
        p.setNonRealtime(false);
        process(p);
        check(!p.bounceActive() && p.finishBounce() == juce::File(), "a real-time render starts no bounce log");
        p.setNonRealtime(true);
        for (int pass = 0; pass < 12; ++pass) process(p);
        check(p.bounceActive(), "an offline render starts a bounce log");
        p.setNonRealtime(false);
        const auto file = p.finishBounce();
        check(file.existsAsFile() && file.withFileExtension(".csv").existsAsFile() && file.withFileExtension(".html").existsAsFile(), "the bounce log writes a summary, a CSV and a visual report");
        const auto text = file.loadFileAsString();
        check(text.contains("Integrated:") && text.contains("True peak:") && text.contains("Measured on:") && text.contains("48000 Hz"), "the bounce summary has the loudness figures");
        const auto lines = juce::StringArray::fromLines(file.withFileExtension(".csv").loadFileAsString());
        check(lines.size() >= 3 && lines[0].startsWith("seconds,") && lines[1].startsWith("1,"), "the CSV has a row per second");
        p.setBounceLog(false);
        p.setNonRealtime(true);
        process(p);
        check(!p.bounceActive(), "the bounce log can be switched off");
        p.setNonRealtime(false);
        p.setBounceLog(true);
        logs.deleteRecursively();
    }
    // Every meter type on the Classic dial, in each colour.
    for (int theme = 0; theme < 3; ++theme) {
        p.setTheme(theme);
        for (int kind = 0; kind < 9; ++kind) {
            parameter(p, "kind")->setValueNotifyingHost(parameter(p, "kind")->convertTo0to1((float)kind));
            parameter(p, "face")->setValueNotifyingHost(0);
            float vw = 1100.0f, vh = 500.0f;
            arcvu_view_size(0, &vw, &vh);
            e->setSize((int) vw, (int) vh);
            process(p); pump();
            snapshot(*e, output.getChildFile("classic-kind-" + juce::String(kind) + "-theme-" + juce::String(theme) + ".png"));
        }
    }
    parameter(p, "kind")->setValueNotifyingHost(0);
    p.setSkin(0);
    p.setTheme(0);
    parameter(p, "kind")->setValueNotifyingHost(0); parameter(p, "face")->setValueNotifyingHost(0);
    process(p); pump();
    for (int face = 0; face < 3; ++face) {
        auto* fp = parameter(p, "face"); fp->setValueNotifyingHost(fp->convertTo0to1((float)face));
        for (int scale = 0; scale < 3; ++scale) {
            auto* sp = parameter(p, "guiScale"); sp->setValueNotifyingHost(sp->convertTo0to1((float)scale));
            process(p); pump();
            snapshot(*e, output.getChildFile("face-" + juce::String(face) + "-scale-" + juce::String(scale) + ".png"));
        }
        parameter(p, "guiScale")->setValueNotifyingHost(0); process(p); pump();
        for (int group = 1; group <= 7; ++group) {
            e->showPanel(group); auto* panel = e->parameterPanel();
            check(panel != nullptr, "panel " + juce::String(group));
            for (int page = 0; page < panel->pageCount(); ++page) {
                panel->setPage(page);
                snapshot(*e, output.getChildFile("face-" + juce::String(face) + "-panel-" + juce::String(group) + "-page-" + juce::String(page) + ".png"));
                for (auto& row : panel->rows) {
                    if (!row->isVisible() || row->customId.isNotEmpty()) continue;
                    covered.insert(row->parameter.paramID);
                    check(panel->getLocalBounds().contains(row->getBounds()), "visible bounds " + row->parameter.paramID);
                    if (face != 0) continue;
                    auto& rp = row->parameter; const float saved = rp.getValue();
                    if (row->sa) {
                        auto val = rp.convertFrom0to1(0.71f);
                        row->slider.setValue(val, juce::sendNotificationSync);
                        check(std::abs(rp.getValue() - rp.convertTo0to1((float)row->slider.getValue())) < 0.001f, "GUI to host " + rp.paramID);
                        rp.setValueNotifyingHost(rp.convertTo0to1(rp.convertFrom0to1(0.21f))); pump();
                        check(std::abs((float)row->slider.getValue() - rp.convertFrom0to1(rp.getValue())) < 0.02f, "host to GUI " + rp.paramID);
                    } else if (row->ca) {
                        if (rp.paramID == "monitor") {
                            rp.setValueNotifyingHost(rp.convertTo0to1(4.0f)); pump();
                            row->monitorLR.onClick();
                            check(rp.convertFrom0to1(rp.getValue()) == 0.0f, "monitor LR reset callback returns to LR");
                        }
                        row->choice.setSelectedItemIndex(row->choice.getNumItems() - 1, juce::sendNotificationSync);
                        check(rp.getValue() > 0.99f, "GUI to host " + rp.paramID);
                        rp.setValueNotifyingHost(0); pump();
                        check(row->choice.getSelectedItemIndex() == 0, "host to GUI " + rp.paramID);
                    } else {
                        row->toggle.setToggleState(true, juce::sendNotificationSync);
                        check(rp.getValue() > 0.99f, "GUI to host " + rp.paramID);
                        rp.setValueNotifyingHost(0); pump();
                        check(!row->toggle.getToggleState(), "host to GUI " + rp.paramID);
                    }
                    rp.setValueNotifyingHost(saved); process(p); pump();
                }
            }
            e->showPanel(group);
        }
        parameter(p, "guiScale")->setValueNotifyingHost(1.0f); process(p); pump();
        for (int group = 1; group <= 7; ++group) {
            e->showPanel(group); auto* panel = e->parameterPanel();
            for (int page = 0; page < panel->pageCount(); ++page) {
                panel->setPage(page);
                snapshot(*e, output.getChildFile("face-" + juce::String(face) + "-panel-" + juce::String(group) + "-page-" + juce::String(page) + "-scale-2.png"));
            }
            e->showPanel(group);
        }
        parameter(p, "guiScale")->setValueNotifyingHost(0); process(p); pump();
        {
            auto* hpf = parameter(p, "hpfPoles");
            auto* trimM = parameter(p, "trimM");
            auto* polL = parameter(p, "polL");
            hpf->setValueNotifyingHost(hpf->convertTo0to1(1.0f));
            trimM->setValueNotifyingHost(trimM->convertTo0to1(6.0f));
            polL->setValueNotifyingHost(1.0f);
            process(p); pump();
            snapshot(*e, output.getChildFile("face-" + juce::String(face) + "-activity-on.png"));
            hpf->setValueNotifyingHost(0.0f); trimM->setValueNotifyingHost(0.0f); polL->setValueNotifyingHost(0.0f);
            process(p); pump();
            snapshot(*e, output.getChildFile("face-" + juce::String(face) + "-activity-off.png"));
        }
        {
            auto* ms = parameter(p, "midSide");
            auto* trimM = parameter(p, "trimM");
            ms->setValueNotifyingHost(ms->convertTo0to1(0.0f));
            trimM->setValueNotifyingHost(trimM->convertTo0to1(6.0f));
            process(p); pump();
            snapshot(*e, output.getChildFile("face-" + juce::String(face) + "-hidden-ms-trim.png"));
            ms->setValueNotifyingHost(0.0f); trimM->setValueNotifyingHost(0.0f);
            process(p); pump();
        }
        p.setup.panel = 8; p.noteSetup(); process(p); pump();
        snapshot(*e, output.getChildFile("face-" + juce::String(face) + "-preset-menu.png"));
        p.setup.panel = 0; p.noteSetup(); process(p); pump();
        for (auto* id : {"single", "midSide", "metersOnly", "showTools"}) {
            parameter(p, id)->setValueNotifyingHost(1); process(p); pump();
            snapshot(*e, output.getChildFile("face-" + juce::String(face) + "-" + id + ".png"));
            parameter(p, id)->setValueNotifyingHost(0); process(p); pump();
            if (juce::String(id) == "showTools") {
                check(p.setup.show_tools == 0, "Show tools host state off");
                snapshot(*e, output.getChildFile("face-" + juce::String(face) + "-showTools-off.png"));
            }
        }
    }
    for (int i = 0; i < p.getParameters().size(); ++i) p.getParameters()[i]->setValueNotifyingHost(defaults[i]);
    for (auto* ap : p.getParameters()) {
        auto* rp = dynamic_cast<juce::RangedAudioParameter*>(ap);
        rp->setValueNotifyingHost(rp->isDiscrete() ? 1.0f : 0.85f);
    }
    p.setup.label[0] = 'X';
    p.applyFactory(4);
    for (int n = 0; n < p.getParameters().size(); ++n)
        check(std::abs(p.getParameters()[n]->getValue() - defaults[n]) < 0.001f,
              "factory reset default " + p.getParameters()[n]->getName(64));
    check(p.setup.label[0] != 'X', "factory reset restores instance label");
    // Preserve all host parameters and the UTF-8 label through the actual blob path.
    int i = 0;
    for (auto* ap : p.getParameters()) {
        float value = ((i++ % 7) + 1) / 8.0f;
        if (ap->isDiscrete()) value = std::round(value * (ap->getNumSteps() - 1)) / (ap->getNumSteps() - 1);
        ap->setValueNotifyingHost(value);
    }
    process(p);
    juce::MemoryBlock state; p.getStateInformation(state);
    ArcVuProcessor restored; restored.setStateInformation(state.getData(), (int)state.getSize());
    for (auto* ap : p.getParameters()) {
        auto* rp = dynamic_cast<juce::RangedAudioParameter*>(ap);
        check(covered.count(rp->paramID) != 0, "parameter coverage " + rp->paramID);
        auto* other = parameter(restored, rp->paramID.toRawUTF8());
        check(other && std::abs(other->getValue() - rp->getValue()) < 0.001f, "state roundtrip " + rp->paramID);
    }
    // Hosts may save automation changes before another audio callback occurs.
    auto* trim = parameter(p, "trimL");
    trim->setValueNotifyingHost(trim->convertTo0to1(13.5f));
    juce::MemoryBlock stoppedState; p.getStateInformation(stoppedState);
    ArcVuProcessor stopped; stopped.setStateInformation(stoppedState.getData(), (int)stoppedState.getSize());
    check(std::abs(parameter(stopped, "trimL")->convertFrom0to1(parameter(stopped, "trimL")->getValue()) - 13.5f) < 0.001f, "state captures host change while stopped");
    p.getStateInformation(stoppedState);
    check(stoppedState.getSize() == sizeof(ArcVuSetup), "state overwrites reused MemoryBlock");
    check(covered.size() == (size_t)p.getParameters().size(), "coverage count " + juce::String((int)covered.size()));
    for (int n = 0; n < 10; ++n) { e.reset(static_cast<ArcVuEditor*>(p.createEditor())); process(p); pump(); }
    check(true, "editor lifecycle 10 reopenings");
    std::cout << "FAILURES " << failures << std::endl;
    return failures ? 1 : 0;
}
