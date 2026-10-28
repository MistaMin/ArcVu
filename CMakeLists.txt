cmake_minimum_required(VERSION 3.22)
project(ArcVu VERSION 1.0.0)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# macOS release builds: -DARCVU_UNIVERSAL=ON builds arm64 + x86_64 (the Rust library too).
option(ARCVU_UNIVERSAL "Build arm64 + x86_64 on macOS" OFF)
if(APPLE AND ARCVU_UNIVERSAL)
    set(CMAKE_OSX_ARCHITECTURES "arm64;x86_64" CACHE STRING "" FORCE)
endif()
set(CMAKE_OSX_DEPLOYMENT_TARGET "11.0" CACHE STRING "Minimum macOS version")
if(MSVC)
    add_compile_definitions(_USE_MATH_DEFINES NOMINMAX)
endif()

# JUCE: a local checkout next to this repository, or the public MistaMin/audiogridder checkout layout.
set(JUCE_DIR "${CMAKE_SOURCE_DIR}/../audiogridder/JUCE" CACHE PATH "JUCE checkout")
add_subdirectory(${JUCE_DIR} ${CMAKE_BINARY_DIR}/juce EXCLUDE_FROM_ALL)

# The detector and the drawing list are a Rust static library. Universal macOS builds both slices and joins them.
find_program(ARCVU_CARGO cargo HINTS "$ENV{HOME}/.cargo/bin" "/opt/homebrew/bin" "/usr/local/bin")
if(NOT ARCVU_CARGO)
    message(FATAL_ERROR "cargo (Rust) is required to build the ArcVu detector")
endif()
if(WIN32)
    set(ARCVU_LIB ${CMAKE_BINARY_DIR}/arcvu_dsp.lib)
    set(ARCVU_CARGO_LIB arcvu_dsp.lib)
else()
    set(ARCVU_LIB ${CMAKE_BINARY_DIR}/libarcvu_dsp.a)
    set(ARCVU_CARGO_LIB libarcvu_dsp.a)
endif()
set(ARCVU_RUST_SOURCES ${CMAKE_SOURCE_DIR}/dsp/src/lib.rs ${CMAKE_SOURCE_DIR}/dsp/src/extra.rs ${CMAKE_SOURCE_DIR}/dsp/src/loudness.rs ${CMAKE_SOURCE_DIR}/dsp/src/loudness_panel.rs ${CMAKE_SOURCE_DIR}/dsp/Cargo.toml)
if(APPLE AND ARCVU_UNIVERSAL)
    add_custom_command(
        OUTPUT ${ARCVU_LIB}
        COMMAND ${ARCVU_CARGO} build --release --target aarch64-apple-darwin --manifest-path ${CMAKE_SOURCE_DIR}/dsp/Cargo.toml
                --target-dir ${CMAKE_BINARY_DIR}/dsp-target
        COMMAND ${ARCVU_CARGO} build --release --target x86_64-apple-darwin --manifest-path ${CMAKE_SOURCE_DIR}/dsp/Cargo.toml
                --target-dir ${CMAKE_BINARY_DIR}/dsp-target
        COMMAND lipo -create -output ${ARCVU_LIB}
                ${CMAKE_BINARY_DIR}/dsp-target/aarch64-apple-darwin/release/${ARCVU_CARGO_LIB}
                ${CMAKE_BINARY_DIR}/dsp-target/x86_64-apple-darwin/release/${ARCVU_CARGO_LIB}
        DEPENDS ${ARCVU_RUST_SOURCES}
        COMMENT "Building ArcVu Rust detector (arm64 + x86_64)")
else()
    add_custom_command(
        OUTPUT ${ARCVU_LIB}
        COMMAND ${ARCVU_CARGO} build --release --manifest-path ${CMAKE_SOURCE_DIR}/dsp/Cargo.toml
                --target-dir ${CMAKE_BINARY_DIR}/dsp-target
        COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_BINARY_DIR}/dsp-target/release/${ARCVU_CARGO_LIB} ${ARCVU_LIB}
        DEPENDS ${ARCVU_RUST_SOURCES}
        COMMENT "Building ArcVu Rust detector")
endif()
add_custom_target(arcvu_dsp DEPENDS ${ARCVU_LIB})

# VST3, LV2 and Standalone everywhere, AU on macOS, CLAP through clap-juce-extensions. AAX is built when the
# Avid AAX SDK is present (it is not distributed with the source).
set(ARCVU_FORMATS VST3 LV2 Standalone)
if(APPLE)
    list(APPEND ARCVU_FORMATS AU)
endif()
set(ARCVU_AAX_SDK_PATH "${CMAKE_CURRENT_SOURCE_DIR}/../aax-sdk-2-8-1" CACHE PATH "Avid AAX SDK root (optional)")
option(ARCVU_BUILD_AAX "Build the AAX format when the SDK is available" ON)
if(ARCVU_BUILD_AAX AND (APPLE OR WIN32) AND EXISTS "${ARCVU_AAX_SDK_PATH}/Interfaces/AAX.h")
    juce_set_aax_sdk_path("${ARCVU_AAX_SDK_PATH}")
    list(APPEND ARCVU_FORMATS AAX)
