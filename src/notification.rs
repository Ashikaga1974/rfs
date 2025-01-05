use winrt_notification::Toast;

pub fn send_start_notification() {
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("Application Started")
        .text1("Your Rust application is running!")
        .text2("This is a Windows notification.")
        .show()
        .expect("Unable to send notification");
}

pub fn send_custom_notification(title: &str, text1: &str, text2: &str) {
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title(title)
        .text1(text1)
        .text2(text2)
        .show()
        .expect("Unable to send notification");
}
