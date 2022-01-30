mod wallpaper_changer;
extern crate execute;

use notify_rust::Notification;

#[tokio::main]
async fn main() {
    let mut i: i8 = 0;
    
    // give the program 3 chances to restart itself
    while i < 4 {
   
        // the x in Ok was being annoying
        #[allow(unused_variables)]
        // start the automatic wallpaper manager
        match wallpaper_changer::wallpaper_changerd().await {
           Err(x) => show_notification(x),
           Ok(x) => ()
        }
        
        i += 1;
       
    }
   
}

// show notifications
fn show_notification(x: String) {
    Notification::new()
        .summary("Stella Desktop")
        .body(&x)
        .show().unwrap();
}
