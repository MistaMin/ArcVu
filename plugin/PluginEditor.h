#pragma once
#include <JuceHeader.h>
#include "PluginProcessor.h"
#include "ParameterPanel.h"

class ArcVuEditor : public juce::AudioProcessorEditor, private juce::Timer {
public:
    explicit ArcVuEditor(ArcVuProcessor&);
    void paint(juce::Graphics&) override;
    void resized() override;
    void showPanel(int group);
    ArcVuParameterPanel* parameterPanel() { return controls.get(); }
    void mouseDown(const juce::MouseEvent&) override;
    void mouseDrag(const juce::MouseEvent&) override;
    void mouseUp(const juce::MouseEvent&) override;

private:
    void timerCallback() override;
    ArcVuProcessor& arcvu;
    int dragTrim = -1;
    float dragOrigin = 0.0f;
    int lastHz = 30;
    int lastFace = -1;
    int lastStep = -1;
    int lastSkin = 0;
    float lastExtra = 0.0f;
    float userScale = 1.0f;
    bool zoomDirty = false;
    juce::ComponentBoundsConstrainer constrainer;
    juce::TextEditor labelBox;
    void commitLabel();
    void choosePresetFile(bool save);
    bool runPresetRow(int row);
    void showUserPresetMenu();
    std::unique_ptr<juce::FileChooser> presetChooser;
    std::unique_ptr<ArcVuParameterPanel> controls;
    juce::TooltipWindow tooltip { this, 700 };
};
