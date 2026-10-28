#pragma once
#include "PluginProcessor.h"

// The visible controls bind directly to the existing host registry. No second
// list of parameter ranges/defaults or setup-field offsets is needed here.
// Round knob with a lit value arc, used by the Tools dropdown.
struct ArcVuKnobLook : juce::LookAndFeel_V4 {
    juce::Colour accent { 0xff3dcc4a };
    void drawRotarySlider(juce::Graphics& g, int x, int y, int w, int h, float pos, float start, float end, juce::Slider& slider) override {
        auto area = juce::Rectangle<float>((float) x, (float) y, (float) w, (float) h).reduced(4.0f);
        const float d = juce::jmin(area.getWidth(), area.getHeight());
        auto knob = juce::Rectangle<float>(d, d).withCentre(area.getCentre());
        const float angle = start + pos * (end - start);
        const auto centre = knob.getCentre();
        const float arcR = d * 0.5f;
        juce::Path track, value;
        track.addCentredArc(centre.x, centre.y, arcR, arcR, 0.0f, start, end, true);
        value.addCentredArc(centre.x, centre.y, arcR, arcR, 0.0f, start, angle, true);
        g.setColour(juce::Colour(0xff070a0c));
        g.strokePath(track, juce::PathStrokeType(4.0f, juce::PathStrokeType::curved, juce::PathStrokeType::rounded));
        if (slider.isEnabled() && pos > 0.0f) {
            for (float spread : { 9.0f, 6.0f })
                { g.setColour(accent.withAlpha(spread > 7.0f ? 0.10f : 0.20f)); g.strokePath(value, juce::PathStrokeType(spread, juce::PathStrokeType::curved, juce::PathStrokeType::rounded)); }
            g.setColour(accent);
            g.strokePath(value, juce::PathStrokeType(3.0f, juce::PathStrokeType::curved, juce::PathStrokeType::rounded));
        }
        auto body = knob.reduced(7.0f);
        g.setGradientFill(juce::ColourGradient(juce::Colour(0xff5b676e), body.getX(), body.getY(), juce::Colour(0xff161c20), body.getX(), body.getBottom(), false));
        g.fillEllipse(body);
        g.setColour(juce::Colour(0xff9aa8af)); g.drawEllipse(body, 1.0f);
        g.setGradientFill(juce::ColourGradient(juce::Colour(0xff2b343a), body.getX(), body.getY(), juce::Colour(0xff0d1114), body.getX(), body.getBottom(), false));
        g.fillEllipse(body.reduced(3.5f));
        juce::Path pointer;
        pointer.addRoundedRectangle(-1.4f, -body.getHeight() * 0.5f + 3.0f, 2.8f, body.getHeight() * 0.28f, 1.2f);
        g.setColour(juce::Colours::white);
        g.fillPath(pointer, juce::AffineTransform::rotation(angle).translated(centre.x, centre.y));
    }
};

