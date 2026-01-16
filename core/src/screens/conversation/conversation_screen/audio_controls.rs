//! Audio control methods for ConversationScreen
//!
//! Handles audio device selection, mic monitoring, and level visualization.

use makepad_widgets::*;
use makepad_component::*;

use super::ConversationScreen;

impl ConversationScreen {
    /// Initialize audio manager and timers (UI controls removed)
    pub(super) fn init_audio(&mut self, cx: &mut Cx) {
        let mut audio_manager = crate::audio::AudioManager::new();

        // Start mic monitoring with default device
        if let Err(e) = audio_manager.start_mic_monitoring(None) {
            eprintln!("Failed to start mic monitoring: {}", e);
        }

        self.audio_manager = Some(audio_manager);

        // Initialize audio player for TTS playback (32kHz for PrimeSpeech)
        match crate::audio_player::create_audio_player(32000) {
            Ok(player) => {
                ::log::info!("Audio player initialized (32kHz)");
                self.audio_player = Some(player);
            }
            Err(e) => {
                ::log::error!("Failed to create audio player: {}", e);
            }
        }

        // Start timer for mic level updates (50ms) and dora polling (100ms)
        self.audio_timer = cx.start_interval(0.05);
        self.dora_timer = cx.start_interval(0.1);

        // Initialize demo log entries
        self.init_demo_logs(cx);

        self.view.redraw(cx);
    }

    /// Initialize log entries with a startup message
    pub(super) fn init_demo_logs(&mut self, cx: &mut Cx) {
        // Start with empty logs - real logs will come from log_bridge
        self.log_entries = vec![
            "[INFO] [App] Colang initialized".to_string(),
            "[INFO] [App] System log ready - Rust logs will appear here".to_string(),
        ];

        // Update the log display
        self.update_log_display(cx);
    }

    /// Update mic level (no visual meter in the new layout)
    pub(super) fn update_mic_level(&mut self, _cx: &mut Cx) {
        if let Some(ref audio_manager) = self.audio_manager {
            let _ = audio_manager.get_mic_level();
        }
    }

    /// Select input device for mic monitoring
    pub(super) fn select_input_device(&mut self, _cx: &mut Cx, device_name: &str) {
        if let Some(ref mut audio_manager) = self.audio_manager {
            if let Err(e) = audio_manager.set_input_device(device_name) {
                eprintln!("Failed to set input device '{}': {}", device_name, e);
            }
        }
    }

    /// Select output device
    pub(super) fn select_output_device(&mut self, device_name: &str) {
        if let Some(ref mut audio_manager) = self.audio_manager {
            audio_manager.set_output_device(device_name);
        }
    }
}
