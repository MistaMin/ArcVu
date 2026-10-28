#include "PluginProcessor.h"
#include "PluginEditor.h"
#include <algorithm>
#include <cmath>
#include <cstdlib>
#include <cstring>
#include <vector>

ArcVuProcessor::ArcVuProcessor()
    : AudioProcessor(BusesProperties().withInput("Input", juce::AudioChannelSet::stereo(), true)
                                      .withOutput("Output", juce::AudioChannelSet::stereo(), true)) {
    addParameter(referenceDb = new juce::AudioParameterChoice(
        juce::ParameterID { "reference", 1 }, "0 VU reference",
        juce::StringArray { "20", "18", "14" }, 1));
    addParameter(peakModeParam = new juce::AudioParameterChoice(
        juce::ParameterID { "peak", 1 }, "Peak",
        juce::StringArray { "Follow", "Auto", "Hold" }, 1));
    auto choice = [this](const char* id, const char* name, juce::StringArray items, int def) {
        auto* p = new juce::AudioParameterChoice(juce::ParameterID { id, 1 }, name, items, def);
        addParameter(p);
        return p;
    };
    auto flag = [this](const char* id, const char* name, bool def) {
        auto* p = new juce::AudioParameterBool(juce::ParameterID { id, 1 }, name, def);
        addParameter(p);
        return p;
    };
    auto flt = [this](const char* id, const char* name, float lo, float hi, float def) {
        auto* p = new juce::AudioParameterFloat(juce::ParameterID { id, 1 }, name, lo, hi, def);
        addParameter(p);
        return p;
    };
    auto num = [this](const char* id, const char* name, int lo, int hi, int def) {
        auto* p = new juce::AudioParameterInt(juce::ParameterID { id, 1 }, name, lo, hi, def);
        addParameter(p);
        return p;
    };
    kindP = choice("kind", "Meter type", juce::StringArray { "VU", "RMS", "K-12", "K-14", "K-20", "DIN", "Nordic", "BBC", "EBU" }, 0);
    faceP = choice("face", "Face", juce::StringArray { "Arc", "Horizontal", "Vertical" }, 0);
    singleP = flag("single", "Single meter", false);
    midP = flag("midSide", "Mid side", false);
    aesP = flag("aes17", "AES-17", true);
    rmsP = flt("rmsMs", "RMS window", 30.0f, 2000.0f, 300.0f);
    clipP = flt("clipDb", "Clip threshold", -12.0f, 0.0f, 0.0f);
    const char* trimNames[] = { "trimL", "trimR", "trimM", "trimS" };
    const char* trimLabels[] = { "Trim L", "Trim R", "Trim M", "Trim S" };
    for (int i = 0; i < 4; ++i) trimP[i] = flt(trimNames[i], trimLabels[i], -24.0f, 24.0f, 0.0f);
    polP[0] = flag("polL", "Polarity L", false);
    polP[1] = flag("polR", "Polarity R", false);
    muteP[0] = flag("muteL", "Mute L", false);
    muteP[1] = flag("muteR", "Mute R", false);
    monitorP = choice("monitor", "Monitor", juce::StringArray { "LR", "RL", "L", "R", "Mid", "Side" }, 0);
    hpfHzP = flt("hpfHz", "High-pass", 20.0f, 500.0f, 80.0f);
    hpfPolesP = num("hpfPoles", "High-pass slope", 0, 4, 0);
    lpfHzP = flt("lpfHz", "Low-pass", 1000.0f, 20000.0f, 16000.0f);
    lpfPolesP = num("lpfPoles", "Low-pass slope", 0, 4, 0);
    monoHzP = flt("monoHz", "Mono frequency", 20.0f, 400.0f, 120.0f);
    monoAmtP = flt("monoAmt", "Mono amount", 0.0f, 1.0f, 0.0f);
    dynHzP = flt("dynHz", "Dynamic frequency", 80.0f, 12000.0f, 4000.0f);
    dynDepthP = flt("dynDepth", "Dynamic depth", 0.0f, 1.0f, 0.0f);
    dynOnP = flag("dynOn", "Dynamic on", false);
    dynTypeP = num("dynType", "Dynamic type", 0, 4, 0);
    dynListenP = flag("dynListen", "Dynamic listen", false);
    vuModelP = num("vuModel", "VU model", 0, 2, 0);
    riseP = flt("riseMs", "Rise", 50.0f, 1000.0f, 300.0f);
    fallP = flt("fallMs", "Fall", 50.0f, 2000.0f, 300.0f);
    overP = flt("overshoot", "Overshoot", 0.2f, 8.0f, 1.0f);
    peakHoldP = flt("peakHold", "Peak hold", -1.0f, 30.0f, 3.0f);
    valueHoldP = flt("valueHold", "Value hold", -1.0f, 30.0f, 5.0f);
    clipHoldP = flt("clipHold", "Clip hold", -1.0f, 30.0f, 2.0f);
    peakThreshP = flt("peakThresh", "Peak lamp threshold", -24.0f, 0.0f, -6.0f);
    redP = flt("redDb", "Red zone", -12.0f, 12.0f, 0.0f);
    comfortP = flt("comfort", "Comfort zone", 1.0f, 12.0f, 4.0f);
    analogP = flag("analogRms", "Analog RMS", true);
    weightP = num("weight", "Weighting", 0, 5, 0);
    forceP = flag("forceMono", "Force mono", false);
    linkP = flag("trimLink", "Trim link", false);
    inPlaceP = flag("inPlace", "In place", true);
    showValuesP = flag("showValues", "Show values", true);
    showHoldP = flag("showHold", "Show hold", true);
    showToolsP = flag("showTools", "Show tools", true);
    showLabelP = flag("showLabel", "Show label", true);
    metersOnlyP = flag("metersOnly", "Meters only", false);
    guiScaleP = num("guiScale", "GUI size", 0, 2, 0);
    dragP = flt("dragSens", "Drag sensitivity", 0.25f, 4.0f, 1.0f);
    refreshP = num("refresh", "Meter refresh", 10, 60, 30);
    bypassP = flag("bypassHidden", "Bypass hidden processors", false);
    tipsP = flag("tooltips", "Tooltips", true);
    topP = flag("alwaysOnTop", "Stay above windows", false);
    meter = arcvu_create(48000.0);
    setup.kind = 0;
    setup.face = 0;
    setup.aes17 = 1;
    setup.rms_ms = 300.0f;
    setup.clip_db = 0.0f;
    setup.reference_db = -18.0f;
    setup.peak_mode = 1;
    setup.hpf_hz = 80.0f;
    setup.lpf_hz = 16000.0f;
    setup.mono_hz = 120.0f;
    setup.dyn_hz = 4000.0f;
    setup.panel = 0;
    setup.rise_ms = 300.0f;
    setup.fall_ms = 300.0f;
    setup.overshoot = 1.0f;
    setup.peak_hold_s = 3.0f;
    setup.value_hold_s = 5.0f;
    setup.clip_hold_s = 2.0f;
    setup.peak_thresh_db = -6.0f;
    setup.red_db = 0.0f;
    setup.comfort_db = 4.0f;
    setup.analog_rms = 1;
    setup.show_values = 1;
    setup.show_hold = 1;
    setup.show_tools = 1;
    setup.show_label = 1;
    setup.in_place = 1;
    setup.drag_sens = 1.0f;
    setup.refresh_hz = 30;
    setup.tooltips = 1;
    std::strncpy(reinterpret_cast<char*>(setup.label), "ArcVu", sizeof setup.label - 1);
    pullSetup();
    initialSetup = setup;
    loadDefault();
    loadGlobalSettings();
    startTimer(1000);
}

