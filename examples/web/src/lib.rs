include!("main.rs");

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn wasm_start_main() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    //#[cfg(feature = "console_error_panic_hook")]
    //console_error_panic_hook::set_once();

    // Only Print Panics With wasm-pack --dev
    #[cfg(debug_assertions)]
    {
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            use wasm_bindgen::JsValue;
            use web_sys::console::warn_1 as warn;

            hook(panic_info);
            if let Some(location) = panic_info.location() {
                let l = location.line();
                let f = location.file();
                let s = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                    s
                } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
                    &s
                } else {
                    "Unknown"
                };
                warn(&JsValue::from_str(&format!("Panic: {}:{}\n\n{}", f, l, s)));
            }
            web_sys::console::trace_0();
        }));
    }

    // Call the main function.
    main()
}
