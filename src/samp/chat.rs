//! Чат, ввод и таблица игроков (TAB) SA-MP 0.3.7-R3.
//!
//! Нативный чат заменяется интерфейсом плагина: строки, которые игра добавляет в чат, перехватываются
//! (`on_entry`), а сама отрисовка чата, открытие строки ввода, переключение вида по F7 и таблица игроков по TAB
//! отключаются (`disable_native`). Адреса получены из базы IDA sampR3.dll (Chat / Input / Scoreboard).
//! Для других версий SA-MP ничего не делается (`install` возвращает false): нативный чат остаётся как был.

use std::ffi::{c_void, CStr, CString};
use std::os::raw::c_char;

use retour::GenericDetour;

use super::inputs::Input;
use super::version::{version, Version};
use super::{handle, v037r3};

// Адреса относительно samp.dll (R3). RVA = адрес в IDA - 0x10000000
const CHAT_ADD_ENTRY: usize = 0x67460; // CChat::AddEntry(int type, const char* text, const char* prefix, D3DCOLOR textColor, D3DCOLOR prefixColor)
const CHAT_DRAW: usize = 0x67680; // CChat::Draw
const CHAT_SCROLLBAR: usize = 0x11E; // CChat::m_pScrollbar (CDXUTScrollBar*)
const SCROLLBAR_SET_VISIBLE: usize = 64; // смещение SetVisible в vtable полосы прокрутки
const CHAT_SWITCH_MODE: usize = 0x60B50; // CChat::SwitchMode (F7)
const INPUT_OPEN: usize = 0x68D10; // CInput::Open (T, F6)
const INPUT_SEND: usize = 0x69190; // CInput::Send(const char*) - отправка строки на сервер
const INPUT_GET_COMMAND_HANDLER: usize = 0x68FA0; // CInput::GetCommandHandler(const char* name)
const SCOREBOARD_ENABLE: usize = 0x6EC80; // CScoreboard::Enable (TAB)
const HELP_DIALOG_SHOW: usize = 0x6B3C0; // CHelpDialog::Show (F1: окно с клавишами SA-MP)

/// Тип строки чата в нативном `CChat::AddEntry`
pub mod entry_type {
    /// Сообщение игрока: `prefix` - ник, `prefix_color` - цвет ника
    pub const CHAT: i32 = 2;
    /// Сообщение сервера (SendClientMessage) и сообщения игры
    pub const INFO: i32 = 4;
    /// Отладочное сообщение клиента
    pub const DEBUG: i32 = 8;
}

/// Максимальная длина введённой строки в SA-MP
pub const MAX_INPUT_LENGTH: usize = 128;

pub struct ChatEntry {
    pub kind: i32,
    /// Текст в кодировке игры (cp1251), без завершающего нуля
    pub text: Vec<u8>,
    pub prefix: Vec<u8>,
    /// Цвета ARGB
    pub text_color: u32,
    pub prefix_color: u32,
}

type AddEntryFn = extern "thiscall" fn(*mut c_void, i32, *const c_char, *const c_char, u32, u32);
type ThisFn = extern "thiscall" fn(*mut c_void);

static mut ENTRY_HOOK: Option<GenericDetour<AddEntryFn>> = None;
static mut ENTRY_CALLBACK: Option<Box<dyn FnMut(ChatEntry)>> = None;
static mut NOOP_HOOKS: Vec<GenericDetour<ThisFn>> = Vec::new();
static mut DRAW_HOOK: Option<GenericDetour<ThisFn>> = None;

extern "thiscall" fn add_entry_hook(this: *mut c_void, kind: i32, text: *const c_char, prefix: *const c_char, text_color: u32, prefix_color: u32) {
    unsafe {
        // Игра по-прежнему ведёт свой журнал (chatlog.txt) и историю, их никто не рисует
        if let Some(hook) = ENTRY_HOOK.as_ref() {
            hook.call(this, kind, text, prefix, text_color, prefix_color);
        }

        if text.is_null() {
            return;
        }

        if let Some(callback) = ENTRY_CALLBACK.as_mut() {
            callback(ChatEntry {
                kind,
                text: CStr::from_ptr(text).to_bytes().to_vec(),
                prefix: if prefix.is_null() { Vec::new() } else { CStr::from_ptr(prefix).to_bytes().to_vec() },
                text_color,
                prefix_color,
            });
        }
    }
}

extern "thiscall" fn noop_hook(_this: *mut c_void) {}