ArcVuProcessor::~ArcVuProcessor() {
    stopTimer();
    finishBounce();
    arcvu_destroy(meter);
}

void ArcVuProcessor::prepareToPlay(double sampleRate, int samplesPerBlock) {
    finishBounce();
    preparedRate = sampleRate;
    arcvu_set_sample_rate(meter, sampleRate);
    interleaved.reserve((size_t) juce::jmax(1, samplesPerBlock) * 2);
}

bool ArcVuProcessor::isBusesLayoutSupported(const BusesLayout& layouts) const {
    const auto& out = layouts.getMainOutputChannelSet();
    if (out != juce::AudioChannelSet::mono() && out != juce::AudioChannelSet::stereo()) return false;
    return layouts.getMainInputChannelSet() == out;
}

void ArcVuProcessor::processBlock(juce::AudioBuffer<float>& buffer, juce::MidiBuffer&) {
    juce::ScopedNoDenormals noDenormals;
    for (int ch = getTotalNumInputChannels(); ch < getTotalNumOutputChannels(); ++ch)
        buffer.clear(ch, 0, buffer.getNumSamples());

    // An offline render starts a bounce log; it ends when the host releases the plug-in or stops sending blocks.
    const bool offline = isNonRealtime();
    if (offline && bounceLogOn && !bounceRunning.load()) {
        arcvu_bounce(meter, 1);
        bounceStartMs = juce::Time::currentTimeMillis();
        bounceRunning = true;
    }
    if (bounceRunning.load()) {
        lastBlockOffline = offline;
        lastBlockMs = juce::Time::getMillisecondCounter();
    }

    // Message-thread code owns `setup`; the audio thread works from its own copy plus the parameters.
    audioSetup = setup;
    readParams(audioSetup);
    arcvu_setup(meter, &audioSetup);

    const int channels = juce::jlimit(1, 2, buffer.getNumChannels());
    const int frames = buffer.getNumSamples();
    interleaved.resize((size_t) frames * (size_t) channels);
    auto* dest = interleaved.data();
    for (int i = 0; i < frames; ++i)
        for (int ch = 0; ch < channels; ++ch)
            dest[i * channels + ch] = buffer.getReadPointer(ch)[i];
    arcvu_process(meter, dest, frames, channels);
    for (int i = 0; i < frames; ++i)
        for (int ch = 0; ch < channels; ++ch)
            buffer.getWritePointer(ch)[i] = dest[i * channels + ch];

    ArcVuLevels next[2] {};
    arcvu_levels(meter, 0, &next[0]);
    arcvu_levels(meter, channels > 1 ? 1 : 0, &next[1]);
    const juce::SpinLock::ScopedLockType lock(levelLock);
    cached[0] = next[0];
    cached[1] = next[1];
}

int ArcVuProcessor::paint(ArcVuPaint* out, int cap) const {
    const float norm = referenceDb->getIndex() / 2.0f;
    return arcvu_paint(meter, norm, out, cap);
}

ArcVuLevels ArcVuProcessor::levels(int channel) const {
    const juce::SpinLock::ScopedLockType lock(levelLock);
    return cached[juce::jlimit(0, 1, channel)];
}

juce::AudioProcessorEditor* ArcVuProcessor::createEditor() { return new ArcVuEditor(*this); }

namespace {
void setFloat(juce::AudioParameterFloat* p, float v) { *p = v; }
void setBool(juce::AudioParameterBool* p, int v) { *p = v != 0; }
void setInt(juce::AudioParameterInt* p, int v) { *p = v; }
void setChoice(juce::AudioParameterChoice* p, int v) { *p = v; }
}

