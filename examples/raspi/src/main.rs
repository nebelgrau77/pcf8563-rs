/* 
talking to the PCF8563 module over I2C
*/

use rppal::i2c::I2c;
use pcf8563::*;

fn main() {
    // new I2C instance with rppal
    let i2c = I2c::new().unwrap();

    // new RTC instance
    let mut rtc = PCF8563::new(i2c);    

    // initialize the clock module
    rtc.rtc_init().unwrap();

    // get date and time
    let now = rtc.get_datetime().unwrap();

    println!("It's {}, {} {} 20{:02} {:02}:{:02}:{:02}",
            weekday_name(now),
            now.day,
            month_name(now),
            now.year,
            now.hours,
            now.minutes,
            now.seconds);

}

// Helper function to get the correct name of the day
fn weekday_name(datetime: DateTime) -> &'static str  {
    let mut name = "Sunday";
    let day = datetime.weekday;
    match day {
        0 => {name = "Sunday"}
        1 => {name = "Monday"}
        2 => {name = "Tuesday"}
        3 => {name = "Wednesday"}
        4 => {name = "Thursday"}
        5 => {name = "Friday"}
        6 => {name = "Saturday"}
        _ => ()
    }
    name
}

// Helper function to get the correct name of the month
fn month_name(datetime: DateTime) -> &'static str  {
    let mut name = "January";
    let month = datetime.month;
    match month {
        1 => {name = "January"}
        2 => {name = "February"}
        3 => {name = "March"}
        4 => {name = "April"}
        5 => {name = "May"}
        6 => {name = "June"}
        7 => {name = "July"}
        8 => {name = "August"}
        9 => {name = "September"}
        10 => {name = "October"}
        11 => {name = "November"}
        12 => {name = "December"}        
        _ => ()
    }
    name
}