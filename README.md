# Solo D&D App Skeleton

This is a starter project for a text-based solo D&D-style engine.

## Layout

- `engine/` – Rust engine crate (rules, campaign model, simple combat, FFI hooks)
- `content/` – Sample campaign and pre-generated character
- `android/` – Minimal Android + Jetpack Compose skeleton and JNI wrapper (you may want to regenerate the Gradle project in Android Studio and then drop these sources in).

## Engine

The Rust crate is configured as both an `rlib` and `cdylib` so you can:

- Use it as a normal Rust library (for CLI or desktop)
- Build it as a shared library for Android via JNI

## Notes

- No paid APIs are used.
- The engine is pure Rust with no network calls; you can add local LLM / free 5e API integration later.
- Saving/loading is not implemented yet; you can store state as JSON or in SQLite depending on your platform.
