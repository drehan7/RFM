use std::panic;
use color_eyre::{config::HookBuilder, eyre};

use crate::tui;

// * Fixes the terminal on panic so nothing looks all jank
// Replaces standard color_eyre panic & error hooks with ones that replace terminal
pub fn install_hooks() -> color_eyre::Result<()> {
    let (panic_hook, eyre_hook) = HookBuilder::default().into_hooks();

    // Convert form color_eyre PanicHook to standard
    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = tui::restore();
        panic_hook(panic_info);
    }));

    // convert from color_eyre to ErrorHook
    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(
        move |error: &(dyn std::error::Error + 'static)| {
            let _ = tui::restore();
            eyre_hook(error)
        }
    ))?;

    Ok(())
}