class ArcVuParameterPanel : public juce::Component {
public:
    struct Row : juce::Component {
        juce::Label name;
        juce::Slider slider;
        juce::ComboBox choice;
        juce::TextButton monitorLR { "LR" };
        juce::ToggleButton toggle;
        std::unique_ptr<juce::SliderParameterAttachment> sa;
        std::unique_ptr<juce::ComboBoxParameterAttachment> ca;
        std::unique_ptr<juce::ButtonParameterAttachment> ba;
        juce::RangedAudioParameter* parameterPtr = nullptr;
        juce::RangedAudioParameter& parameter;
        juce::String customId;
        juce::String id() const { return parameterPtr ? parameterPtr->paramID : customId; }
        bool knob = false;
        bool tools = false;
        // A choice that is not a host parameter (skin, colour): reads and writes through callbacks.
        Row(const juce::String& label, const juce::StringArray& items, int current, std::function<void(int)> set, juce::RangedAudioParameter& unused)
            : parameter(unused), customId(label) {
            setName(label);
            name.setText(label, juce::dontSendNotification);
            name.setColour(juce::Label::textColourId, juce::Colour(0xffd2dbdf));
            name.setFont(juce::Font(juce::FontOptions(12.0f)));
            addAndMakeVisible(name);
            choice.addItemList(items, 1);
            choice.setSelectedItemIndex(current, juce::dontSendNotification);
            choice.onChange = [this, set] { set(choice.getSelectedItemIndex()); };
            addAndMakeVisible(choice);
        }
        // A switch that is not a host parameter (the loudness panel).
        Row(const juce::String& label, bool current, std::function<void(bool)> set, juce::RangedAudioParameter& unused)
            : parameter(unused), customId(label) {
            setName(label);
            name.setText(label, juce::dontSendNotification);
            name.setColour(juce::Label::textColourId, juce::Colour(0xffd2dbdf));
            name.setFont(juce::Font(juce::FontOptions(12.0f)));
            addAndMakeVisible(name);
            toggle.setButtonText("On");
            toggle.setToggleState(current, juce::dontSendNotification);
            toggle.onClick = [this, set] { set(toggle.getToggleState()); };
            addAndMakeVisible(toggle);
        }
        // A number that is not a host parameter (loudness target, ceiling, window): drag or type it.
        Row(const juce::String& label, double lo, double hi, double step, double current, const juce::String& suffix, std::function<void(double)> set, juce::RangedAudioParameter& unused)
            : parameter(unused), customId(label) {
            setName(label);
            name.setText(label, juce::dontSendNotification);
            name.setColour(juce::Label::textColourId, juce::Colour(0xffd2dbdf));
            name.setFont(juce::Font(juce::FontOptions(12.0f)));
            addAndMakeVisible(name);
            slider.setSliderStyle(juce::Slider::LinearHorizontal);
            slider.setTextBoxStyle(juce::Slider::TextBoxRight, false, 72, 24);
            slider.setRange(lo, hi, step);
            slider.setNumDecimalPlacesToDisplay(1);
            slider.setTextValueSuffix(suffix);
            slider.setValue(current, juce::dontSendNotification);
            slider.onValueChange = [this, set] { set(slider.getValue()); };
            addAndMakeVisible(slider);
        }
        Row(juce::RangedAudioParameter& p, bool asKnob = false, bool inTools = false) : parameterPtr(&p), parameter(p), knob(asKnob), tools(inTools) {
            setName(p.paramID);
            name.setText(p.getName(64), juce::dontSendNotification);
            name.setColour(juce::Label::textColourId, juce::Colour(0xffd2dbdf));
            name.setFont(juce::Font(juce::FontOptions(12.0f)));
            addAndMakeVisible(name);
            if (dynamic_cast<juce::AudioParameterBool*>(&p)) {
                addAndMakeVisible(toggle);
                toggle.setButtonText(tools ? p.getName(64) : "On");
                if (tools) name.setVisible(false);
                ba = std::make_unique<juce::ButtonParameterAttachment>(p, toggle);
            } else {
                auto strings = p.getAllValueStrings();
                if (p.paramID == "weight") strings = {"Off", "A", "B", "C", "D", "K"};
                if (p.paramID == "vuModel") strings = {"Ideal 300 ms", "Model 2", "Custom"};
                if (p.paramID == "dynType") strings = {"Off", "High-pass", "Low-pass", "Narrow band", "Wide band"};
                if (p.paramID == "guiScale") strings = {"100%", "125%", "150%"};
                if (p.paramID == "hpfPoles" || p.paramID == "lpfPoles") strings = {"Off", "6 dB/oct", "12 dB/oct", "18 dB/oct", "24 dB/oct"};
                const bool poles = p.paramID == "hpfPoles" || p.paramID == "lpfPoles";
                if (!strings.isEmpty() && !(knob && poles)) {
                    if (tools) name.setText("Type", juce::dontSendNotification);
                    choice.addItemList(strings, 1);
                    addAndMakeVisible(choice);
                    ca = std::make_unique<juce::ComboBoxParameterAttachment>(p, choice);
                    if (p.paramID == "monitor") {
                        addAndMakeVisible(monitorLR);
                        monitorLR.setTooltip("Return monitoring to left and right");
                        monitorLR.onClick = [this] {
                            parameter.setValueNotifyingHost(parameter.convertTo0to1(0.0f));
                        };
                    }
                } else {
                    if (knob) {
                        slider.setSliderStyle(juce::Slider::RotaryHorizontalVerticalDrag);
                        slider.setTextBoxStyle(juce::Slider::TextBoxBelow, false, 64, 16);
                        slider.setRotaryParameters(juce::MathConstants<float>::pi * 1.2f, juce::MathConstants<float>::pi * 2.8f, true);
                        name.setJustificationType(juce::Justification::centred);
                        static const std::map<juce::String, juce::String> shortNames {
                            { "hpfHz", "Freq" }, { "hpfPoles", "Slope" }, { "lpfHz", "Freq" }, { "lpfPoles", "Slope" },
                            { "monoHz", "Freq" }, { "monoAmt", "Amount" }, { "dynHz", "Freq" }, { "dynDepth", "Depth" } };
                        if (auto it = shortNames.find(p.paramID); it != shortNames.end()) name.setText(it->second, juce::dontSendNotification);
                    } else {
                        slider.setSliderStyle(juce::Slider::LinearHorizontal);
                        slider.setTextBoxStyle(juce::Slider::TextBoxRight, false, 72, 24);
                    }
                    slider.setNumDecimalPlacesToDisplay(dynamic_cast<juce::AudioParameterInt*>(&p) ? 0 : 2);
                    if (p.paramID.endsWith("Hz")) slider.setTextValueSuffix(" Hz");
                    else if (p.paramID.endsWith("Ms")) slider.setTextValueSuffix(" ms");
                    else if (p.paramID.endsWith("Hold")) slider.setTextValueSuffix(" s");
                    slider.setTooltip(p.getName(64) + (p.paramID.endsWith("Hold") ? ": negative values hold indefinitely" : ""));
                    addAndMakeVisible(slider);
                    sa = std::make_unique<juce::SliderParameterAttachment>(p, slider);
                    if (knob) {
                        if (poles) slider.textFromValueFunction = [](double v) { const int n = (int) std::lround(v); return n == 0 ? juce::String("Off") : juce::String(n * 6) + " dB"; };
                        if (p.paramID.endsWith("Hz")) {
                            slider.textFromValueFunction = [](double v) { return v >= 1000.0 ? juce::String(v / 1000.0, 1) : juce::String(juce::roundToInt(v)); };
                            slider.valueFromTextFunction = [](const juce::String& t) { const auto d = t.getDoubleValue(); return t.containsIgnoreCase("k") || (d > 0.0 && d < 25.0 && t.contains(".")) ? d * 1000.0 : d; };
                            slider.onValueChange = [this] { name.setText(slider.getValue() >= 1000.0 ? "Freq kHz" : "Freq Hz", juce::dontSendNotification); };
                        } else if (p.paramID.endsWith("Amt") || p.paramID.endsWith("Depth")) {
                            slider.textFromValueFunction = [](double v) { return juce::String(juce::roundToInt(v * 100.0)) + "%"; };
                            slider.valueFromTextFunction = [](const juce::String& t) { return t.getDoubleValue() / 100.0; };
                        }
                        if (poles) slider.textFromValueFunction = [](double v) { const int n = (int) std::lround(v); return n == 0 ? juce::String("Off") : juce::String(n * 6) + " dB"; };
                        slider.setTextValueSuffix("");
                        slider.updateText();
                        if (slider.onValueChange) slider.onValueChange();
                    }
                }
            }
        }
        bool p_isChoiceInTools() const { return id() == "dynType"; }
        bool sliderStyleIsRotary() const { return slider.getSliderStyle() == juce::Slider::RotaryHorizontalVerticalDrag; }
        void resized() override {
            auto r = getLocalBounds().reduced(6, 3);
            if (knob && sliderStyleIsRotary()) {
                name.setBounds(r.removeFromTop(14));
                slider.setBounds(r);
                return;
            }
            name.setBounds(r.removeFromLeft(tools ? (p_isChoiceInTools() ? 40 : 0) : 145));
            slider.setBounds(r);
            if (id() == "monitor") {
                monitorLR.setBounds(r.removeFromRight(42));
                choice.setBounds(r);
            } else choice.setBounds(r);
            toggle.setBounds(r);
        }
    };
    ArcVuParameterPanel(ArcVuProcessor& p, int group) : toolsMode(group == 6) {
        const juce::Colour accents[] = { juce::Colour(0xff3dcc4a), juce::Colour(0xffffa21f), juce::Colour(0xff2ad4ff) };
        knobLook.accent = accents[juce::jlimit(0, 2, p.theme())];
        const juce::StringArray ids = group == 1 ? juce::StringArray{"kind"}
            : group == 2 ? juce::StringArray{"face", "single", "guiScale", "metersOnly", "showValues", "showHold", "showTools", "showLabel"}
            : group == 3 ? juce::StringArray{"reference"}
            : group == 4 ? juce::StringArray{"peak", "rmsMs", "aes17", "vuModel", "riseMs", "fallMs", "overshoot", "peakHold", "valueHold", "clipHold", "clipDb", "peakThresh", "redDb", "comfort", "analogRms", "weight"}
            : group == 5 ? juce::StringArray{"midSide", "trimL", "trimR", "trimM", "trimS", "trimLink", "polL", "polR", "muteL", "muteR", "monitor", "forceMono", "inPlace"}
            : group == 6 ? juce::StringArray{"hpfHz", "hpfPoles", "lpfHz", "lpfPoles", "monoHz", "monoAmt", "dynHz", "dynDepth", "dynOn", "dynType", "dynListen"}
            : juce::StringArray{"dragSens", "refresh", "bypassHidden", "tooltips", "alwaysOnTop"};
        for (auto& id : ids) {
            for (auto* param : p.getParameters()) {
                if (auto* rp = dynamic_cast<juce::RangedAudioParameter*>(param); rp && rp->paramID == id) {
                    auto row = std::make_unique<Row>(*rp, toolsMode && !dynamic_cast<juce::AudioParameterBool*>(rp) && rp->paramID != "dynType", toolsMode);
                    row->slider.setLookAndFeel(&knobLook);
                    addChildComponent(*row);
                    rows.push_back(std::move(row));
                }
            }
        }
        if (group == 2) {
            auto* anchor = dynamic_cast<juce::RangedAudioParameter*>(p.getParameters()[0]);
            auto skin = std::make_unique<Row>("Skin", juce::StringArray{"Modern", "Classic"}, p.skin(), [&p](int i) { p.setSkin(i); ArcVuProcessor::saveGlobalKey("skin", juce::String(i)); }, *anchor);
            auto colour = std::make_unique<Row>("Colour", juce::StringArray{"Graphite / Ivory", "Amber", "Ice"}, p.theme(), [&p](int i) { p.setTheme(i); ArcVuProcessor::saveGlobalKey("theme", juce::String(i)); }, *anchor);
            auto peakScale = std::make_unique<Row>("Peak scale", juce::StringArray{"Meter scale", "dBFS"}, p.peakScale(), [&p](int i) { p.setPeakScale(i); ArcVuProcessor::saveGlobalKey("peak_scale", juce::String(i)); }, *anchor);
            // Loudness panel: switch, target, true-peak ceiling, EBU scale, unit and sliding window. Saved with the other global settings.
            const auto loudSet = [&p](auto change) { auto s = p.loudness(); change(s); p.setLoudness(s); };
            auto loudOn = std::make_unique<Row>("Loudness panel", p.loudness().on != 0, [loudSet](bool on) { loudSet([on](ArcVuLoudSettings& s) { s.on = on ? 1 : 0; }); }, *anchor);
            auto loudTarget = std::make_unique<Row>("Loudness target", -40.0, -5.0, 0.5, p.loudness().target, " LUFS", [loudSet](double v) { loudSet([v](ArcVuLoudSettings& s) { s.target = (float) v; }); }, *anchor);
            auto loudCeiling = std::make_unique<Row>("True-peak ceiling", -12.0, 0.0, 0.5, p.loudness().ceiling, " dBTP", [loudSet](double v) { loudSet([v](ArcVuLoudSettings& s) { s.ceiling = (float) v; }); }, *anchor);
            auto loudWindow = std::make_unique<Row>("Sliding window", 1.0, 30.0, 1.0, p.loudness().window, " s", [loudSet](double v) { loudSet([v](ArcVuLoudSettings& s) { s.window = (float) v; }); }, *anchor);
            auto bounceLog = std::make_unique<Row>("Bounce log", p.bounceLog(), [&p](bool on) { p.setBounceLog(on); }, *anchor);
            auto loudScale = std::make_unique<Row>("Loudness scale", juce::StringArray{"EBU +9", "EBU +18"}, p.loudness().scale, [loudSet](int i) { loudSet([i](ArcVuLoudSettings& s) { s.scale = i; }); }, *anchor);
            auto loudUnit = std::make_unique<Row>("Loudness unit", juce::StringArray{"LUFS (absolute)", "LU (to target)"}, p.loudness().unit, [loudSet](int i) { loudSet([i](ArcVuLoudSettings& s) { s.unit = i; }); }, *anchor);
            for (auto* r : { skin.get(), colour.get(), peakScale.get(), loudOn.get(), loudTarget.get(), loudCeiling.get(), loudWindow.get(), loudScale.get(), loudUnit.get(), bounceLog.get() }) addChildComponent(*r);
            // The loudness rows go last, so the look choices and the face choices stay on the first page of the Display menu.
            rows.push_back(std::move(loudOn)); rows.push_back(std::move(loudTarget)); rows.push_back(std::move(loudCeiling));
            rows.push_back(std::move(loudWindow)); rows.push_back(std::move(loudScale)); rows.push_back(std::move(loudUnit)); rows.push_back(std::move(bounceLog));
            rows.insert(rows.begin(), std::move(peakScale));
            rows.insert(rows.begin(), std::move(colour));
            rows.insert(rows.begin(), std::move(skin));
        }
        const char* titles[] = {"", "Meter type", "Display", "Reference", "Meter settings", "Channel tools", "Processing tools", "Preferences"};
        title.setText(titles[juce::jlimit(1, 7, group)], juce::dontSendNotification);
        title.setColour(juce::Label::textColourId, juce::Colour(0xffedf2f4));
        addAndMakeVisible(title);
        for (auto* b : {&previous, &next, &close}) addAndMakeVisible(b);
        addAndMakeVisible(pages);
        if (toolsMode) { previous.setVisible(false); next.setVisible(false); pages.setVisible(false); }
        previous.onClick = [this] { setPage(page - 1); };
        next.onClick = [this] { setPage(page + 1); };
        close.onClick = [this] { if (onClose) onClose(); };
    }
    void paint(juce::Graphics& g) override {
        g.fillAll(juce::Colour(0xff151b1e));
        g.setColour(juce::Colour(0xff77858c)); g.drawRect(getLocalBounds());
        g.setColour(juce::Colour(0xff323e44));
        g.drawHorizontalLine(31, 1.0f, (float) getWidth() - 1.0f);
        if (toolsMode && rows.size() >= 8) {
            g.setColour(juce::Colour(0xff8b9aa1));
            g.setFont(juce::Font(juce::FontOptions(10.5f)));
            const char* captions[] = { "HIGH-PASS", "LOW-PASS", "MONO MAKER", "DYNAMIC" };
            for (int group = 0; group < 4; ++group) {
                auto b = rows[(size_t) group * 2]->getBounds().getUnion(rows[(size_t) group * 2 + 1]->getBounds());
                g.setColour(juce::Colour(0xff0a0e10)); g.fillRoundedRectangle(b.toFloat().expanded(0.0f, 0.0f).withTrimmedTop(-2.0f), 6.0f);
                g.setColour(juce::Colour(0xff323e44)); g.drawRoundedRectangle(b.toFloat(), 6.0f, 1.0f);
                g.setColour(juce::Colour(0xff8b9aa1));
                g.drawText(captions[group], b.getX(), b.getY() - 14, b.getWidth(), 12, juce::Justification::centred);
            }
        }
    }
    ~ArcVuParameterPanel() override { for (auto& row : rows) row->slider.setLookAndFeel(nullptr); }
    // Knob grid for the Tools dropdown: four groups of two knobs, then the dynamic-filter switches.
    void layoutTools() {
        const int cellW = 62, cellH = 92, groupGap = 10;
        const int perRow = getWidth() >= 540 ? 4 : 2;
        const int groupW = cellW * 2;
        const int left = (getWidth() - (perRow * groupW + (perRow - 1) * groupGap)) / 2;
        for (int i = 0; i < 8 && i < (int) rows.size(); ++i) {
            const int group = i / 2, inGroup = i % 2;
            const int col = group % perRow, line = group / perRow;
            rows[(size_t) i]->setVisible(true);
            rows[(size_t) i]->setBounds(left + col * (groupW + groupGap) + inGroup * cellW, 52 + line * (cellH + 18), cellW, cellH);
        }
        const int lines = (4 + perRow - 1) / perRow;
        const int y = 52 + lines * (cellH + 18) + 2;
        int x = 12, line = 0;
        for (size_t i = 8; i < rows.size(); ++i) {
            const int w = stripWidth(rows[i]->id());
            if (x + w > getWidth() - 8) { x = 12; ++line; }
            rows[i]->setVisible(true);
            rows[i]->setBounds(x, y + line * 32, w, 28);
            x += w + 6;
        }
    }
    static int stripWidth(const juce::String& id) { return id == "dynType" ? 190 : id == "dynListen" ? 130 : 120; }
    int toolsHeight() const {
        int x = 12, lines = 1;
        for (auto* id : { "dynOn", "dynType", "dynListen" }) {
            const int w = stripWidth(id);
            if (x + w > getWidth() - 8) { x = 12; ++lines; }
            x += w + 6;
        }
        return 52 + (getWidth() >= 540 ? 1 : 2) * 110 + 2 + lines * 32 + 8;
    }
    void resized() override {
        title.setBounds(12, 4, getWidth() - 120, 24);
        close.setBounds(getWidth() - 88, 5, 76, 22);
        if (toolsMode) { layoutTools(); return; }
        perColumn = juce::jmax(1, (getHeight() - 70) / 32);
        const int columns = getWidth() >= 650 ? 2 : 1;
        pageSize = perColumn * columns;
        page = juce::jlimit(0, pageCount() - 1, page);
        for (int i = 0; i < (int) rows.size(); ++i) {
            int slot = i - page * pageSize;
            rows[(size_t)i]->setVisible(slot >= 0 && slot < pageSize);
            if (slot >= 0 && slot < pageSize)
                rows[(size_t)i]->setBounds(8 + slot / perColumn * (getWidth() - 16) / columns, 36 + slot % perColumn * 32, (getWidth() - 16) / columns, 30);
        }
        previous.setBounds(12, getHeight() - 29, 90, 22);
        next.setBounds(getWidth() - 102, getHeight() - 29, 90, 22);
        pages.setBounds(110, getHeight() - 29, getWidth() - 220, 22);
        pages.setJustificationType(juce::Justification::centred);
        pages.setText("Page " + juce::String(page + 1) + " / " + juce::String(pageCount()) + "   •   Drag or type a value", juce::dontSendNotification);
        previous.setEnabled(page > 0); next.setEnabled(page + 1 < pageCount());
    }
    int pageCount() const { return juce::jmax(1, ((int) rows.size() + pageSize - 1) / pageSize); }
    void setPage(int n) { page = juce::jlimit(0, pageCount() - 1, n); resized(); }
    std::function<void()> onClose;
    std::vector<std::unique_ptr<Row>> rows;
private:
    bool toolsMode = false;
    ArcVuKnobLook knobLook;
    juce::Label title, pages;
    juce::TextButton previous{"Previous"}, next{"Next"}, close{"Close"};
    int page = 0, perColumn = 1, pageSize = 2;
};
