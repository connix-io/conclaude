use notify_rust::Notification;
use notify_rust::Timeout;

fn main() {
    Notification::new()
        .summary("Firefox News")
        .body("This will almost look like a real firefox notification.")
        .icon("firefox")
        .timeout(Timeout::Milliseconds(6000)) //milliseconds
        .show()
        .unwrap();
}
