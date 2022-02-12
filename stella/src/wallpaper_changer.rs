mod devices;
extern crate execute;

use log::{info, warn, error};
use serde_json::{Value, Map};
use std::{thread, time};
use std::process::Command;
use execute::Execute;
use chrono::prelude::*;
use self::devices::{ Earphone, check_earphones };

struct Condition {
    time_day: Option<TimeOfDay>,
    temperature: Option<Temperature>,
    earphone: Option<Earphone>,
} 

impl Condition {
    fn new() -> Self {
        Self{time_day: None, temperature:None, earphone:None}
    }
}


// define the possible categories for wallpaper
#[derive(Debug, Clone, Copy)]
enum TimeOfDay {
    Day,
    Night,
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

// the one to manage it all;
pub async fn wallpaper_changerd() -> Result<(), String> {
   
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
     * NOTE: deprecated (stopped working; switched to more effective native solution)
     * time api: "https://timeapi.io/api/Time/current/zone?timeZone=Asia/Dubai"
     */
    
    // let mut map: Map<String, Value> = Map::new();
     
    let mut condition = Condition::new();
    // usual average temperature
    let mut prev_temp: f32 = 23.0;

    #[allow(irrefutable_let_patterns)] 
    while let _ = true {
      
        //earphone variant
        let earphone = match check_earphones() {
            Ok(x) => x,
            Err(err) => {
                error!("[-] Error getting earphone data: {}", err);
                
                // local return to the variable
                Earphone::Deactivated 
            },
        };
        // timeOfDay variant
        let new_time_day: TimeOfDay = match check_time() {
            Ok(x) => x,
            Err(x) => return Err(x)
        };
        // temp variant
        let mut temp_variant: Option<Temperature> = None; 
        
        /*
         * ~ If the condition is None where this is the first run, check the temperature;
         * ~ Afterwards if there is no change in the variant of timeOfDay, do not run an api call
         * and use prev_temp value
         *
         */
        let mut body: Option<Result<String, reqwest::Error>> = None;
        let temp: f32;
        if are_variants_same(&condition.time_day, &None) || !are_variants_same(&condition.time_day.unwrap(), &new_time_day) {
        
            // get the weather data only if time of day is changed
            body = Some(get_from_api(String::from("http://dataservice.accuweather.com/forecasts/v1/daily/1day/234826?apikey=3rcCpg1dvHQFtIiGEksOfP2JUSge4zTE&day=1&unit=c&lang=en-us&details=true&metric=true"
            )).await); 
            
        }
        
        /* Extract current weather data 
         * if there is no internet connection, use default value
         */
        match map_value(body) {
            Ok(map) => {
                let curr_temp = map["DailyForecasts"][0]["RealFeelTemperature"]["Minimum"]["Value"].clone();
                temp = (curr_temp.to_string()).parse::<f32>().unwrap(); 
                prev_temp = temp;
            },
            Err(err) => {
                warn!("[-] Error in getting weather data: {}", err);
                temp = prev_temp.clone();
            } 
        }; 
     

        if temp > 0.0 && temp <= 22.0 {
           temp_variant = Some(Temperature::Cold); 
        } else if temp > 22.0 {
            temp_variant = Some(Temperature::Hot);
        }


        // if std::mem::discriminant(&condition.earphone) == std::mem::discriminant(&None) || std::mem::discriminant(&condition.earphone.unwrap()) != std::mem::discriminant(&earphone) {
        if are_variants_same(&condition.earphone, &None) || !are_variants_same(&condition.earphone.unwrap(), &earphone) || !are_variants_same(&condition.time_day.unwrap(), &new_time_day) {
            condition.temperature = temp_variant;
            condition.time_day = Some(new_time_day);
            condition.earphone = Some(earphone);
            
            change_wallpaper(&condition);
        }
        
        // sleep until the next check 
        let time_to_sleep = time::Duration::from_secs(5);
        thread::sleep(time_to_sleep);
    }

    // Your annoying
    Ok(())
}

fn are_variants_same <T> (variant1: &T, variant2: &T) -> bool {
    return std::mem::discriminant(variant1) == std::mem::discriminant(variant2)
}

fn check_time() -> Result<TimeOfDay, String> {
        
       let utc: DateTime<Utc> = Utc::now();
       
       // NOTE: Mauritius time is UTC + 4 hours.
       let currhour = utc.with_timezone(&FixedOffset::east(4*3600)).hour();

        // will be used to compare new variant to old variant and 
        // decide whether to request for temp and change wallaper 
        let new_condition: TimeOfDay;

        match currhour {
             
            07..=17 => new_condition = TimeOfDay::Day,
            18..=23 | 0..=06 => new_condition = TimeOfDay::Night,
            _ => return Err(String::from("Invalid time"))
        
        }
        
        return Ok(new_condition)
}


fn map_value( body: Option<Result<String, reqwest::Error>>) -> Result<Map<String, Value>, String> {

    let map: Map<String, Value>; 
    match body {
        Some(k) => match k {
            Ok(k) => match get_map_from_string(&k) {
                Ok(k) => map = k,
                Err(e) => return Err(e.to_string())
            },
            Err(e) => return Err(e.to_string()),
         },
         None => return Err(String::from("None type"))
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
fn change_wallpaper(condition: &Condition) {
    
    let temp = condition.temperature.unwrap().to_string().to_lowercase();
    let time = condition.time_day.unwrap().to_string().to_lowercase();
    let mut earphone = condition.earphone.unwrap().to_string().to_lowercase();
    
    if earphone == "deactivated" {
        earphone = "No_Headphone".to_string();
    } else if earphone == "activated" {
        earphone = "Headphone".to_string();
    }

    let mut command = Command::new("gsettings");
    command.arg("set");
    command.arg("org.gnome.desktop.background");
    command.arg("picture-uri");
    command.arg(format!("file:///home/akame/Prog/Stella_desktop/lofi_wallpaper/{}_{}_{}.png", time, temp, earphone));

    command.execute_output().unwrap();
    println!("Changed wallpaper to {}_{}_{}", time, temp, earphone);

}
