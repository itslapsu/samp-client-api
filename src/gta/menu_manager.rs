use retour::GenericDetour;

use super::display::CSprite2d;
use super::matrix::CVector2D;

const MENU_MANAGER: usize = 0xBA6748;
const MAX_VOLUME: i8 = 64;
const PROCESS: usize = 0x57B440; // CMenuManager::Process(), thiscall, без аргументов

// Глобалы вне CMenuManager (мышь, звук машины мышью) и нативные функции "применить" для настроек,
// которые не сводятся к простой записи поля — см. CEF PauseMenu::Настройки
const MOUSE_ACCEL_HORZNTL: usize = 0xB6EC1C; // f32, CCamera::m_fMouseAccelHorzntl
const INVERT_MOUSE_Y: usize = 0xBA6745; // bool, bInvertMouseY
const VEHICLE_MOUSE_STEERING: usize = 0xC1CC02; // bool, CVehicle::m_bEnableMouseSteering
const VEHICLE_MOUSE_FLYING: usize = 0xC1CC03; // bool, CVehicle::m_bEnableMouseFlying
const LOD_DIST_SCALE: usize = 0x8CD800; // f32, CRenderer::ms_lodDistScale
const GAMMA: usize = 0xC92134; // CGamma instance
const GAMMA_SET_GAMMA: usize = 0x747200; // CGamma::SetGamma(float, bool), thiscall
const RW_TEXTURE_SET_MIPMAPPING: usize = 0x7F3530; // RwBool __cdecl(RwBool)
const RW_D3D9_CHANGE_MULTISAMPLING_LEVELS: usize = 0x7F8A90; // RwBool __cdecl(RwUInt32)
const RW_D3D9_GET_MAX_MULTISAMPLING_LEVELS: usize = 0x7F84E0; // RwUInt32 __cdecl(void)
const SET_VIDEO_MODE: usize = 0x745C70; // void __cdecl(int32) — пересоздаёт D3D9-буфер
const GET_VIDEO_MODE_LIST: usize = 0x745AF0; // char** __cdecl(void)
const GET_NUM_VIDEO_MODES: usize = 0x7F2CC0; // RwInt32 __cdecl(void)
const SAVE_SETTINGS: usize = 0x57C660; // CMenuManager::SaveSettings(), thiscall, пишет gta_sa.set

