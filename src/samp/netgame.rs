// use std::ptr::NonNull;
use std::net::SocketAddr;

use super::{v037, v037r3, v03dlr1};
use super::version::{Version, version};
use retour::GenericDetour;
// use crate::samp::Gamestate;

pub struct NetGame<'a> {
    netgame_v1: Option<&'a mut v037::CNetGame>,
    netgame_v3: Option<&'a mut v037r3::CNetGame>,
    netgame_v03dl: Option<&'a mut v03dlr1::CNetGame>,
}

impl<'a> NetGame<'a> {
    pub fn get() -> NetGame<'a> {
        match version() {
            Version::V037 => NetGame {
                netgame_v1: v037::CNetGame::get(),
                netgame_v3: None,
                netgame_v03dl: None,
            },

            Version::V037R3 => NetGame {
                netgame_v1: None,
                netgame_v3: v037r3::CNetGame::get(),
                netgame_v03dl: None,
            },

            Version::V03DLR1 => NetGame {
                netgame_v1: None,
                netgame_v3: None,
                netgame_v03dl: v03dlr1::CNetGame::get(),
            },

            _ => panic!("Unknown SA:MP version"),
        }
    }

    pub fn addr(&self) -> Option<SocketAddr> {
        match version() {
            Version::V037 => self.netgame_v1.as_ref().and_then(|netgame| netgame.addr()),
            Version::V037R3 => self.netgame_v3.as_ref().and_then(|netgame| netgame.addr()),
            Version::V03DLR1 => self.netgame_v03dl.as_ref().and_then(|netgame| netgame.addr()),
            _ => None,
        }
    }

    pub fn on_destroy<F: FnMut() + 'static>(callback: F) {
        let address = match version() {
            Version::V037 => 0x9380,
            Version::V037R3 => 0x9510,
            Version::V03DLR1 => 0x9570,
            _ => return,
        };

        unsafe {
            let ptr = super::handle().add(address);
            let func: extern "thiscall" fn(this: *mut ()) = std::mem::transmute(ptr);

            let _ = GenericDetour::new(func, cnetgame_destroy)
                .map(|hook| {
                    let _ = hook.enable();

                    DESTROY_HOOK = Some(CNetGameDestroyHook {
                        hook,
                        callback: Box::new(callback),
                    });
                });
        }
    }

    pub fn on_reconnect<F: FnMut() + 'static>(callback: F) {
        let address = match version() {
            Version::V037 => 0xA060,
            Version::V037R3 => 0xA1E0,
            Version::V03DLR1 => 0xA230,
            _ => return,
        };

        unsafe {
            let ptr = super::handle().add(address);
            let func: extern "thiscall" fn(*mut ()) = std::mem::transmute(ptr);

            let _ = GenericDetour::new(func, cnetgame_reconnect)
                .map(|hook| {
                    let _ = hook.enable();

                    RECONNECT_HOOK = Some(CNetGameReconnectHook {
                        hook,
                        callback: Box::new(callback),
                    });
                });
        }
    }

    pub fn on_connected<F: FnMut() + 'static>(callback: F) {
        let address = match version() {
            Version::V037 => 0xA890,
            Version::V037R3 => 0xAA20,
            Version::V03DLR1 => 0xAA60,
            _ => return,
        };

        unsafe {
            let ptr = super::handle().add(address);
            let func: extern "thiscall" fn(*mut (), *mut ()) = std::mem::transmute(ptr);

            let _ = GenericDetour::new(func, cnetgame_connect)
                .map(|hook| {
                    let _ = hook.enable();

                    STATE_HOOK = Some(CNetGameStateHook {
                        hook,
                        callback: Box::new(callback),
                    });
                });
        }
    }

    pub fn on_closed_connection<F: FnMut() + 'static>(callback: F) {
        let address = match version() {
            Version::V037 => 0xA500,   //hz
            Version::V037R3 => 0x8A70,  
            Version::V03DLR1 => 0xA600, //hz
            _ => return,
        };
    
        unsafe {
            let ptr = super::handle().add(address);
            let func: extern "thiscall" fn(*mut (), *mut ()) = std::mem::transmute(ptr);
    
            let _ = GenericDetour::new(func, cnetgame_closed_connection)
                .map(|hook| {
                    let _ = hook.enable();
    
                    CLOSED_CONNECTION_HOOK = Some(CNetGameClosedConnectionHook {
                        hook,
                        callback: Box::new(callback),
                    });
                });
        }
    }

    pub fn server_is_full<F: FnMut() + 'static>(callback: F) {
        let address = match version() {
            Version::V037 => 0xA500,   //hz
            Version::V037R3 => 0x8A40,  
            Version::V03DLR1 => 0xA600, //hz
            _ => return,
        };
    
        unsafe {
            let ptr = super::handle().add(address);
            let func: extern "thiscall" fn(*mut (), *mut ()) = std::mem::transmute(ptr);
    
            let _ = GenericDetour::new(func, cnetgame_server_full)
                .map(|hook| {
                    let _ = hook.enable();
    
                    SERVER_FOOL_HOOK = Some(CNetGameServerFoolHook {
                        hook,
                        callback: Box::new(callback),
                    });
                });
        }
    }
}