else()
    message(STATUS "AAX SDK not used")
endif()
message(STATUS "ArcVu formats: ${ARCVU_FORMATS}")

juce_add_plugin(ArcVu
    COMPANY_NAME "OpenGrid"
    BUNDLE_ID com.opengrid.arcvu
    ICON_BIG "${CMAKE_CURRENT_SOURCE_DIR}/Resources/icon-1024.png"
    ICON_SMALL "${CMAKE_CURRENT_SOURCE_DIR}/Resources/icon-128.png"
    IS_SYNTH FALSE
    NEEDS_MIDI_INPUT FALSE
    NEEDS_MIDI_OUTPUT FALSE
    IS_MIDI_EFFECT FALSE
    EDITOR_WANTS_KEYBOARD_FOCUS FALSE
    COPY_PLUGIN_AFTER_BUILD FALSE
    PLUGIN_MANUFACTURER_CODE OpGr
    PLUGIN_CODE ArcV
    FORMATS ${ARCVU_FORMATS}
    DESCRIPTION "Digital analogue VU, RMS, K and PPM meter"
    VST3_CATEGORIES Fx Analyzer
    AU_MAIN_TYPE kAudioUnitType_Effect
    LV2URI "urn:opengrid:arcvu"
    MICROPHONE_PERMISSION_ENABLED TRUE
    MICROPHONE_PERMISSION_TEXT "ArcVu uses audio input to meter live sound in the standalone application."
    HARDENED_RUNTIME_ENABLED TRUE
    HARDENED_RUNTIME_OPTIONS com.apple.security.device.audio-input
    AAX_IDENTIFIER com.opengrid.arcvu
    AAX_CATEGORY AAX_ePlugInCategory_SoundField
    PRODUCT_NAME "ArcVu")

option(ARCVU_BUILD_CLAP "Build the CLAP plugin" ON)
if(ARCVU_BUILD_CLAP)
    set(ARCVU_CLAP_EXT_SOURCE "${CMAKE_SOURCE_DIR}/../clap-juce-extensions" CACHE PATH "An existing clap-juce-extensions checkout")
    include(FetchContent)
    if(EXISTS "${ARCVU_CLAP_EXT_SOURCE}/CMakeLists.txt")
        set(FETCHCONTENT_SOURCE_DIR_CLAP-JUCE-EXTENSIONS "${ARCVU_CLAP_EXT_SOURCE}")
    endif()
    FetchContent_Declare(
        clap-juce-extensions
        GIT_REPOSITORY https://github.com/free-audio/clap-juce-extensions.git
        # Pinned commit on main (the last tag predates JUCE 8).
        GIT_TAG        55525c9858d4b25687be7759a5e0f70eccef218e
        GIT_SUBMODULES_RECURSE TRUE)
    FetchContent_MakeAvailable(clap-juce-extensions)
endif()

juce_generate_juce_header(ArcVu)
target_sources(ArcVu PRIVATE
    plugin/PluginProcessor.cpp
    plugin/PluginEditor.cpp)
target_include_directories(ArcVu PRIVATE plugin)
target_compile_definitions(ArcVu PUBLIC
    JUCE_WEB_BROWSER=0
    JUCE_USE_CURL=0
    JUCE_VST3_CAN_REPLACE_VST2=0)
target_link_libraries(ArcVu PRIVATE
    ${ARCVU_LIB}
    juce::juce_audio_utils
    juce::juce_recommended_config_flags
    juce::juce_recommended_warning_flags)
if(WIN32)
    target_link_libraries(ArcVu PRIVATE ws2_32 userenv ntdll bcrypt advapi32)
elseif(UNIX AND NOT APPLE)
    target_link_libraries(ArcVu PRIVATE pthread dl m)
endif()
add_dependencies(ArcVu arcvu_dsp)
if(ARCVU_BUILD_CLAP)
    clap_juce_extensions_plugin(TARGET ArcVu
        CLAP_ID "com.opengrid.arcvu"
        CLAP_FEATURES audio-effect analyzer utility stereo
        CLAP_MANUAL_URL ""
        CLAP_SUPPORT_URL "")
endif()

option(ARCVU_BUILD_QA "Build the offscreen parameter and UI audit harness" OFF)
if(ARCVU_BUILD_QA)
    juce_add_console_app(ArcVu_QA PRODUCT_NAME "ArcVu QA")
    juce_generate_juce_header(ArcVu_QA)
    target_sources(ArcVu_QA PRIVATE qa/UiAudit.cpp plugin/PluginProcessor.cpp plugin/PluginEditor.cpp)
    target_include_directories(ArcVu_QA PRIVATE plugin)
    target_compile_definitions(ArcVu_QA PRIVATE ARCVU_BUILD_QA=1 JUCE_WEB_BROWSER=0 JUCE_USE_CURL=0 JUCE_MODAL_LOOPS_PERMITTED=1)
    target_link_libraries(ArcVu_QA PRIVATE ${ARCVU_LIB} juce::juce_audio_utils juce::juce_recommended_config_flags)
    add_dependencies(ArcVu_QA arcvu_dsp)
endif()