// Reads every parameter into s. Parameters are the only state the audio thread may read, so it never touches the shared s.
void ArcVuProcessor::readParams(ArcVuSetup& s) const {
    static const float refs[] = { -20.0f, -18.0f, -14.0f };
    s.kind = kindP->getIndex();
    s.face = faceP->getIndex();
    s.single = singleP->get() ? 1 : 0;
    s.mid_side = midP->get() ? 1 : 0;
    s.aes17 = aesP->get() ? 1 : 0;
    s.peak_mode = peakModeParam->getIndex();
    s.rms_ms = rmsP->get();
    s.clip_db = clipP->get();
    s.reference_db = refs[juce::jlimit(0, 2, referenceDb->getIndex())];
    s.trim_l = trimP[0]->get();
    s.trim_r = trimP[1]->get();
    s.trim_m = trimP[2]->get();
    s.trim_s = trimP[3]->get();
    s.pol_l = polP[0]->get();
    s.pol_r = polP[1]->get();
    s.mute_l = muteP[0]->get();
    s.mute_r = muteP[1]->get();
    s.monitor = monitorP->getIndex();
    s.hpf_hz = hpfHzP->get();
    s.hpf_poles = hpfPolesP->get();
    s.lpf_hz = lpfHzP->get();
    s.lpf_poles = lpfPolesP->get();
    s.mono_hz = monoHzP->get();
    s.mono_amt = monoAmtP->get();
    s.dyn_hz = dynHzP->get();
    s.dyn_depth = dynDepthP->get();
    s.dyn_on = dynOnP->get();
    s.dyn_type = dynTypeP->get();
    s.dyn_listen = dynListenP->get();
    s.vu_model = vuModelP->get();
    s.rise_ms = riseP->get();
    s.fall_ms = fallP->get();
    s.overshoot = overP->get();
    s.peak_hold_s = peakHoldP->get();
    s.value_hold_s = valueHoldP->get();
    s.clip_hold_s = clipHoldP->get();
    s.peak_thresh_db = peakThreshP->get();
    s.red_db = redP->get();
    s.comfort_db = comfortP->get();
    s.analog_rms = analogP->get();
    s.weight = weightP->get();
    s.force_mono = forceP->get();
    s.trim_link = linkP->get();
    s.in_place = inPlaceP->get();
    s.show_values = showValuesP->get();
    s.show_hold = showHoldP->get();
    s.show_tools = showToolsP->get();
    s.show_label = showLabelP->get();
    s.meters_only = metersOnlyP->get();
    s.gui_scale = guiScaleP->get();
    s.drag_sens = dragP->get();
    s.refresh_hz = refreshP->get();
    s.bypass_hidden = bypassP->get();
    s.tooltips = tipsP->get();
    s.always_on_top = topP->get();
}

void ArcVuProcessor::pullSetup() { readParams(setup); }

void ArcVuProcessor::noteSetup() {
    int ref = 1;
    if (setup.reference_db > -16.0f) ref = 2;
    else if (setup.reference_db < -19.0f) ref = 0;
    setChoice(kindP, setup.kind);
    setChoice(faceP, setup.face);
    setBool(singleP, setup.single);
    setBool(midP, setup.mid_side);
    setBool(aesP, setup.aes17);
    setChoice(peakModeParam, setup.peak_mode);
    setChoice(referenceDb, ref);
    setFloat(rmsP, setup.rms_ms);
    setFloat(clipP, setup.clip_db);
    setFloat(trimP[0], setup.trim_l);
    setFloat(trimP[1], setup.trim_r);
    setFloat(trimP[2], setup.trim_m);
    setFloat(trimP[3], setup.trim_s);
    setBool(polP[0], setup.pol_l);
    setBool(polP[1], setup.pol_r);
    setBool(muteP[0], setup.mute_l);
    setBool(muteP[1], setup.mute_r);
    setChoice(monitorP, setup.monitor);
    setFloat(hpfHzP, setup.hpf_hz);
    setInt(hpfPolesP, setup.hpf_poles);
    setFloat(lpfHzP, setup.lpf_hz);
    setInt(lpfPolesP, setup.lpf_poles);
    setFloat(monoHzP, setup.mono_hz);
    setFloat(monoAmtP, setup.mono_amt);
    setFloat(dynHzP, setup.dyn_hz);
    setFloat(dynDepthP, setup.dyn_depth);
    setBool(dynOnP, setup.dyn_on);
    setInt(dynTypeP, setup.dyn_type);
    setBool(dynListenP, setup.dyn_listen);
    setInt(vuModelP, setup.vu_model);
    setFloat(riseP, setup.rise_ms);
    setFloat(fallP, setup.fall_ms);
    setFloat(overP, setup.overshoot);
    setFloat(peakHoldP, setup.peak_hold_s);
    setFloat(valueHoldP, setup.value_hold_s);
    setFloat(clipHoldP, setup.clip_hold_s);
    setFloat(peakThreshP, setup.peak_thresh_db);
    setFloat(redP, setup.red_db);
    setFloat(comfortP, setup.comfort_db);
    setBool(analogP, setup.analog_rms);
    setInt(weightP, setup.weight);
    setBool(forceP, setup.force_mono);
    setBool(linkP, setup.trim_link);
    setBool(inPlaceP, setup.in_place);
    setBool(showValuesP, setup.show_values);
    setBool(showHoldP, setup.show_hold);
    setBool(showToolsP, setup.show_tools);
    setBool(showLabelP, setup.show_label);
    setBool(metersOnlyP, setup.meters_only);
    setInt(guiScaleP, setup.gui_scale);
    setFloat(dragP, setup.drag_sens);
    setInt(refreshP, setup.refresh_hz > 0 ? setup.refresh_hz : 30);
    setBool(bypassP, setup.bypass_hidden);
    setBool(tipsP, setup.tooltips);
    setBool(topP, setup.always_on_top);
}