#[repr(C)]
pub struct CMenuManager {
    pub field_0: std::os::raw::c_char,
    pub field_1: [std::os::raw::c_char; 3],
    pub m_fStatsScrollSpeed: f32,
    pub field_8: std::os::raw::c_char,
    pub field_9: [std::os::raw::c_char; 23],
    pub field_20: std::os::raw::c_char,
    pub m_bHudOn: bool,
    pub field_22: [std::os::raw::c_char; 2],
    pub m_nRadarMode: std::os::raw::c_int,
    pub field_28: [std::os::raw::c_char; 4],
    pub m_nTargetBlipIndex: std::os::raw::c_int,
    pub field_30: std::os::raw::c_char,
    pub field_31: std::os::raw::c_char,
    pub m_bDontDrawFrontEnd: bool,
    pub m_bActivateMenuNextFrame: bool,
    pub m_bMenuAccessWidescreen: bool,
    pub field_35: std::os::raw::c_char,
    pub field_36: [std::os::raw::c_char; 2],
    pub field_38: std::os::raw::c_int,
    pub m_nBrightness: std::os::raw::c_int,
    pub m_fDrawDistance: f32,
    pub m_bShowSubtitles: bool,
    pub field_45: [std::os::raw::c_char; 4],
    pub field_49: std::os::raw::c_char,
    pub m_bMapLegend: bool,
    pub m_bWidescreenOn: bool,
    pub m_bFrameLimiterOn: bool,
    pub m_bRadioAutoSelect: bool,
    pub field_4E: std::os::raw::c_char,
    pub m_nSfxVolume: std::os::raw::c_char,
    pub m_nRadioVolume: std::os::raw::c_char,
    pub m_bRadioEq: bool,
    pub m_nRadioStation: std::os::raw::c_char,
    pub field_53: std::os::raw::c_char,
    pub m_nSelectedMenuItem: std::os::raw::c_int,
    pub field_58: std::os::raw::c_char,
    pub drawRadarOrMap: std::os::raw::c_char,
    pub field_5A: std::os::raw::c_char,
    pub field_5B: std::os::raw::c_char,
    pub m_bMenuActive: bool,
    pub doGameReload: std::os::raw::c_char,
    pub field_5E: std::os::raw::c_char,
    pub isSaveDone: std::os::raw::c_char,
    pub m_bLoadingData: bool,
    pub field_61: [std::os::raw::c_char; 3],
    pub m_fMapZoom: f32,
    pub m_fMapBaseX: f32,
    pub m_fMapBaseY: f32,
    pub m_vMousePos: CVector2D,
    pub field_78: std::os::raw::c_char,
    pub field_79: [std::os::raw::c_char; 3],
    pub titleLanguage: std::os::raw::c_int,
    pub textLanguage: std::os::raw::c_int,
    pub m_nLanguage: std::os::raw::c_char,
    pub m_nPreviousLanguage: std::os::raw::c_char,
    pub field_86: [std::os::raw::c_char; 2],
    pub field_88: std::os::raw::c_int,
    pub m_bLanguageChanged: bool,
    pub field_8D: [std::os::raw::c_char; 3],
    pub field_90: std::os::raw::c_int,
    pub field_94: std::os::raw::c_int,
    pub m_pJPegBuffer: *mut std::os::raw::c_char,
    pub field_9C: [std::os::raw::c_char; 16],
    pub field_AC: std::os::raw::c_int,
    pub m_nRadioMode: std::os::raw::c_char,
    pub invertPadX1: std::os::raw::c_char,
    pub invertPadY1: std::os::raw::c_char,
    pub invertPadX2: std::os::raw::c_char,
    pub invertPadY2: std::os::raw::c_char,
    pub swapPadAxis1: std::os::raw::c_char,
    pub swapPadAxis2: std::os::raw::c_char,
    pub field_B7: std::os::raw::c_char,
    pub m_bDrawMouse: bool,
    pub field_B9: [std::os::raw::c_char; 3],
    pub m_nMousePosLeft: std::os::raw::c_int,
    pub m_nMousePosTop: std::os::raw::c_int,
    pub m_bMipMapping: bool,
    pub m_bTracksAutoScan: bool,
    pub field_C6: std::os::raw::c_short,
    pub m_nAppliedAntiAliasingLevel: std::os::raw::c_int,
    pub m_nAntiAliasingLevel: std::os::raw::c_int,
    pub m_nController: std::os::raw::c_char,
    pub field_D1: [std::os::raw::c_char; 3],
    pub m_nAppliedResolution: std::os::raw::c_int,
    pub m_nResolution: std::os::raw::c_int,
    pub field_DC: std::os::raw::c_int,
    pub mousePosLeftA: std::os::raw::c_int,
    pub mousePosTopA: std::os::raw::c_int,
    pub m_bSavePhotos: bool,
    pub m_bMainMenuSwitch: bool,
    pub m_nPlayerNumber: std::os::raw::c_char,
    pub field_EB: std::os::raw::c_char,
    pub field_EC: std::os::raw::c_int,
    pub field_F0: std::os::raw::c_int,
    pub field_F4: std::os::raw::c_char,
    pub field_F5: [std::os::raw::c_char; 3],
    pub m_apTextures: [CSprite2d; 25],
    pub m_bTexturesLoaded: bool,
    pub m_nCurrentMenuPage: std::os::raw::c_uchar,
    pub field_15E: std::os::raw::c_char,
    pub m_bSelectedSaveGame: std::os::raw::c_uchar,
    pub field_160: std::os::raw::c_char,
    pub field_161: std::os::raw::c_char,
    pub m_szMpackName: [std::os::raw::c_char; 8],
    pub field_16A: [std::os::raw::c_char; 6486],
    pub field_1AC0: std::os::raw::c_int,
    pub field_1AC4: std::os::raw::c_int,
    pub field_1AC8: std::os::raw::c_int,
    pub field_1ACC: std::os::raw::c_int,
    pub field_1AD0: std::os::raw::c_int,
    pub field_1AD4: std::os::raw::c_int,
    pub field_1AD8: std::os::raw::c_int,
    pub field_1ADC: std::os::raw::c_short,
    pub m_bChangeVideoMode: bool,
    pub field_1ADF: std::os::raw::c_char,
    pub field_1AE0: std::os::raw::c_int,
    pub field_1AE4: std::os::raw::c_int,
    pub field_1AE8: std::os::raw::c_char,
    pub field_1AE9: std::os::raw::c_char,
    pub field_1AEA: std::os::raw::c_char,
    pub m_bScanningUserTracks: bool,
    pub field_1AEC: std::os::raw::c_int,
    pub field_1AF0: std::os::raw::c_char,
    pub field_1AF1: std::os::raw::c_char,
    pub field_1AF2: std::os::raw::c_char,
    pub field_1AF3: std::os::raw::c_char,
    pub field_1AF4: std::os::raw::c_int,
    pub field_1AF8: std::os::raw::c_int,
    pub field_1AFC: std::os::raw::c_int,
    pub field_1B00: std::os::raw::c_int,
    pub field_1B04: std::os::raw::c_int,
    pub field_1B08: std::os::raw::c_char,
    pub field_1B09: std::os::raw::c_char,
    pub field_1B0A: std::os::raw::c_char,
    pub field_1B0B: std::os::raw::c_char,
    pub field_1B0C: std::os::raw::c_int,
    pub field_1B10: std::os::raw::c_char,
    pub field_1B11: std::os::raw::c_char,
    pub field_1B12: std::os::raw::c_char,
    pub field_1B13: std::os::raw::c_char,
    pub field_1B14: std::os::raw::c_char,
    pub field_1B15: std::os::raw::c_char,
    pub field_1B16: std::os::raw::c_char,
    pub field_1B17: std::os::raw::c_char,
    pub EventToDo: std::os::raw::c_int,
    pub field_1B1C: std::os::raw::c_int,
    pub m_nTexturesRound: std::os::raw::c_uchar,
    pub m_nNumberOfMenuOptions: std::os::raw::c_uchar,
    pub field_1B22: std::os::raw::c_short,
    pub field_1B24: std::os::raw::c_int,
    pub field_1B28: std::os::raw::c_char,
    pub field_1B29: std::os::raw::c_char,
    pub field_1B2A: std::os::raw::c_short,
    pub field_1B2C: std::os::raw::c_int,
    pub field_1B30: std::os::raw::c_int,
    pub field_1B34: std::os::raw::c_short,
    pub field_1B36: std::os::raw::c_short,
    pub field_1B38: std::os::raw::c_int,
    pub field_1B3C: std::os::raw::c_char,
    pub field_1B3D: std::os::raw::c_char,
    pub field_1B3E: std::os::raw::c_char,
    pub field_1B3F: std::os::raw::c_char,
    pub field_1B40: std::os::raw::c_int,
    pub field_1B44: std::os::raw::c_char,
    pub field_1B45: std::os::raw::c_char,
    pub field_1B46: std::os::raw::c_short,
    pub field_1B48: std::os::raw::c_int,
    pub field_1B4C: std::os::raw::c_int,
    pub m_nBackgroundSprite: std::os::raw::c_char,
    pub field_1B51: std::os::raw::c_char,
    pub field_1B52: std::os::raw::c_short,
    pub field_1B54: std::os::raw::c_int,
    pub field_1B58: std::os::raw::c_int,
    pub field_1B5C: std::os::raw::c_char,
    pub field_1B5D: std::os::raw::c_char,
    pub field_1B5E: std::os::raw::c_short,
    pub field_1B60: std::os::raw::c_int,
    pub field_1B64: std::os::raw::c_int,
    pub field_1B68: std::os::raw::c_int,
    pub field_1B6C: std::os::raw::c_int,
    pub field_1B70: std::os::raw::c_int,
    pub field_1B74: std::os::raw::c_int,
}

