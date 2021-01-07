use auxtools::*;

use detour::RawDetour;

use std::ffi::c_void;

extern "C" {
    static mut send_maps_original: unsafe extern "C" fn();
}

#[hook("/proc/initialize_maptick")]
fn _init_maptick_hook() {
    let byondcore = sigscan::Scanner::for_module(BYONDCORE).unwrap();
    let mut send_maps_byond: *const c_void = std::ptr::null();
	if cfg!(windows) {
		let ptr = byondcore
			.find(signature!(
				"55 8B EC 6A FF 68 ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? 50 81 EC ?? ?? ?? ?? A1 ?? ?? ?? ?? 33 C5 89 45 F0 53 56 57 50 8D 45 F4 ?? ?? ?? ?? ?? ?? A0 ?? ?? ?? ?? 04 01 75 05 E8 ?? ?? ?? ?? E8"
			))
			.ok_or_else(|| runtime!("Couldn't find send_maps"))?;

        send_maps_byond = ptr as *const c_void;
	}

	if cfg!(unix) {
		let ptr = byondcore
			.find(signature!(
				"55 89 E5 57 56 53 81 EC ?? ?? ?? ?? 80 3D ?? ?? ?? ?? ?? 0F 84 ?? ?? ?? ??"
			))
			.ok_or_else(|| runtime!("Couldn't find send_maps"))?;

        send_maps_byond = ptr as *const c_void;
    }
    unsafe {
        let tick_hook = RawDetour::new(
            send_maps_byond as *const (),
            map_tick_hook as *const (),
        )
        .map_err(|_| runtime!("Couldn't detour send_maps"))?;
    
        tick_hook.enable().map_err(|_| runtime!("Couldn't enable send_maps detour"))?;
        send_maps_original = std::mem::transmute(tick_hook.trampoline());
        std::mem::forget(tick_hook);    
    };
    Ok(Value::null())
}

#[no_mangle]
extern "C" fn map_tick_hook() {
    let start = std::time::Instant::now();
    unsafe {
        send_maps_original();
    }
	Value::globals().set("internal_tick_usage", start.elapsed().as_micros() as f32 / 100000.0);
}