void ArcVuProcessor::applyFactory(int which) {
    if (which == 0) { setup.kind = 0; setup.reference_db = -18.0f; setup.peak_mode = 1; setup.aes17 = 1; }
    else if (which == 1) { setup.kind = 3; setup.analog_rms = 1; }
    else if (which == 2) { setup.kind = 5; }
    else if (which == 3) { setup.kind = 0; setup.reference_db = -14.0f; }
    else {
        setup = initialSetup;
    }
    noteSetup();
}

static juce::File arcVuDefaultFile() {
#if defined(ARCVU_BUILD_QA)
    if (const char* overridePath = std::getenv("ARCVU_QA_DEFAULT_FILE"); overridePath != nullptr && *overridePath != '\0')
        return juce::File(overridePath);
#endif
    return juce::File::getSpecialLocation(juce::File::userApplicationDataDirectory)
        .getChildFile("ArcVu").getChildFile("default.arcvu");
}

static juce::File arcVuGlobalFile() {
#if defined(ARCVU_BUILD_QA)
    if (const char* overridePath = std::getenv("ARCVU_QA_GLOBAL_FILE"); overridePath != nullptr && *overridePath != '\0')
        return juce::File(overridePath);
#endif
    return juce::File::getSpecialLocation(juce::File::userApplicationDataDirectory)
        .getChildFile("ArcVu").getChildFile("global.txt");
}

void ArcVuProcessor::saveGlobalKey(const juce::String& key, const juce::String& value) {
    const auto file = arcVuGlobalFile();
    juce::StringArray lines = file.existsAsFile() ? juce::StringArray::fromLines(file.loadFileAsString()) : juce::StringArray();
    bool found = false;
    for (auto& line : lines) {
        if (line.upToFirstOccurrenceOf("=", false, false).trim() == key) { line = key + "=" + value; found = true; }
    }
    if (!found) lines.add(key + "=" + value);
    lines.removeEmptyStrings();
    file.getParentDirectory().createDirectory();
    file.replaceWithText(lines.joinIntoString("\n") + "\n");
}

void ArcVuProcessor::saveGlobalSettings() {
    pullSetup();
    juce::String text;
    text << "drag_mode=" << dragModeIndex << "\n"
         << "drag_sens=" << juce::String(setup.drag_sens, 4) << "\n"
         << "refresh_hz=" << setup.refresh_hz << "\n"
         << "bypass_hidden=" << setup.bypass_hidden << "\n"
         << "tooltips=" << setup.tooltips << "\n"
         << "theme=" << themeIndex << "\n"
         << "skin=" << skinIndex << "\n"
         << "peak_scale=" << peakScaleIndex << "\n"
         << "bounce_log=" << (bounceLogOn ? 1 : 0) << "\n"
         << "loud_on=" << loud.on << "\n"
         << "loud_target=" << juce::String(loud.target, 1) << "\n"
         << "loud_ceiling=" << juce::String(loud.ceiling, 1) << "\n"
         << "loud_scale=" << loud.scale << "\n"
         << "loud_unit=" << loud.unit << "\n"
         << "loud_window=" << juce::String(loud.window, 1) << "\n"
         << "zoom=" << juce::String(zoomValue, 4) << "\n";
    arcVuGlobalFile().getParentDirectory().createDirectory();
    arcVuGlobalFile().replaceWithText(text);
}

void ArcVuProcessor::loadGlobalSettings() {
    arcvu_set_loudness(meter, &loud);  // also when there is no file, so the window size follows these settings
    const auto file = arcVuGlobalFile();
    if (!file.existsAsFile()) return;
    for (const auto& line : juce::StringArray::fromLines(file.loadFileAsString())) {
        const auto key = line.upToFirstOccurrenceOf("=", false, false).trim();
        const auto value = line.fromFirstOccurrenceOf("=", false, false).trim();
        if (key == "drag_mode") dragModeIndex = juce::jlimit(0, 1, value.getIntValue());
        else if (key == "drag_sens") setup.drag_sens = juce::jlimit(0.25f, 4.0f, value.getFloatValue());
        else if (key == "refresh_hz") setup.refresh_hz = juce::jlimit(5, 60, value.getIntValue());
        else if (key == "bypass_hidden") setup.bypass_hidden = value.getIntValue() != 0 ? 1 : 0;
        else if (key == "tooltips") setup.tooltips = value.getIntValue() != 0 ? 1 : 0;
        else if (key == "theme") setTheme(value.getIntValue());
        else if (key == "skin") setSkin(value.getIntValue());
        else if (key == "peak_scale") setPeakScale(value.getIntValue());
        else if (key == "bounce_log") bounceLogOn = value.getIntValue() != 0;
        else if (key == "loud_on") loud.on = value.getIntValue() != 0 ? 1 : 0;
        else if (key == "loud_target") loud.target = juce::jlimit(-40.0f, -5.0f, value.getFloatValue());
        else if (key == "loud_ceiling") loud.ceiling = juce::jlimit(-12.0f, 0.0f, value.getFloatValue());
        else if (key == "loud_scale") loud.scale = juce::jlimit(0, 1, value.getIntValue());
        else if (key == "loud_unit") loud.unit = juce::jlimit(0, 1, value.getIntValue());
        else if (key == "loud_window") loud.window = juce::jlimit(1.0f, 30.0f, value.getFloatValue());
        else if (key == "zoom") zoomValue = juce::jlimit(0.4f, 4.0f, value.getFloatValue());
    }
    arcvu_set_loudness(meter, &loud);
    noteSetup();
}

void ArcVuProcessor::saveDefault() {
    auto file = arcVuDefaultFile();
    file.getParentDirectory().createDirectory();
    file.replaceWithData(&setup, sizeof setup);
}

void ArcVuProcessor::loadDefault() {
    auto file = arcVuDefaultFile();
    juce::MemoryBlock block;
    if (file.getSize() >= (juce::int64) sizeof(ArcVuSetup) && file.loadFileAsData(block))
        setStateInformation(block.getData(), (int) block.getSize());
}