impl CMenuManager {
    pub fn get<'a>() -> &'a CMenuManager {
        unsafe { &*(MENU_MANAGER as *const _) }
    }

    pub fn is_active(&self) -> bool {
        self.m_bMenuActive
    }

    pub fn current_page(&self) -> eMenuPage::Type {
        self.m_nCurrentMenuPage as _
    }

    pub fn sfx_volume(&self) -> f32 {
        self.m_nSfxVolume as f32 / MAX_VOLUME as f32
    }

    pub fn is_menu_active() -> bool {
        Self::get().is_active()
    }

    pub fn set_dont_draw_frontend(value: bool) {
        unsafe { (*(MENU_MANAGER as *mut CMenuManager)).m_bDontDrawFrontEnd = value; }
    }

    pub fn set_menu_active(value: bool) {
        unsafe { (*(MENU_MANAGER as *mut CMenuManager)).m_bMenuActive = value; }
    }

    // ---- Настройки игры (видео/звук/управление), см. CEF PauseMenu::Настройки ----
    // Часть значений — просто поля CMenuManager (виджет/пад), часть требует отдельного нативного
    // вызова "применить" (звук, mip-маппинг, сглаживание, яркость) — поле само по себе ничего не
    // меняет. Смена разрешения намеренно не реализована: небезопасно вызывать смену видеорежима
    // (device reset) без возможности проверить в игре.

