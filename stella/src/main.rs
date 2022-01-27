extern crate execute;

use serde_json::{Value, Map};
use std::{thread, time};
use std::process::Command;
use execute::Execute;

// define the possible categories for wallpaper
#[derive(Debug, Clone, Copy)]
enum TimeOfDay {
    
    Day(Option<Temperature>),
    Night(Option<Temperature>),

}

#[derive(Debug, Clone, Copy)]
enum Temperature {
    
    Hot,
    Cold

}

/*
 * Implement formating for the given enums for debugging purposes and allowing 
 * conversion to string for condition checks.
 */
impl std::fmt::Display for TimeOfDay {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl std::fmt::Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[tokio::main]
async fn main() {
    
    wallpaper_changerd().await.unwrap();

}

async fn wallpaper_changerd() -> Result<(), String> {
   
    /* 
     * Note:
     * Hot temperature is > 22 degress celcius
     * Cold temperature is <= 22 degrees celcius
     *
     * Day time is between 06.00 to 18.00
     * Night time is between 18.00 to 06.00
     *
     */

    /*
     * #### API URLs ####
     *
     * weather api: "http://dataservice.accuweather.com/forecasts/v1/daily/1day/234826?apikey=3rcCpg1dvHQFtIiGEksOfP2JUSge4zTE&day=1&unit=c&lang=en-us&details=true&metric=true"
     *
     * time api: "https://timeapi.io/api/Time/current/zone?timeZone=Asia/Dubai"
     */
    
    let mut map: Map<String, Value> = Map::new();
    
    // store the property of the day
    let mut condition: Option<TimeOfDay> = None;  // Just default to avoid uninitialized
    
    #[allow(irrefutable_let_patterns)] 
    while let _ = true {
        
        //get the time of day
        let body = get_from_api(String::from("https://timeapi.io/api/Time/current/zone?timeZone=Asia/Dubai")).await; 
        map = map_value(map, body).unwrap();
        let curr_time: Value = map["hour"].clone(); 
        let nowtime: i32 = (curr_time.to_string()).parse::<i32>().unwrap();  
       
        // will be used to compare new variant to old variant and 
        // decide whether to request for temp and change wallaper 
        let time_day: TimeOfDay;

        match nowtime {
            
            07..=17 => time_day = TimeOfDay::Day(None),
            18..=23 | 00..=06 => time_day = TimeOfDay::Night(None),
            _ => return Err(String::from("Invalid time"))
        
        } 
    
        if std::mem::discriminant(&condition) == std::mem::discriminant(&None) || std::mem::discriminant(&time_day) != std::mem::discriminant(&condition.unwrap()) {
        
            // get the weather data only if time of day is changed
            let body = get_from_api(String::from("http://dataservice.accuweather.com/forecasts/v1/daily/1day/234826?apikey=3rcCpg1dvHQFtIiGEksOfP2JUSge4zTE&day=1&unit=c&lang=en-us&details=true&metric=true"
            )).await;
            map = map_value(map, body).unwrap();
            // Extract current weather data
            let curr_temp: Value = map["DailyForecasts"][0]["RealFeelTemperature"]["Minimum"]["Value"].clone();    
            let temp = (curr_temp.to_string()).parse::<f32>().unwrap();
            condition = Some(time_day);
            
            // Do not judge; i dont know how to fix this right now.
            let mut time_day: Option<TimeOfDay> = Option::None;
            /*
             * This is an annoying bit beacause i dont know how to do it any other way.
             */
            if temp > 0.0 && temp <= 22.0 {
                
                match &condition.unwrap() {
                    TimeOfDay::Day(None) => time_day = Some(TimeOfDay::Day(Some(Temperature::Cold))),
                    TimeOfDay::Night(None) => time_day = Some(TimeOfDay::Night(Some(Temperature::Cold))),
                    _ => return Err(String::from("Invalid enum variant in time_day"))
                }

            } else if temp > 22.0 {
                
                match &condition.unwrap() {
                    TimeOfDay::Day(None) => time_day = Some(TimeOfDay::Day(Some(Temperature::Hot))),
                    TimeOfDay::Night(None) => time_day = Some(TimeOfDay::Night(Some(Temperature::Hot))),
                    _ => return Err(String::from("Invalid enum variant in time_day"))
                }

            }  
            
            change_wallpaper(time_day);
        } 

        // sleep until the next check 
        // 3600 seconds is 1 hour
        let time_to_sleep = time::Duration::from_secs(1800);
        println!("Going to sleep for 30mins.");
        thread::sleep(time_to_sleep);
    }

    // Your annoying
    Ok(())
}


// just for this once let it go please
// TODO: try to understand what the hell happened?????
#[allow(unused_assignments)]
fn map_value(mut map: Map<String, Value>, body: Result<String, reqwest::Error>) -> Result<Map<String, Value>, String> {
     
    match body {
        Ok(k) => match get_map_from_string(&k) {
            Ok(k) => map = k,
            Err(e) => return Err(e.to_string())
        },
        Err(e) => return Err(e.to_string()),
    }
    return Ok(map)

}

// convert json string to map
fn get_map_from_string(k: &String) -> Result<Map<String, Value>, String> {
   
    let parsed: Value = serde_json::from_str(&k).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
   
     return Ok(obj)

}

// get request to Accuweather api for weather json
async fn get_from_api(url: String) -> Result<String, reqwest::Error> {
    
    // let mut map = HashMap::new();
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;
    return Ok(body)

}

// use std::process::command and gnome command gsettings to change wallpaper
fn change_wallpaper(option: Option<TimeOfDay>) {
    
    let option = option.unwrap().to_string(); 
    let mut option: Vec<&str> = option.split("(").collect();

    //make shift fix
    if option[2].to_lowercase().contains("cold") {
        option[2] = "cold";
    } else if option[2].to_lowercase().contains("hot") {
        option[2] = "hot";
    }
    
   /*   */
    // println!("args are {} and {}", String::from(option[0]).to_lowercase(), String::from(option[2]));
   /*   */ 
    
    let mut command = Command::new("gsettings");
    command.arg("set");
    command.arg("org.gnome.desktop.background");
    command.arg("picture-uri");
    command.arg(format!("file:///home/akame/Prog/Stella_desktop/lofi_wallpaper/{}_{}.png", String::from(option[0]).to_lowercase(), String::from(option[2])));

    command.execute_output().unwrap();
    println!("Changed wallpaper to {}_{}", String::from(option[0]).to_lowercase(), String::from(option[2]));

}
