// CControllerConfigManager — таблица привязок клавиш игры (глобал ControlsManager, 0xB70198).
// Адреса и порядок enum из декомпилированного gta-reversed-ref (ControllerConfigManager.h/.cpp).

const CONTROLS_MANAGER: usize = 0xB70198;

const GET_KEY: usize = 0x52F4F0; // KeyCode GetControllerKeyAssociatedWithAction(eControllerAction, eControllerType)
const SET_KEY: usize = 0x530490; // void SetControllerKeyAssociatedWithAction(eControllerAction, KeyCode, eControllerType)
const RESET_DEFAULTS: usize = 0x530640; // void InitDefaultControlConfiguration()

// eControllerType (Enums/eControllerType.h)
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ControllerType {
    Keyboard = 0,
    OptionalExtraKey = 1,
    Mouse = 2,
    JoyStick = 3,
}

pub struct ControlsManager;

impl ControlsManager {
    pub fn get_key(action: i32, kind: ControllerType) -> u32 {
        unsafe {
            let func: extern "thiscall" fn(*mut (), i32, i32) -> u32 = std::mem::transmute(GET_KEY);
            func(CONTROLS_MANAGER as *mut (), action, kind as i32)
        }
    }

    pub fn set_key(action: i32, kind: ControllerType, key: u32) {
        unsafe {
            let func: extern "thiscall" fn(*mut (), i32, u32, i32) = std::mem::transmute(SET_KEY);
            func(CONTROLS_MANAGER as *mut (), action, key, kind as i32);
        }
    }

    // Возврат всех клавиш к заводским (как "По умолчанию" в ванильном меню Controller Setup)
    pub fn reset_to_defaults() {
        unsafe {
            let func: extern "thiscall" fn(*mut ()) = std::mem::transmute(RESET_DEFAULTS);
            func(CONTROLS_MANAGER as *mut ());
        }
    }
}

// eControllerAction (ControllerConfigManager.h) — только реальные игровые действия, без служебных
// (COMBOLOCK/CA_NONE — сентинелы, NUM_OF_1ST_PERSON_ACTIONS — счётчик, TOGGLE_DPAD/SWITCH_DEBUG_CAM_ON/
// TAKE_SCREEN_SHOT/SHOW_MOUSE_POINTER_TOGGLE/SWITCH_CAM_DEBUG_MENU — мобильная версия/debug-меню).
pub const ACTIONS: &[(i32, &str)] = &[
    (0, "Стрельба"),
    (1, "Стрельба (альт.)"),
    (2, "Следующее оружие"),
    (3, "Предыдущее оружие"),
    (4, "Идти вперёд"),
    (5, "Идти назад"),
    (6, "Идти влево"),
    (7, "Идти вправо"),
    (8, "Приблизить прицел снайпера"),
    (9, "Отдалить прицел снайпера"),
    (10, "Сесть/выйти из транспорта"),
    (11, "Сменить вид камеры"),
    (12, "Прыжок"),
    (13, "Бег"),
    (14, "Взгляд назад"),
    (15, "Присесть"),
    (16, "Ответить на звонок"),
    (17, "Шаг (ходьба)"),
    (18, "Стрельба из транспорта"),
    (19, "Стрельба из транспорта (альт.)"),
    (20, "Руль влево"),
    (21, "Руль вправо"),
    (22, "Руль вверх (самолёт)"),
    (23, "Руль вниз (самолёт)"),
    (24, "Газ"),
    (25, "Тормоз/задний ход"),
    (26, "Радио: следующая станция"),
    (27, "Радио: предыдущая станция"),
    (28, "Радио: следующий трек"),
    (29, "Клаксон"),
    (30, "Доп. функция (сабмиссии)"),
    (31, "Ручной тормоз"),
    (32, "Взгляд влево (от первого лица)"),
    (33, "Взгляд вправо (от первого лица)"),
    (34, "Обзор влево (в транспорте)"),
    (35, "Обзор вправо (в транспорте)"),
    (36, "Обзор назад (в транспорте)"),
    (37, "Управление мышью в транспорте"),
    (38, "Турель влево"),
    (39, "Турель вправо"),
    (40, "Турель вверх"),
    (41, "Турель вниз"),
    (42, "Следующая цель"),
    (43, "Предыдущая цель"),
    (44, "Камера за спиной персонажа"),
    (45, "Захват цели"),
    (46, "Голосовой чат (рация)"),
    (47, "Ответ «Да»"),
    (48, "Ответ «Нет»"),
    (49, "Группа: вперёд"),
    (50, "Группа: назад"),
    (51, "Взгляд вверх (от первого лица)"),
    (52, "Взгляд вниз (от первого лица)"),
];