// Вместо отрисовки чата: полоса прокрутки чата SA-MP рисуется общим диалогом игры, а видимой её делал именно CChat::Draw
extern "thiscall" fn draw_hook(this: *mut c_void) {
    // Полоса прячется не каждый кадр, а раз в полсекунды: этого достаточно (её пересоздаёт только сброс устройства)
    static mut FRAME: u32 = 0;

    unsafe {
        FRAME = FRAME.wrapping_add(1);
        if FRAME % 30 != 1 {
            return;
        }

        let scrollbar = *((this as *const u8).add(CHAT_SCROLLBAR) as *const *mut c_void);
        if scrollbar.is_null() {
            return;
        }

        let vtable = *(scrollbar as *const *const usize);
        let set_visible: extern "thiscall" fn(*mut c_void, i32) = std::mem::transmute(*((vtable as *const u8).add(SCROLLBAR_SET_VISIBLE) as *const usize));
        set_visible(scrollbar, 0);
    }
}

fn supported() -> bool {
    version() == Version::V037R3 && !handle().is_null()
}

/// Перехватывает строки, которые игра добавляет в чат. Возвращает false, если версия SA-MP не поддерживается.
pub fn on_entry<F: FnMut(ChatEntry) + 'static>(callback: F) -> bool {
    if !supported() {
        return false;
    }

    unsafe {
        if ENTRY_HOOK.is_some() {
            return true;
        }

        let func: AddEntryFn = std::mem::transmute(handle().add(CHAT_ADD_ENTRY));

        match GenericDetour::new(func, add_entry_hook) {
            Ok(hook) => {
                ENTRY_CALLBACK = Some(Box::new(callback));
                if hook.enable().is_err() {
                    ENTRY_CALLBACK = None;
                    return false;
                }
                ENTRY_HOOK = Some(hook);
                true
            }
            Err(_) => false,
        }
    }
}

/// Отключает нативный чат: отрисовку, открытие строки ввода (T, F6), смену вида (F7), таблицу игроков (TAB) и справку SA-MP (F1)
pub fn disable_native() -> bool {
    if !supported() {
        return false;
    }

    unsafe {
        if !NOOP_HOOKS.is_empty() || DRAW_HOOK.is_some() {
            return true;
        }

        let draw: ThisFn = std::mem::transmute(handle().add(CHAT_DRAW));
        if let Ok(hook) = GenericDetour::new(draw, draw_hook) {
            if hook.enable().is_ok() {
                DRAW_HOOK = Some(hook);
            }
        }

        for address in [INPUT_OPEN, CHAT_SWITCH_MODE, SCOREBOARD_ENABLE, HELP_DIALOG_SHOW] {
            let func: ThisFn = std::mem::transmute(handle().add(address));

            if let Ok(hook) = GenericDetour::new(func, noop_hook) {
                if hook.enable().is_ok() {
                    NOOP_HOOKS.push(hook);
                }
            }
        }

        NOOP_HOOKS.len() == 4 && DRAW_HOOK.is_some()
    }
}

/// Игровой поток: строка, введённая игроком, разбирается так же, как это делает `CInput::ProcessInput`:
/// команда клиента (`/dl`, `/q`, ...) выполняется на месте, остальное уходит на сервер
pub fn submit(text: &[u8]) -> bool {
    if !supported() {
        return false;
    }

    let text = &text[..text.len().min(MAX_INPUT_LENGTH)];
    if text.is_empty() || text.contains(&0) {
        return false;
    }

    let Some(input) = Input::get() else { return false };
    let input_ptr = &mut *input as *mut Input as *mut c_void;

    unsafe {
        let get_handler: extern "thiscall" fn(*mut c_void, *const c_char) -> super::inputs::CMDPROC =
            std::mem::transmute(handle().add(INPUT_GET_COMMAND_HANDLER));
        let send: extern "thiscall" fn(*mut c_void, *const c_char) = std::mem::transmute(handle().add(INPUT_SEND));

        let full = match CString::new(text) {
            Ok(full) => full,
            Err(_) => return false,
        };

        if text[0] == b'/' {
            let split = text.iter().position(|&byte| byte == b' ');
            let (name, args) = match split {
                Some(at) => (&text[1..at], &text[at + 1..]),
                None => (&text[1..], &text[text.len()..]),
            };

            let name = CString::new(name).unwrap_or_default();
            let args = CString::new(args).unwrap_or_default();

            if let Some(handler) = get_handler(input_ptr, name.as_ptr()) {
                handler(args.as_ptr());
            } else if v037r3::CNetGame::get().is_some() {
                send(input_ptr, full.as_ptr());
            }
        } else {
            let default = { input.m_pDefaultCommand };
            if let Some(handler) = default {
                handler(full.as_ptr());
            }
        }
    }

    true
}
