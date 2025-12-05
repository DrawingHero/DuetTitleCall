#![feature(lazy_cell, ptr_sub_ptr)]
use engage::proc::ProcInstFields;
use unity::prelude::*;
//Making a struct with the same fields as ProcTitleCall__Fields
pub struct TitleSequence {
    // Proc Title Call 
    pub proc: ProcInstFields,
    pidorgid: &'static Il2CppString,
    isherofemale: bool,
    // End here
}
#[skyline::hook(offset = 0x2206350)]
pub fn titlesequence_oncreate(_this: &TitleSequence, method_info: OptionalMethod) {
    //Call the original function
    //This sets up who announces the title screen
    return call_original!(_this, method_info);
}
#[skyline::hook(offset = 0x22068d0)]
pub fn titlesequence_playvoicecommon(_this: &TitleSequence, soundeventname: &'static Il2CppString, isherofemale: bool, method_info: OptionalMethod) {
    //Call the original function
    call_original!(_this, soundeventname, isherofemale, method_info)
}
#[skyline::hook(offset = 0x2206ab0)]
pub fn titlesequence_playsubtitlevoice(_this: &TitleSequence, method_info: OptionalMethod) {
    //Reroll the PID or GID
    //This is done by calling the function that decides up who voices title screen again, after the "Fire Emblem" (Title) and before the "Engage" (Subtitle)
    titlesequence_oncreate(_this, method_info);
    //Play the specific voice line just like the original App.TitleSequence.ProcTitleCall$$PlaySubtitleVoice
    titlesequence_playvoicecommon(_this,Il2CppString::new("V_Title_02"), false, method_info);
    return;
}
/// The internal name of your plugin. This will show up in crash logs. Make it 8 characters long at max.
#[skyline::main(name = "FrEmエンゲジ")]
pub fn main() {
    // Install a panic handler for your plugin, allowing you to customize what to do if there's an issue in your code.
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        // Some magic thing to turn what was provided to the panic into a string. Don't mind it too much.
        // The message will be stored in the msg variable for you to use.
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };

        // This creates a new String with a message of your choice, writing the location of the panic and its message inside of it.
        // Note the \0 at the end. This is needed because show_error is a C function and expects a C string.
        // This is actually just a result of bad old code and shouldn't be necessary most of the time.
        let err_msg = format!(
            "Custom plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        // We call the native Error dialog of the Nintendo Switch with this convenient method.
        // The error code is set to 69 because we do need a value, while the first message displays in the popup and the second shows up when pressing Details.
        skyline::error::show_error(
            67,
            "Custom plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));

    // This is what you call to install your hook(s).
    // If you do not install your hook(s), they will just not execute and nothing will be done with them.
    // It is common to install then in ``main`` but nothing stops you from only installing a hook if some conditions are fulfilled.
    // Do keep in mind that hooks cannot currently be uninstalled, so proceed accordingly.
    //
    // A ``install_hooks!`` variant exists to let you install multiple hooks at once if separated by a comma.
    skyline::install_hooks!(titlesequence_playvoicecommon, titlesequence_playsubtitlevoice, titlesequence_oncreate);
}
