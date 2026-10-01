// CAudioEngine — глобальный движок звука/радио игры (0xB6BC90). Методы thiscall, адреса из
// декомпилированного gta-reversed-ref (Audio/AudioEngine.cpp).

const AUDIO_ENGINE: usize = 0xB6BC90;

const SET_EFFECTS_MASTER_VOLUME: usize = 0x506E10; // void(int8)
const SET_MUSIC_MASTER_VOLUME: usize = 0x506DE0; // void(int8)
const SET_BASS_ENHANCE_ON_OFF: usize = 0x506F90; // void(bool)
const SET_RADIO_AUTO_RETUNE_ON_OFF: usize = 0x506F80; // void(bool)
const RETUNE_RADIO: usize = 0x507E10; // void(int8 eRadioID)

pub struct AudioEngine;

impl AudioEngine {
    pub fn set_effects_master_volume(volume: i8) {
        unsafe {
            let func: extern "thiscall" fn(*mut (), i8) = std::mem::transmute(SET_EFFECTS_MASTER_VOLUME);
            func(AUDIO_ENGINE as *mut (), volume);
        }
    }

    pub fn set_music_master_volume(volume: i8) {
        unsafe {
            let func: extern "thiscall" fn(*mut (), i8) = std::mem::transmute(SET_MUSIC_MASTER_VOLUME);
            func(AUDIO_ENGINE as *mut (), volume);
        }
    }

    pub fn set_bass_enhance(enable: bool) {
        unsafe {
            let func: extern "thiscall" fn(*mut (), bool) = std::mem::transmute(SET_BASS_ENHANCE_ON_OFF);
            func(AUDIO_ENGINE as *mut (), enable);
        }
    }

    pub fn set_radio_auto_retune(enable: bool) {
        unsafe {
            let func: extern "thiscall" fn(*mut (), bool) = std::mem::transmute(SET_RADIO_AUTO_RETUNE_ON_OFF);
            func(AUDIO_ENGINE as *mut (), enable);
        }
    }

    pub fn retune_radio(station: i8) {
        unsafe {
            let func: extern "thiscall" fn(*mut (), i8) = std::mem::transmute(RETUNE_RADIO);
            func(AUDIO_ENGINE as *mut (), station);
        }
    }
}

// eRadioID (game_sa/Enums/eRadioID.h): станции игры + выкл/пользовательские треки
pub mod radio {
    pub const STATIONS: &[(i8, &str)] = &[
        (0, "AA (аварийная)"),
        (1, "Playback FM"),
        (2, "K-Rose"),
        (3, "K-DST"),
        (4, "Bounce FM"),
        (5, "SF-UR"),
        (6, "Radio Los Santos"),
        (7, "Radio X"),
        (8, "CSR 103.9"),
        (9, "K-Jah West"),
        (10, "Master Sounds 98.3"),
        (11, "WCTR"),
        (12, "Пользовательские треки"),
        (13, "Выключено"),
    ];
}
