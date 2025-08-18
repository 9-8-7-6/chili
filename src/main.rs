pub mod action;
pub mod mail;

use crate::action::boot_on::check_boot_id;
use crate::mail::email_client::send_email;

fn main() {
    dotenv::dotenv().ok();

    let (is_new_boot, boot_time) = check_boot_id();

    if is_new_boot {
        send_email(boot_time);
    }
}