    pub fn widescreen() -> bool { unsafe { (*(MENU_MANAGER as *const CMenuManager)).m_bWidescreenOn } }
    pub fn set_widescreen(value: bool) { unsafe { (*(MENU_MANAGER as *mut CMenuManager)).m_bWidescreenOn = value; } }

    pub fn frame_limiter() -> bool { unsafe { (*(MENU_MANAGER as *const CMenuManager)).m_bFrameLimiterOn } }
    pub fn set_frame_limiter(value: bool) { unsafe { (*(MENU_MANAGER as *mut CMenuManager)).m_bFrameLimiterOn = value; } }

    pub fn draw_distance() -> f32 { unsafe { (*(MENU_MANAGER as *const CMenuManager)).m_fDrawDistance } }
    pub fn set_draw_distance(value: f32) {
        unsafe {
            (*(MENU_MANAGER as *mut CMenuManager)).m_fDrawDistance = value;
            *(LOD_DIST_SCALE as *mut f32) = value; // CRenderer::ms_lodDistScale — иначе значение не подействует
        }
    }

    pub fn mipmapping() -> bool { unsafe { (*(MENU_MANAGER as *const CMenuManager)).m_bMipMapping } }
    pub fn set_mipmapping(value: bool) {
        unsafe {
            (*(MENU_MANAGER as *mut CMenuManager)).m_bMipMapping = value;
            let apply: extern "cdecl" fn(i32) -> i32 = std::mem::transmute(RW_TEXTURE_SET_MIPMAPPING);
            apply(value as i32);
        }
    }

    // ПОЛЕ + РЕАЛЬНОЕ ПРИМЕНЕНИЕ: применение сглаживания требует IDirect3DDevice9::Reset() (через
    // SetVideoMode/RwD3D9ChangeVideoMode) — дважды крашило игру под SA-MP при следующей отрисовке
    // текста на объекте (samp.dll!CObjectMaterialText::Create, m_pSprite становится нулевым — SA-MP
    // не пересоздаёт свои D3DPOOL_DEFAULT-ресурсы на сторонний Reset). Третья попытка: сам Reset
    // теперь оборачивается EndScene/BeginScene на стороне cef-plugin (render.rs::apply_antialiasing) —
    // там же есть доступ к устройству D3D9 напрямую. Эта функция только пишет поля; реальный вызов
    // смотри в cef-plugin. См. память cef-pausemenu
    pub fn antialiasing() -> i32 { unsafe { (*(MENU_MANAGER as *const CMenuManager)).m_nAntiAliasingLevel } }
    pub fn max_antialiasing() -> i32 {
        unsafe {
            let get_max: extern "cdecl" fn() -> u32 = std::mem::transmute(RW_D3D9_GET_MAX_MULTISAMPLING_LEVELS);
            (get_max().min(4)) as i32
        }
    }
    pub fn write_antialiasing_fields(level: i32) {
        unsafe {
            (*(MENU_MANAGER as *mut CMenuManager)).m_nAntiAliasingLevel = level;
            (*(MENU_MANAGER as *mut CMenuManager)).m_nAppliedAntiAliasingLevel = level;
        }
    }
    pub fn applied_resolution() -> i32 { unsafe { (*(MENU_MANAGER as *const CMenuManager)).m_nAppliedResolution } }
    pub fn write_resolution_field(mode: i32) {
        unsafe {
            (*(MENU_MANAGER as *mut CMenuManager)).m_nAppliedResolution = mode;
            (*(MENU_MANAGER as *mut CMenuManager)).m_nResolution = mode;
        }
    }

