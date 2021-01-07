use auxtools::*;

use detour::RawDetour;

use std::ffi::c_void;

thread_local! {
    static SEND_MAPS_ORIGINAL: unsafe extern "C" fn() = {
        let byondcore = sigscan::Scanner::for_module(BYONDCORE).unwrap();
        let mut send_maps_byond: *const c_void = std::ptr::null();
        if cfg!(windows) {
            let ptr = byondcore
                .find(signature!(
                    "55 8B EC 6A FF 68 ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? 50 81 EC ?? ?? ?? ?? A1 ?? ?? ?? ?? 33 C5 89 45 F0 53 56 57 50 8D 45 F4 ?? ?? ?? ?? ?? ?? A0 ?? ?? ?? ?? 04 01 75 05 E8 ?? ?? ?? ?? E8"
                ))
                .unwrap();
    
            send_maps_byond = ptr as *const c_void;
        }
    
        if cfg!(unix) {
            let ptr = byondcore
                .find(signature!(
                    "55 89 E5 57 56 53 81 EC ?? ?? ?? ?? 80 3D ?? ?? ?? ?? ?? 0F 84 ?? ?? ?? ??"
                ))
                .unwrap();
    
            send_maps_byond = ptr as *const c_void;
        }
        unsafe {
            let tick_hook = RawDetour::new(
                send_maps_byond as *const (),
                map_tick_hook as *const (),
            )
            .unwrap();
        
            tick_hook.enable().unwrap();
            let ret = std::mem::transmute(tick_hook.trampoline());
            std::mem::forget(tick_hook);
            ret
        }
    }
}

#[no_mangle]
unsafe extern "C" fn map_tick_hook() {
    let start = std::time::Instant::now();
    SEND_MAPS_ORIGINAL.with(|send_maps_original| {
        send_maps_original();
    });
	Value::globals().set("internal_tick_usage", start.elapsed().as_micros() as f32 / 100000.0);
}

#[hook("/proc/initialize_maptick")]
fn _init_map_tick() {
    Ok(Value::from(SEND_MAPS_ORIGINAL.with(|_| {
        true
    })))
}