struct CNetGameDestroyHook {
    hook: GenericDetour<extern "thiscall" fn(*mut ())>,
    callback: Box<dyn FnMut()>,
}

static mut DESTROY_HOOK: Option<CNetGameDestroyHook> = None;

extern "thiscall" fn cnetgame_destroy(this: *mut ()) {
    unsafe {
        if let Some(hook) = DESTROY_HOOK.as_mut() {
            (hook.callback)();
            hook.hook.call(this);
        }
    }
}

struct CNetGameStateHook {
    hook: GenericDetour<extern "thiscall" fn(*mut (), *mut ())>,
    callback: Box<dyn FnMut()>,
}

static mut STATE_HOOK: Option<CNetGameStateHook> = None;

extern "thiscall" fn cnetgame_connect(this: *mut (), packet: *mut ()) {
    unsafe {
        if let Some(hook) = STATE_HOOK.as_mut() {
            (hook.callback)();
            hook.hook.call(this, packet);
        }
    }
}

struct CNetGameReconnectHook {
    hook: GenericDetour<extern "thiscall" fn(*mut ())>,
    callback: Box<dyn FnMut()>,
}

static mut RECONNECT_HOOK: Option<CNetGameReconnectHook> = None;

extern "thiscall" fn cnetgame_reconnect(this: *mut ()) {
    unsafe {
        if let Some(hook) = RECONNECT_HOOK.as_mut() {
            (hook.callback)();
            hook.hook.call(this);
        }
    }
}

struct CNetGameClosedConnectionHook {
    hook: GenericDetour<extern "thiscall" fn(*mut (), *mut ())>,
    callback: Box<dyn FnMut()>,
}

static mut CLOSED_CONNECTION_HOOK: Option<CNetGameClosedConnectionHook> = None;

extern "thiscall" fn cnetgame_closed_connection(this: *mut (), packet: *mut ()) {
    unsafe {
        if let Some(hook) = CLOSED_CONNECTION_HOOK.as_mut() {
            (hook.callback)();  // Вызов callback
            hook.hook.call(this, packet); // Вызов оригинальной функции
        }
    }
}

static mut SERVER_FOOL_HOOK_HOOK: Option<CNetGameServerFoolHook> = None;

extern "thiscall" fn cnetgame_server_full(this: *mut (), packet: *mut ()) {
    unsafe {
        if let Some(hook) = SERVER_FOOL_HOOK_HOOK.as_mut() {
            (hook.callback)();  // Вызов callback
            hook.hook.call(this, packet); // Вызов оригинальной функции
        }
    }
}