    // Список доступных видеорежимов (те, что GetVideoModeList не пометил недоступными — часть
    // индексов бывает null, это нормально). Индекс в паре — реальный индекс режима для SetVideoMode,
    // не позиция в списке (пропуски возможны)
    pub fn video_modes() -> Vec<(i32, String)> {
        unsafe {
            let count_fn: extern "cdecl" fn() -> i32 = std::mem::transmute(GET_NUM_VIDEO_MODES);
            let count = count_fn();
            if count <= 0 {
                return Vec::new();
            }

            let list_fn: extern "cdecl" fn() -> *mut *mut std::os::raw::c_char = std::mem::transmute(GET_VIDEO_MODE_LIST);
            let list = list_fn();
            if list.is_null() {
                return Vec::new();
            }

            let mut all = Vec::new();
            for i in 0..count {
                let ptr = *list.offset(i as isize);
                if ptr.is_null() {
                    continue;
                }
                let label = std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned();
                all.push((i, label));
            }

            // Метка режима — только "W X H X BPP" (без частоты обновления), поэтому один и тот же
            // W/H/BPP на разных частотах даёт несколько одинаковых на вид записей подряд — оставляем
            // одну на каждую метку. Если среди дублей есть текущий применённый режим, оставляем
            // именно его (иначе выпадающий список показал бы не тот индекс как "текущий")
            let current = (*(MENU_MANAGER as *const CMenuManager)).m_nAppliedResolution;
            let mut by_label: std::collections::HashMap<String, i32> = std::collections::HashMap::new();
            for (index, label) in &all {
                by_label
                    .entry(label.clone())
                    .and_modify(|kept| if *index == current { *kept = *index })
                    .or_insert(*index);
            }

            let mut modes: Vec<(i32, String)> = Vec::new();
            let mut added = std::collections::HashSet::new();
            for (index, label) in all {
                if by_label.get(&label) == Some(&index) && added.insert(label.clone()) {
                    modes.push((index, label));
                }
            }
            modes
        }
    }

    // Диапазон совпадает с ползунком в игре: примерно 0..1024 (native / 512.0 = уровень гаммы)
    pub fn brightness() -> i32 { unsafe { (*(MENU_MANAGER as *const CMenuManager)).m_nBrightness } }
    pub fn set_brightness(value: i32) {
        unsafe {
            (*(MENU_MANAGER as *mut CMenuManager)).m_nBrightness = value;
            let apply: extern "thiscall" fn(*mut (), f32, i32) = std::mem::transmute(GAMMA_SET_GAMMA);
            apply(GAMMA as *mut (), value as f32 / 512.0, 0);
        }
    }

    // 0..MAX_VOLUME (64), см. sfx_volume() для варианта в долях (0..1)
    pub fn set_sfx_volume(value: i8) {
        unsafe { (*(MENU_MANAGER as *mut CMenuManager)).m_nSfxVolume = value; }
        crate::gta::audio_engine::AudioEngine::set_effects_master_volume(value);
    }

    // Доля 0..1, как sfx_volume() — родное поле хранит 0..MAX_VOLUME (64)
    pub fn radio_volume() -> f32 {
        unsafe { (*(MENU_MANAGER as *const CMenuManager)).m_nRadioVolume as f32 / MAX_VOLUME as f32 }
    }
    pub fn set_radio_volume(value: i8) {
        unsafe { (*(MENU_MANAGER as *mut CMenuManager)).m_nRadioVolume = value; }
        crate::gta::audio_engine::AudioEngine::set_music_master_volume(value);
    }

    pub fn radio_eq() -> bool { unsafe { (*(MENU_MANAGER as *const CMenuManager)).m_bRadioEq } }
    pub fn set_radio_eq(value: bool) {
        unsafe { (*(MENU_MANAGER as *mut CMenuManager)).m_bRadioEq = value; }
        crate::gta::audio_engine::AudioEngine::set_bass_enhance(value);
    }

    pub fn radio_auto_select() -> bool { unsafe { (*(MENU_MANAGER as *const CMenuManager)).m_bRadioAutoSelect } }
    pub fn set_radio_auto_select(value: bool) {
        unsafe { (*(MENU_MANAGER as *mut CMenuManager)).m_bRadioAutoSelect = value; }
        crate::gta::audio_engine::AudioEngine::set_radio_auto_retune(value);
    }

