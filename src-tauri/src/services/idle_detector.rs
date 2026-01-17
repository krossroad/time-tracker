#[cfg(target_os = "macos")]
pub fn get_idle_time_seconds() -> f64 {
    // Use raw FFI call to get idle time from macOS
    extern "C" {
        fn CGEventSourceSecondsSinceLastEventType(
            stateID: u32,
            eventType: u32,
        ) -> f64;
    }

    // kCGEventSourceStateCombinedSessionState = 0
    // kCGAnyInputEventType = 0xFFFFFFFF (~0u32)
    const COMBINED_SESSION_STATE: u32 = 0;
    const ANY_INPUT_EVENT_TYPE: u32 = 0xFFFFFFFF;

    unsafe {
        CGEventSourceSecondsSinceLastEventType(COMBINED_SESSION_STATE, ANY_INPUT_EVENT_TYPE)
    }
}

#[cfg(not(target_os = "macos"))]
pub fn get_idle_time_seconds() -> f64 {
    0.0
}

pub fn is_user_idle(threshold_minutes: u32) -> bool {
    let idle_seconds = get_idle_time_seconds();
    idle_seconds >= (threshold_minutes as f64 * 60.0)
}