void ArcVuProcessor::copySetup() {
    juce::MemoryBlock block(&setup, sizeof setup);
    juce::SystemClipboard::copyTextToClipboard(block.toBase64Encoding());
}

void ArcVuProcessor::pasteSetup() {
    juce::MemoryBlock block;
    if (block.fromBase64Encoding(juce::SystemClipboard::getTextFromClipboard()))
        setStateInformation(block.getData(), (int) block.getSize());
}

juce::String ArcVuProcessor::fitLabel(juce::String text) {
    // Labels hold at most 31 UTF-8 bytes; dropping whole characters keeps every sequence valid.
    while (text.getNumBytesAsUTF8() > 31) text = text.dropLastCharacters(1);
    return text;
}

void ArcVuProcessor::stepPreset(int delta) {
    // An unset index starts at the first item going forward and the last going back.
    const auto nextIndex = [delta](int current, int count) {
        if (current < 0) return delta > 0 ? 0 : count - 1;
        return (current + delta + count) % count;
    };
    if (presetCategoryIndex == 0) {
        constexpr int count = 4;
        factoryIndex = nextIndex(factoryIndex, count);
        applyFactory(factoryIndex);
        return;
    }
    const auto files = userPresetFiles();
    if (files.isEmpty()) return;
    userIndex = nextIndex(userIndex, files.size());
    loadPresetFile(files.getReference(userIndex));
}

void ArcVuProcessor::setZoom(float zoom) {
    if (std::abs(zoom - zoomValue) < 0.005f) return;
    zoomValue = juce::jlimit(0.4f, 4.0f, zoom);
    saveGlobalKey("zoom", juce::String(zoomValue, 4));
}

void ArcVuProcessor::setSkin(int skin) {
    skinIndex = juce::jlimit(0, 1, skin);
    arcvu_set_skin(meter, skinIndex);
}

void ArcVuProcessor::setPeakScale(int scale) {
    peakScaleIndex = juce::jlimit(0, 1, scale);
    arcvu_set_peak_scale(meter, peakScaleIndex);
}

void ArcVuProcessor::setLoudness(const ArcVuLoudSettings& s) {
    loud = s;
    arcvu_set_loudness(meter, &loud);
    arcvu_get_loudness(meter, &loud);  // the meter holds the clamped values
    saveGlobalKey("loud_on", juce::String(loud.on));
    saveGlobalKey("loud_target", juce::String(loud.target, 1));
    saveGlobalKey("loud_ceiling", juce::String(loud.ceiling, 1));
    saveGlobalKey("loud_scale", juce::String(loud.scale));
    saveGlobalKey("loud_unit", juce::String(loud.unit));
    saveGlobalKey("loud_window", juce::String(loud.window, 1));
}

void ArcVuProcessor::pullLoudness() {
    arcvu_get_loudness(meter, &loud);
    setLoudness(loud);
}

ArcVuLoudness ArcVuProcessor::loudnessReadings() const {
    ArcVuLoudness r {};
    arcvu_loudness(meter, &r);
    return r;
}

int ArcVuProcessor::loudnessClick(float x, float y) {
    const int result = arcvu_loudness_click(meter, x, y);
    if (result == 2) pullLoudness();
    return result;
}

void ArcVuProcessor::setBounceLog(bool on) {
    bounceLogOn = on;
    saveGlobalKey("bounce_log", juce::String(on ? 1 : 0));
}

juce::File ArcVuProcessor::bounceLogDirectory() {
#if defined(ARCVU_BUILD_QA)
    if (const char* overridePath = std::getenv("ARCVU_QA_LOG_DIR"); overridePath != nullptr && *overridePath != '\0')
        return juce::File(overridePath);
#endif
    return juce::File::getSpecialLocation(juce::File::userDocumentsDirectory).getChildFile("ArcVu Bounce Logs");
}

// The host calls this on the message thread; it ends a bounce whose blocks stopped or turned real-time.
void ArcVuProcessor::timerCallback() {
    if (bounceRunning.load() && (!lastBlockOffline.load() || juce::Time::getMillisecondCounter() - lastBlockMs.load() > 2000))
        finishBounce();
}