    pub fn radio_station() -> i8 { unsafe { (*(MENU_MANAGER as *const CMenuManager)).m_nRadioStation } }
    pub fn set_radio_station(value: i8) {
        unsafe { (*(MENU_MANAGER as *mut CMenuManager)).m_nRadioStation = value; }
        crate::gta::audio_engine::AudioEngine::retune_radio(value);
    }

    // Инверсия/своп стика геймпада, порт 1/2 (в игре хранятся как char 0/1, не bool)
    pub fn pad_invert_x1() -> bool { unsafe { (*(MENU_MANAGER as *const CMenuManager)).invertPadX1 != 0 } }
    pub fn set_pad_invert_x1(v: bool) { unsafe { (*(MENU_MANAGER as *mut CMenuManager)).invertPadX1 = v as i8; } }
    pub fn pad_invert_y1() -> bool { unsafe { (*(MENU_MANAGER as *const CMenuManager)).invertPadY1 != 0 } }
    pub fn set_pad_invert_y1(v: bool) { unsafe { (*(MENU_MANAGER as *mut CMenuManager)).invertPadY1 = v as i8; } }
    pub fn pad_invert_x2() -> bool { unsafe { (*(MENU_MANAGER as *const CMenuManager)).invertPadX2 != 0 } }
    pub fn set_pad_invert_x2(v: bool) { unsafe { (*(MENU_MANAGER as *mut CMenuManager)).invertPadX2 = v as i8; } }
    pub fn pad_invert_y2() -> bool { unsafe { (*(MENU_MANAGER as *const CMenuManager)).invertPadY2 != 0 } }
    pub fn set_pad_invert_y2(v: bool) { unsafe { (*(MENU_MANAGER as *mut CMenuManager)).invertPadY2 = v as i8; } }
    pub fn pad_swap_axis1() -> bool { unsafe { (*(MENU_MANAGER as *const CMenuManager)).swapPadAxis1 != 0 } }
    pub fn set_pad_swap_axis1(v: bool) { unsafe { (*(MENU_MANAGER as *mut CMenuManager)).swapPadAxis1 = v as i8; } }
    pub fn pad_swap_axis2() -> bool { unsafe { (*(MENU_MANAGER as *const CMenuManager)).swapPadAxis2 != 0 } }
    pub fn set_pad_swap_axis2(v: bool) { unsafe { (*(MENU_MANAGER as *mut CMenuManager)).swapPadAxis2 = v as i8; } }

    // Мышь и управление машиной мышью — глобалы вне CMenuManager
    pub fn mouse_sensitivity() -> f32 { unsafe { *(MOUSE_ACCEL_HORZNTL as *const f32) } }
    pub fn set_mouse_sensitivity(value: f32) { unsafe { *(MOUSE_ACCEL_HORZNTL as *mut f32) = value; } }

    pub fn mouse_invert_y() -> bool { unsafe { *(INVERT_MOUSE_Y as *const bool) } }
    pub fn set_mouse_invert_y(value: bool) { unsafe { *(INVERT_MOUSE_Y as *mut bool) = value; } }

    pub fn vehicle_mouse_steering() -> bool { unsafe { *(VEHICLE_MOUSE_STEERING as *const bool) } }
    pub fn set_vehicle_mouse_steering(value: bool) { unsafe { *(VEHICLE_MOUSE_STEERING as *mut bool) = value; } }

    pub fn vehicle_mouse_flying() -> bool { unsafe { *(VEHICLE_MOUSE_FLYING as *const bool) } }
    pub fn set_vehicle_mouse_flying(value: bool) { unsafe { *(VEHICLE_MOUSE_FLYING as *mut bool) = value; } }

    // Пишет gta_sa.set (родной формат игры) — то же самое, что штатное сохранение настроек
    pub fn save_settings() {
        unsafe {
            let func: extern "thiscall" fn(*mut ()) = std::mem::transmute(SAVE_SETTINGS);
            func(MENU_MANAGER as *mut ());
        }
    }

    // Полностью отключает нативное меню паузы GTA:SA (детур CMenuManager::Process): пока выключено,
    // игра ни разу не заходит в CheckForMenuClosing — ESC никак не активирует/не закрывает нативное
    // меню, не проигрывает его звук и не останавливает обработку персонажа этим флагом. Вызывать один
    // раз при старте (install), дальше только переключать set_native_menu_enabled по кадрам.
    pub fn install_process_hook() {
        unsafe {
            let original: extern "thiscall" fn(*mut ()) = std::mem::transmute(PROCESS);
            if let Ok(hook) = GenericDetour::new(original, process_hook) {
                let _ = hook.enable();
                PROCESS_HOOK = Some(hook);
            }
        }
    }

