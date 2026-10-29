#pragma once
#include <JuceHeader.h>
#include "arcvu.h"
#include <vector>

struct ArcVuAAXExtensions : juce::AAXClientExtensions {
    juce::String getPageFileName() const override { return "ArcVu.xml"; }
};

class ArcVuProcessor : public juce::AudioProcessor, private juce::Timer {
public:
    ArcVuProcessor();
    ~ArcVuProcessor() override;

    void prepareToPlay(double sampleRate, int samplesPerBlock) override;
    void releaseResources() override { finishBounce(); }
    void processBlock(juce::AudioBuffer<float>&, juce::MidiBuffer&) override;
    bool isBusesLayoutSupported(const BusesLayout& layouts) const override;
    juce::AAXClientExtensions& getAAXClientExtensions() override { return aaxExtensions; }

    juce::AudioProcessorEditor* createEditor() override;
    bool hasEditor() const override { return true; }
    const juce::String getName() const override { return "ArcVu"; }
    bool acceptsMidi() const override { return false; }
    bool producesMidi() const override { return false; }
    double getTailLengthSeconds() const override { return 0.0; }
    int getNumPrograms() override { return 1; }
    int getCurrentProgram() override { return 0; }
    void setCurrentProgram(int) override {}
    const juce::String getProgramName(int) override { return {}; }
    void changeProgramName(int, const juce::String&) override {}
    void getStateInformation(juce::MemoryBlock& dest) override;
    void setStateInformation(const void* data, int size) override;

