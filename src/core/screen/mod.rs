cfg_select! {
    target_os = "windows" => {
        pub mod windows;
        pub use windows::Screen;
    }
    target_os = "linux" => {
        pub mod linux;
        pub use linux::Screen;
    }
    target_os = "macos" => {
        pub mod macos;
        pub use macos::Screen;
    }
}
