// чтобы в релизе на винде не прыгало лишнее окно консоли
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    srappy_lib::run()
}
