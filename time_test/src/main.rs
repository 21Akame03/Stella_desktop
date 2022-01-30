use chrono::prelude::*;

fn main() {
    let time: DateTime<Utc> = Utc::now();
    let dt = time.with_timezone(&FixedOffset::east(4*3600));
    let dt = dt.hour();
    println!("{}", dt);
}
