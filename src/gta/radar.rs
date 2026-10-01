// CRadar::ms_RadarTrace — глобальный массив блипов игры (0xBA86F0). Чтение мировых координат всех
// активных блипов для карты мира (PauseMenu → Карта) — движок не фильтрует этот массив по дальности,
// фильтрация есть только в проекции на HUD-радаре. Адрес/раскладка из декомпилированного
// gta-reversed-ref (source/game_sa/Radar.h).
//
// Установка GPS-метки через нативный CRadar::SetCoordBlip была здесь раньше — убрана: не вызывала
// SA-MP-коллбек OnPlayerClickMap (проверено в игре), метка теперь ставится сервером напрямую через
// Radar_SetMarker в ответ на событие с CEF-карты. См. память cef-worldmap.

const RADAR_TRACE: usize = 0xBA86F0;
const MAX_RADAR_TRACES: usize = 175;

// Повторяет layout tRadarTrace (Radar.h, VALIDATE_SIZE 0x28 = 40 байт). eRadarSprite — int8 (не i32!),
// eBlipDisplay/eBlipType/eBlipAppearance — uint8-битфлаги, отсюда именно такая раскладка полей.
// Не расшифровываем битовые флаги подробно — нужны только m_bTrackingBlip (активен ли слот) и спрайт
#[repr(C)]
pub struct RadarTrace {
    pub colour: u32,
    pub entity_handle: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub counter: u16,
    _pad0: u16,
    pub sphere_radius: f32,
    pub blip_size: u16,
    _pad1: u16,
    pub entry_exit: *const std::ffi::c_void,
    pub blip_sprite: i8,
    flags1: u8, // bright:1, tracking:1, shortRange:1, friendly:1, blipRemain:1, blipFade:1, coordAppearance:2
    flags2: u8, // display:2, type:4, appearance:2
    _pad2: u8,
}

impl RadarTrace {
    // m_bTrackingBlip — второй бит flags1 (после m_bBright): слот активен/используется
    pub fn is_tracking(&self) -> bool {
        self.flags1 & 0b10 != 0
    }
}

const _: () = assert!(std::mem::size_of::<RadarTrace>() == 0x28);

pub struct CRadar;

impl CRadar {
    // Все 175 слотов блипов игры (HUD-радар и карта мира читают один и тот же массив)
    pub fn all_traces() -> &'static [RadarTrace; MAX_RADAR_TRACES] {
        unsafe { &*(RADAR_TRACE as *const [RadarTrace; MAX_RADAR_TRACES]) }
    }
}