    juce::AudioParameterChoice& reference() { return *referenceDb; }
    juce::AudioParameterChoice& peakMode() { return *peakModeParam; }
    void resetPeaks() { arcvu_reset_peaks(meter); }
    void resetOvers() { arcvu_reset_overs(meter); }
    int overs() const { return arcvu_overs(meter); }
    void resetClip() { arcvu_reset_clip(meter); }
    void pullSetup();
    void readParams(ArcVuSetup& s) const;
    void noteSetup();
    void applyFactory(int which);
    void saveDefault();
    void loadDefault();
    void copySetup();
    void pasteSetup();
    void stepPreset(int delta);
    void toggleCategory();
    int presetCategory() const { return presetCategoryIndex; }
    static juce::Array<juce::File> userPresetFiles();
    void savePresetFile(const juce::File& file);
    bool loadPresetFile(const juce::File& file);
    static juce::File userPresetDirectory();
    static juce::String fitLabel(juce::String text);
    void toggleDragMode() { dragModeIndex = 1 - dragModeIndex; }
    void toggleSlowMode();
    int slowMode() const { return slowModeIndex; }
    void setTheme(int theme);
    void setSkin(int skin);
    void setPeakScale(int scale);
    int peakScale() const { return peakScaleIndex; }
    // Loudness panel settings: UI-only, kept in the global settings file like the skin, never in presets or host parameters.
    const ArcVuLoudSettings& loudness() const { return loud; }
    void setLoudness(const ArcVuLoudSettings& s);
    // Reads the settings back from the meter after a click on the panel's buttons.
    void pullLoudness();
    void resetLoudness() { arcvu_reset_loudness(meter); }
    ArcVuLoudness loudnessReadings() const;
    // Offline bounce log: while the host renders offline the output is measured, and when the render ends a text
    // summary and a per-second CSV are written to the Documents/ArcVu Bounce Logs folder.
    bool bounceLog() const { return bounceLogOn; }
    void setBounceLog(bool on);
    bool bounceActive() const { return bounceRunning.load(); }
    // Ends a running bounce and writes its log; returns the summary file, or an empty File when nothing was written.
    juce::File finishBounce();
    static juce::File bounceLogDirectory();
    float loudnessHeight(int face) const { return arcvu_loudness_height(face); }
    int loudnessClick(float x, float y);
    int skin() const { return skinIndex; }
    int theme() const { return themeIndex; }
    void setZoom(float zoom);
    float zoom() const { return zoomValue; }
    int dragMode() const { return dragModeIndex; }
    void saveGlobalSettings();
    // Changes one key of the global settings file and leaves every other line as it is.
    static void saveGlobalKey(const juce::String& key, const juce::String& value);
    void loadGlobalSettings();
    ArcVuSetup setup {};
    ArcVuLevels levels(int channel) const;
    int paint(ArcVuPaint* out, int cap) const;

private:
    ArcVuAAXExtensions aaxExtensions;
    ArcVuSetup initialSetup {};
    int factoryIndex = -1;
    int userIndex = -1;
    int presetCategoryIndex = 0;
    int dragModeIndex = 0;
    int slowModeIndex = 0;
    int themeIndex = 0;
    int skinIndex = 0;
    int peakScaleIndex = 0;
    bool bounceLogOn = true;
    double preparedRate = 48000.0;
    std::atomic<bool> bounceRunning { false };
    std::atomic<bool> lastBlockOffline { false };
    std::atomic<juce::uint32> lastBlockMs { 0 };
    std::atomic<juce::int64> bounceStartMs { 0 };
    void timerCallback() override;
    ArcVuLoudSettings loud { 0, -14.0f, -1.0f, 0, 0, 10.0f };
    float zoomValue = 0.0f;
    ArcVuSetup audioSetup {};
    std::vector<float> interleaved;  // reused every block so processBlock never allocates
    Meter* meter = nullptr;
    juce::AudioParameterChoice* referenceDb = nullptr;
    juce::AudioParameterChoice* peakModeParam = nullptr;
    juce::AudioParameterChoice* kindP = nullptr;
    juce::AudioParameterChoice* faceP = nullptr;
    juce::AudioParameterBool* singleP = nullptr;
    juce::AudioParameterBool* midP = nullptr;
    juce::AudioParameterBool* aesP = nullptr;
    juce::AudioParameterFloat* rmsP = nullptr;
    juce::AudioParameterFloat* clipP = nullptr;
    juce::AudioParameterFloat* trimP[4] {};
    juce::AudioParameterBool* polP[2] {};
    juce::AudioParameterBool* muteP[2] {};
    juce::AudioParameterChoice* monitorP = nullptr;
    juce::AudioParameterFloat* hpfHzP = nullptr;
    juce::AudioParameterInt* hpfPolesP = nullptr;
    juce::AudioParameterFloat* lpfHzP = nullptr;
    juce::AudioParameterInt* lpfPolesP = nullptr;
    juce::AudioParameterFloat* monoHzP = nullptr;
    juce::AudioParameterFloat* monoAmtP = nullptr;
    juce::AudioParameterFloat* dynHzP = nullptr;
    juce::AudioParameterFloat* dynDepthP = nullptr;
    juce::AudioParameterBool* dynOnP = nullptr;
    juce::AudioParameterInt* dynTypeP = nullptr;
    juce::AudioParameterBool* dynListenP = nullptr;
    juce::AudioParameterInt* vuModelP = nullptr;
    juce::AudioParameterFloat* riseP = nullptr;
    juce::AudioParameterFloat* fallP = nullptr;
    juce::AudioParameterFloat* overP = nullptr;
    juce::AudioParameterFloat* peakHoldP = nullptr;
    juce::AudioParameterFloat* valueHoldP = nullptr;
    juce::AudioParameterFloat* clipHoldP = nullptr;
    juce::AudioParameterFloat* peakThreshP = nullptr;
    juce::AudioParameterFloat* redP = nullptr;
    juce::AudioParameterFloat* comfortP = nullptr;
    juce::AudioParameterBool* analogP = nullptr;
    juce::AudioParameterInt* weightP = nullptr;
    juce::AudioParameterBool* forceP = nullptr;
    juce::AudioParameterBool* linkP = nullptr;
    juce::AudioParameterBool* inPlaceP = nullptr;
    juce::AudioParameterBool* showValuesP = nullptr;
    juce::AudioParameterBool* showHoldP = nullptr;
    juce::AudioParameterBool* showToolsP = nullptr;
    juce::AudioParameterBool* showLabelP = nullptr;
    juce::AudioParameterBool* metersOnlyP = nullptr;
    juce::AudioParameterInt* guiScaleP = nullptr;
    juce::AudioParameterFloat* dragP = nullptr;
    juce::AudioParameterInt* refreshP = nullptr;
    juce::AudioParameterBool* bypassP = nullptr;
    juce::AudioParameterBool* tipsP = nullptr;
    juce::AudioParameterBool* topP = nullptr;
    mutable juce::SpinLock levelLock;
    ArcVuLevels cached[2] {};
};