namespace {
juce::String lufs(float v, const char* unit) { return v <= -100.0f ? juce::String("no signal") : juce::String(v, 1) + " " + unit; }
juce::String clock(double seconds) {
    const int total = (int) seconds;
    return juce::String::formatted("%d:%02d.%d", total / 60, total % 60, (int) ((seconds - total) * 10.0));
}
juce::String clockShort(double seconds) {
    const int total = (int) std::lround(seconds);
    return total >= 3600 ? juce::String::formatted("%d:%02d:%02d", total / 3600, (total / 60) % 60, total % 60)
                         : juce::String::formatted("%d:%02d", total / 60, total % 60);
}
juce::String esc(const juce::String& s) { return s.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace("\"", "&quot;"); }

// One chart: the series are columns of the per-second rows (seconds, M, S, I, true peak). Values at or below -100 are gaps.
struct Series { int column; const char* colour; float width; const char* name; };
juce::String chart(const std::vector<float>& rows, int count, const std::vector<Series>& series, float guide, float band, const char* guideName, bool allowAboveGuide) {
    const float W = 900.0f, H = 250.0f, L = 52.0f, R = 14.0f, T = 12.0f, B = 30.0f;
    float lo = 1.0e9f, hi = -1.0e9f;
    for (int i = 0; i < count; ++i)
        for (const auto& s : series) {
            const float v = rows[(size_t) i * 5 + (size_t) s.column];
            if (v > -100.0f) { lo = juce::jmin(lo, v); hi = juce::jmax(hi, v); }
        }
    if (lo > hi) { lo = guide - 12.0f; hi = guide + 3.0f; }
    lo = juce::jmin(lo, guide - (allowAboveGuide ? 6.0f : 3.0f)); hi = juce::jmax(hi, guide + 3.0f);
    lo = std::floor((juce::jmax(lo, -70.0f) - 1.0f) / 3.0f) * 3.0f;
    hi = std::ceil((juce::jmin(hi, 6.0f) + 1.0f) / 3.0f) * 3.0f;
    const double total = juce::jmax(1.0, (double) rows[(size_t) (count - 1) * 5]);
    const auto X = [&](double s) { return L + (float) (s / total) * (W - L - R); };
    const auto Y = [&](float v) { return T + (hi - juce::jlimit(lo, hi, v)) / (hi - lo) * (H - T - B); };
    juce::String svg;
    svg << "<svg viewBox=\"0 0 900 250\" role=\"img\" xmlns=\"http://www.w3.org/2000/svg\">";
    if (band > 0.0f)
        svg << "<rect x=\"" << L << "\" y=\"" << Y(guide + band) << "\" width=\"" << (W - L - R) << "\" height=\"" << (Y(guide - band) - Y(guide + band)) << "\" fill=\"#46e16e\" opacity=\"0.10\"/>";
    for (float v = lo; v <= hi + 0.01f; v += 3.0f)
        svg << "<line x1=\"" << L << "\" x2=\"" << (W - R) << "\" y1=\"" << Y(v) << "\" y2=\"" << Y(v) << "\" stroke=\"#2a3238\"/><text x=\"" << (L - 6) << "\" y=\"" << (Y(v) + 4) << "\" text-anchor=\"end\" class=\"ax\">" << (int) v << "</text>";
    static const int steps[] = { 5, 10, 15, 30, 60, 120, 300, 600, 900, 1800, 3600, 7200 };
    int step = 7200;
    for (int s : steps) if (total / s <= 10) { step = s; break; }
    for (double s = 0; s <= total + 0.01; s += step)
        svg << "<line x1=\"" << X(s) << "\" x2=\"" << X(s) << "\" y1=\"" << (H - B) << "\" y2=\"" << (H - B + 4) << "\" stroke=\"#6c777d\"/><text x=\"" << X(s) << "\" y=\"" << (H - B + 17) << "\" text-anchor=\"middle\" class=\"ax\">" << clockShort(s) << "</text>";
    svg << "<line x1=\"" << L << "\" x2=\"" << (W - R) << "\" y1=\"" << Y(guide) << "\" y2=\"" << Y(guide) << "\" stroke=\"#f2f6f3\" stroke-dasharray=\"6 4\"/>"
        << "<text x=\"" << (W - R - 4) << "\" y=\"" << (Y(guide) - 5) << "\" text-anchor=\"end\" class=\"ax\" fill=\"#f2f6f3\">" << esc(guideName) << "</text>";
    const int stride = juce::jmax(1, count / 1500);
    for (const auto& s : series) {
        juce::String d;
        bool pen = false;
        for (int i = 0; i < count; i += stride) {
            const float v = rows[(size_t) i * 5 + (size_t) s.column];
            if (v <= -100.0f) { pen = false; continue; }
            d << (pen ? "L" : "M") << juce::String(X(rows[(size_t) i * 5]), 1) << " " << juce::String(Y(v), 1) << " ";
            pen = true;
        }
        if (d.isNotEmpty()) svg << "<path d=\"" << d.trim() << "\" fill=\"none\" stroke=\"" << s.colour << "\" stroke-width=\"" << s.width << "\" stroke-linejoin=\"round\"/>";
    }
    svg << "</svg>";
    return svg;
}
}

