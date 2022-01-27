mod wallpaper_changer;
extern crate execute;

#[tokio::main]
async fn main() {
    
    // start the automatic wallpaper manager
    wallpaper_changer::wallpaper_changerd().await.unwrap();

}