    pub fn set_native_menu_enabled(enabled: bool) {
        unsafe { NATIVE_MENU_ENABLED = enabled; }
    }
}

static mut PROCESS_HOOK: Option<GenericDetour<extern "thiscall" fn(*mut ())>> = None;
static mut NATIVE_MENU_ENABLED: bool = true;

extern "thiscall" fn process_hook(this: *mut ()) {
    unsafe {
        if !NATIVE_MENU_ENABLED {
            return;
        }
        if let Some(hook) = PROCESS_HOOK.as_ref() {
            hook.call(this);
        }
    }
}

pub mod eMenuPage {
    pub type Type = i32;
    pub const MENUPAGE_STATS: Type = 0;
    pub const MENUPAGE_START_GAME: Type = 1;
    pub const MENUPAGE_BRIEF: Type = 2;
    pub const MENUPAGE_AUDIO_SETTINGS: Type = 3;
    pub const MENUPAGE_DISPLAY_SETTINGS: Type = 4;
    pub const MENUPAGE_MAP: Type = 5;
    pub const MENUPAGE_NEW_GAME_ASK: Type = 6;
    pub const MENUPAGE_SELECT_GAME: Type = 7;
    pub const MENUPAGE_MISSIONPACK_LOADING_ASK: Type = 8;
    pub const MENUPAGE_LOAD_GAME: Type = 9;
    pub const MENUPAGE_DELETE_GAME: Type = 10;
    pub const MENUPAGE_LOAD_GAME_ASK: Type = 11;
    pub const MENUPAGE_DELETE_GAME_ASK: Type = 12;
    pub const MENUPAGE_LOAD_FIRST_SAVE: Type = 13;
    pub const MENUPAGE_DELETE_FINISHED: Type = 14;
    pub const MENUPAGE_DELETE_SUCCESSFUL: Type = 15;
    pub const MENUPAGE_GAME_SAVE: Type = 16;
    pub const MENUPAGE_SAVE_WRITE_ASK: Type = 17;
    pub const MENUPAGE_SAVE_DONE_1: Type = 18;
    pub const MENUPAGE_SAVE_DONE_2: Type = 19;
    pub const MENUPAGE_GAME_SAVED: Type = 20;
    pub const MENUPAGE_GAME_LOADED: Type = 21;
    pub const MENUPAGE_GAME_WARNING_DONT_SAVE: Type = 22;
    pub const MENUPAGE_ASK_DISPLAY_DEFAULT_SETS: Type = 23;
    pub const MENUPAGE_ASK_AUDIO_DEFAULT_SETS: Type = 24;
    pub const MENUPAGE_ASK_CONTROLLER_DEFAULT_SETS: Type = 25;
    pub const MENUPAGE_USER_TRACKS_OPTIONS: Type = 26;
    pub const MENUPAGE_DISPLAY_ADVANCED: Type = 27;
    pub const MENUPAGE_LANGUAGE: Type = 28;
    pub const MENUPAGE_SAVE_GAME_DONE: Type = 29;
    pub const MENUPAGE_SAVE_GAME_FAILED: Type = 30;
    pub const MENUPAGE_SAVE_WRITE_FAILED: Type = 31;
    pub const MENUPAGE_SAVE_FAILED_FILE_ERROR: Type = 32;
    pub const MENUPAGE_OPTIONS: Type = 33;
    pub const MENUPAGE_MAIN_MENU: Type = 34;
    pub const MENUPAGE_QUIT_GAME_ASK: Type = 35;
    pub const MENUPAGE_CONTROLLER_SETUP: Type = 36;
    pub const MENUPAGE_REDEFINE_CONTROLS: Type = 37;
    pub const MENUPAGE_CONTROLS_VEHICLE_ONFOOT: Type = 38;
    pub const MENUPAGE_MOUSE_SETTINGS: Type = 39;
    pub const MENUPAGE_JOYPAD_SETTINGS: Type = 40;
    pub const MENUPAGE_PAUSE_MENU: Type = 41;
    pub const MENUPAGE_QUIT_GAME_2: Type = 42;
    pub const MENUPAGE_EMPTY: Type = 43;
}