juce::File ArcVuProcessor::finishBounce() {
    if (!bounceRunning.exchange(false)) return {};
    arcvu_bounce(meter, 0);
    ArcVuLoudness r {};
    uint64_t frames = 0;
    arcvu_bounce_readings(meter, &r, &frames);
    const double sr = juce::jmax(1.0, preparedRate);
    const double seconds = (double) frames / sr;
    if (seconds < 1.0) return {};  // a probe or a very short render is not worth a file

    std::vector<float> rows(5 * 36000);
    const int rowCount = arcvu_bounce_rows(meter, rows.data(), 36000);
    const auto dir = bounceLogDirectory();
    if (!dir.createDirectory().wasOk()) return {};
    const auto started = juce::Time(bounceStartMs.load());
    const auto stem = "ArcVu bounce " + started.formatted("%Y-%m-%d %H-%M-%S");
    const auto summary = dir.getNonexistentChildFile(stem, ".txt", false);
    const auto csv = summary.withFileExtension(".csv");

    const auto& ls = loudness();
    const float offTarget = r.integrated - ls.target;
    juce::String t;
    t << "ArcVu bounce log\n"
      << "================\n"
      << "Started:      " << started.toString(true, true, true, true) << "\n"
      << "Host:         " << juce::PluginHostType().getHostDescription() << "\n"
      << "Plug-in:      ArcVu " << ProjectInfo::versionString << " (" << getWrapperTypeDescription(wrapperType) << ")\n"
      << "Length:       " << clock(seconds) << " (" << (juce::int64) frames << " frames at " << juce::String(sr, 0) << " Hz)\n"
      << "Measured on:  the plug-in output (after trims, filters and the monitor choice)\n\n"
      << "Loudness (ITU-R BS.1770-5, EBU Tech 3341 and 3342)\n"
      << "  Integrated:        " << lufs(r.integrated, "LUFS") << "\n"
      << "  Loudness range:    " << (r.integrated <= -100.0f ? juce::String("no signal") : juce::String(r.range, 1) + " LU") << (r.range_stable ? "" : "  (under 60 s measured: not yet stable)") << "\n"
      << "  Max momentary:     " << lufs(r.max_momentary, "LUFS") << "\n"
      << "  Max short-term:    " << (r.max_short_term <= -100.0f ? juce::String("n/a (under 3 s measured)") : lufs(r.max_short_term, "LUFS")) << "\n"
      << "  True peak:         " << lufs(r.true_peak, "dBTP") << "\n"
      << "  Sample peak:       " << lufs(r.sample_peak, "dBFS") << "\n\n"
      << "Against the panel's settings\n"
      << "  Target " << juce::String(ls.target, 1) << " LUFS: integrated is "
      << (r.integrated <= -100.0f ? juce::String("n/a") : (offTarget >= 0 ? "+" : "") + juce::String(offTarget, 1) + " LU from it, " + (std::abs(offTarget) <= 1.0f ? "within" : "outside") + " +/-1 LU") << "\n"
      << "  Ceiling " << juce::String(ls.ceiling, 1) << " dBTP: true peak is "
      << (r.true_peak <= -100.0f ? juce::String("n/a") : juce::String(r.true_peak <= ls.ceiling ? "under it (pass)" : "over it (fail)")) << "\n\n"
      << "Per-second values: " << csv.getFileName() << "\n";
    juce::String c = "seconds,momentary_LUFS,short_term_LUFS,integrated_LUFS,true_peak_dBTP\n";
    for (int i = 0; i < rowCount; ++i) {
        c << juce::String(rows[(size_t) i * 5], 0);
        for (int k = 1; k < 5; ++k) c << "," << (rows[(size_t) i * 5 + (size_t) k] <= -100.0f ? juce::String() : juce::String(rows[(size_t) i * 5 + (size_t) k], 1));
        c << "\n";
    }
    // The visual report: the same numbers as cards and two charts, in one self-contained page.
    const bool silent = r.integrated <= -100.0f;
    const bool tpKnown = r.true_peak > -100.0f;
    const bool onTarget = !silent && std::abs(offTarget) <= 1.0f;
    const bool underCeiling = tpKnown && r.true_peak <= ls.ceiling;
    juce::String page;
    page << "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">"
         << "<title>" << esc(stem) << "</title><style>"
            "body{margin:0;background:#101416;color:#d8dee2;font:15px/1.5 -apple-system,Segoe UI,Helvetica,Arial,sans-serif}"
            "main{max-width:980px;margin:0 auto;padding:28px 16px 48px}h1{font-size:22px;margin:0 0 4px;color:#f2f6f3}h2{font-size:15px;margin:28px 0 8px;color:#aab4ba;font-weight:600}"
            ".meta{color:#8b969c;font-size:13px}.cards{display:grid;grid-template-columns:repeat(auto-fit,minmax(140px,1fr));gap:10px;margin-top:18px}"
            ".card{background:#161c20;border:1px solid #2a3238;border-radius:8px;padding:12px 14px}.card b{display:block;font-size:26px;font-weight:600;color:#f2f6f3;font-variant-numeric:tabular-nums}"
            ".card span{font-size:12px;color:#8b969c;text-transform:uppercase;letter-spacing:.04em}.card em{font-style:normal;font-size:12px}"
            ".ok{color:#46e16e}.bad{color:#fa4a3c}.na{color:#8b969c}svg{width:100%;height:auto;display:block;background:#0c1013;border:1px solid #2a3238;border-radius:8px}"
            ".ax{fill:#8b969c;font-size:11px}.key{display:flex;gap:16px;flex-wrap:wrap;font-size:13px;color:#aab4ba;margin:6px 0 0}.key i{display:inline-block;width:18px;height:3px;vertical-align:middle;margin-right:6px}"
            "p.note{font-size:13px;color:#8b969c}</style></head><body><main>"
         << "<h1>ArcVu bounce report</h1><div class=\"meta\">" << esc(started.toString(true, true, true, true)) << " &middot; " << esc(juce::PluginHostType().getHostDescription())
         << " &middot; ArcVu " << ProjectInfo::versionString << " &middot; " << clockShort(seconds) << " at " << juce::String(sr, 0) << " Hz</div>"
         << "<div class=\"cards\">"
         << "<div class=\"card\"><span>Integrated</span><b>" << (silent ? juce::String("&ndash;") : juce::String(r.integrated, 1)) << "</b><em class=\"" << (silent ? "na" : onTarget ? "ok" : "bad") << "\">"
         << (silent ? "no signal" : juce::String(offTarget >= 0 ? "+" : "") + juce::String(offTarget, 1) + " LU vs " + juce::String(ls.target, 1) + " LUFS target") << "</em></div>"
         << "<div class=\"card\"><span>True peak</span><b>" << (tpKnown ? juce::String(r.true_peak, 1) : juce::String("&ndash;")) << "</b><em class=\"" << (!tpKnown ? "na" : underCeiling ? "ok" : "bad") << "\">"
         << (!tpKnown ? "no signal" : juce::String(underCeiling ? "under " : "over ") + juce::String(ls.ceiling, 1) + " dBTP ceiling") << "</em></div>"
         << "<div class=\"card\"><span>Loudness range</span><b>" << (silent ? juce::String("&ndash;") : juce::String(r.range, 1)) << "</b><em class=\"na\">" << (r.range_stable ? "LU" : "LU, under 60 s: not stable") << "</em></div>"
         << "<div class=\"card\"><span>Max momentary</span><b>" << (r.max_momentary <= -100.0f ? juce::String("&ndash;") : juce::String(r.max_momentary, 1)) << "</b><em class=\"na\">LUFS</em></div>"
         << "<div class=\"card\"><span>Max short-term</span><b>" << (r.max_short_term <= -100.0f ? juce::String("&ndash;") : juce::String(r.max_short_term, 1)) << "</b><em class=\"na\">" << (r.max_short_term <= -100.0f ? "under 3 s measured" : "LUFS") << "</em></div>"
         << "<div class=\"card\"><span>Sample peak</span><b>" << (r.sample_peak <= -100.0f ? juce::String("&ndash;") : juce::String(r.sample_peak, 1)) << "</b><em class=\"na\">dBFS</em></div></div>";
    if (rowCount >= 2) {
        page << "<h2>Loudness over time (LUFS)</h2>"
             << chart(rows, rowCount, { { 1, "#6f8f9c", 1.0f, "Momentary" }, { 2, "#46e16e", 1.6f, "Short-term" }, { 3, "#4cd2ff", 2.2f, "Integrated" } }, ls.target, 1.0f, ("target " + juce::String(ls.target, 1) + " LUFS, band +/-1 LU").toRawUTF8(), true)
             << "<div class=\"key\"><span><i style=\"background:#6f8f9c\"></i>Momentary (0.4 s)</span><span><i style=\"background:#46e16e\"></i>Short-term (3 s)</span><span><i style=\"background:#4cd2ff\"></i>Integrated so far</span></div>"
             << "<h2>Highest true peak so far (dBTP)</h2>"
             << chart(rows, rowCount, { { 4, "#fa8a3c", 2.0f, "True peak" } }, ls.ceiling, 0.0f, ("ceiling " + juce::String(ls.ceiling, 1) + " dBTP").toRawUTF8(), false);
    }
    page << "<p class=\"note\">Measured on the plug-in output (after trims, filters and the monitor choice) to ITU-R BS.1770-5 and EBU Tech 3341 / 3342. The traces are sampled once a second; the figures in the cards are exact. Per-second values: "
         << esc(csv.getFileName()) << "</p></main></body></html>";
    const auto html = summary.withFileExtension(".html");
    t << "Visual report: " << html.getFileName() << "\n";
    if (!csv.replaceWithText(c) || !html.replaceWithText(page) || !summary.replaceWithText(t)) return {};
    return summary;
}

