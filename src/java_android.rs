// Copyright © The Daybrite Project
// SPDX-License-Identifier: MPL-2.0

//! Android-only glue: the persistent files dir (for the store root) and the uncaught-exception
//! handler install. The handler itself lives in Java (`platform/android/java/.../crash/DayBreak.java`) so
//! that no JNI transition happens during a crash; it writes a `java-<sid>.kv` artifact directly,
//! in the same kv format [`crate::store`] reconciles.

use day_android::jni::objects::JString;
use day_android::{DayBridge, java, java_string, with_env};

// The uncaught-exception handler's Java half (platform/android/java/.../crash/DayBreak.java).
day_android::java_class! {
    DayBreak => dev.daybrite.day.crash.DayBreak,
    methods {
        static fn install(dir: JString, sid: JString),
    },
}

/// `Context.getFilesDir().getAbsolutePath()` via day-android's bridge (resolved on the main
/// thread at init).
pub(crate) fn files_dir() -> Option<std::path::PathBuf> {
    let path = with_env(|env| java_string(env, DayBridge::filesDirPath))?;
    if path.is_empty() {
        None
    } else {
        Some(std::path::PathBuf::from(path))
    }
}

/// Install the default uncaught-exception handler (Java side), pointing it at our store dir and
/// session id so its report artifact lands beside the native ones. Best-effort.
pub(crate) fn install(dir: &std::path::Path, sid: &str) {
    let dir = dir.to_string_lossy().to_string();
    java(|env| {
        let dir = JString::from_str(env, &dir)?;
        let sid = JString::from_str(env, sid)?;
        DayBreak::install(env, &dir, &sid)
    });
}