void ArcVuProcessor::setTheme(int theme) {
    themeIndex = juce::jlimit(0, 2, theme);
    arcvu_set_theme(meter, themeIndex);
}

void ArcVuProcessor::toggleSlowMode() {
    slowModeIndex = 1 - slowModeIndex;
    arcvu_set_slow_mode(meter, slowModeIndex);
}

void ArcVuProcessor::toggleCategory() {
    presetCategoryIndex = 1 - presetCategoryIndex;
}

juce::Array<juce::File> ArcVuProcessor::userPresetFiles() {
    std::vector<juce::File> sorted;
    for (const auto& file : userPresetDirectory().findChildFiles(juce::File::findFiles, false, "*.arcvu"))
        sorted.push_back(file);
    std::sort(sorted.begin(), sorted.end(), [](const juce::File& a, const juce::File& b) {
        return a.getFileName() < b.getFileName();
    });
    juce::Array<juce::File> files;
    for (const auto& file : sorted) files.add(file);
    return files;
}

juce::File ArcVuProcessor::userPresetDirectory() {
    return arcVuDefaultFile().getParentDirectory().getChildFile("UserPresets");
}

void ArcVuProcessor::savePresetFile(const juce::File& file) {
    pullSetup();
    file.getParentDirectory().createDirectory();
    file.replaceWithData(&setup, sizeof setup);
}

bool ArcVuProcessor::loadPresetFile(const juce::File& file) {
    juce::MemoryBlock block;
    if (file.getSize() < (juce::int64) sizeof(ArcVuSetup) || !file.loadFileAsData(block)) return false;
    setStateInformation(block.getData(), (int) block.getSize());
    return true;
}

void ArcVuProcessor::getStateInformation(juce::MemoryBlock& dest) {
    pullSetup();
    dest.reset();
    dest.append(&setup, sizeof(setup));
}

// A state blob is the raw setup struct, so a wrong file (an image, text) can be big enough. Reject values no setup can hold.
static bool plausibleSetup(const ArcVuSetup& s) {
    const auto finite = [](float v, float lo, float hi) { return std::isfinite(v) && v >= lo && v <= hi; };
    return s.kind >= 0 && s.kind <= 8 && s.face >= 0 && s.face <= 2 && s.peak_mode >= 0 && s.peak_mode <= 2
        && s.single >= 0 && s.single <= 1 && s.mid_side >= 0 && s.mid_side <= 1 && s.meters_only >= 0 && s.meters_only <= 1
        && s.monitor >= 0 && s.monitor <= 5 && s.vu_model >= 0 && s.vu_model <= 2 && s.dyn_type >= 0 && s.dyn_type <= 4
        && finite(s.reference_db, -36.0f, 0.0f) && finite(s.trim_l, -24.0f, 24.0f) && finite(s.trim_r, -24.0f, 24.0f)
        && finite(s.trim_m, -24.0f, 24.0f) && finite(s.trim_s, -24.0f, 24.0f) && finite(s.hpf_hz, 1.0f, 30000.0f)
        && finite(s.lpf_hz, 1.0f, 30000.0f) && finite(s.drag_sens, 0.1f, 10.0f);
}

void ArcVuProcessor::setStateInformation(const void* data, int size) {
    ArcVuSetup incoming;
    if (size >= (int) sizeof(ArcVuSetup)) std::memcpy(&incoming, data, sizeof(incoming));
    if (size >= (int) sizeof(ArcVuSetup) && plausibleSetup(incoming)) {
        std::memcpy(&setup, data, sizeof(setup));
        setup.panel = 0;
        setup.label[sizeof(setup.label) - 1] = 0;
        const int ref = setup.reference_db > -16.0f ? 2 : setup.reference_db > -19.0f ? 1 : 0;
        *referenceDb = ref;
        *peakModeParam = setup.peak_mode;
        noteSetup();
    }
}

juce::AudioProcessor* JUCE_CALLTYPE createPluginFilter() { return new ArcVuProcessor(); }
