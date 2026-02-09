use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct InputData {
    date: String,
    time: String,
}


#[derive(Serialize)]
struct Calculation {
    message: String,
    received_date: String,
    received_time: String,
    year_pillar: String,
    month_pillar: String,
    day_pillar: String,
    time_pillar: String,
}

fn get_stems() -> Vec<&'static str> {
    vec!["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"]
}

fn get_branches() -> Vec<&'static str> {
    vec!["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"]
}

fn get_julian_day(year: i32, month: i32, day: i32) -> i32 {
    let (adjusted_year, adjusted_month) = if month < 3 { (year - 1, month + 12) } else { (year, month) };
    let century = adjusted_year / 100;
    let gregorian_correction = 2 - century + century / 4;
    ((365.25 * (adjusted_year as f64 + 4716.0)) as i32)
    + ((30.6001 * (adjusted_month as f64 + 1.0)) as i32)
    + day + gregorian_correction - 1524
}

fn calculate_year_pillar(year: i32) -> String {
    let stems = get_stems();
    let branches = get_branches();
    let stem_index = (year - 4).rem_euclid(10) as usize;
    let branch_index = (year - 4).rem_euclid(12) as usize;

    format!("{}{}", stems[stem_index], branches[branch_index])
}

fn calculate_month_pillar(year: i32, month: i32) -> String {
    let stems = get_stems();
    let branches = get_branches();
    
    let branch_index = if month == 1 { 1 } else if month == 12 { 0 } else { month as usize };
    
    let year_stem_index = (year - 4).rem_euclid(10) as i32;
    let start_stem = (year_stem_index % 5) * 2 + 2;

    let month_offset = if month == 1 { 11 } else if month == 12 { 10 } else { month - 2};
    let stem_index = (start_stem + month_offset).rem_euclid(10) as usize;

    format!("{}{}", stems[stem_index], branches[branch_index as usize])
}

fn calculate_day_pillar(year: i32, month: i32, day: i32) -> (String, usize) {
    let stems = get_stems();
    let branches = get_branches();
    
    let julian_day_number = get_julian_day(year, month, day);
    let gan_zhi_num = (julian_day_number - 8).rem_euclid(60) as usize;
    
    let stem_index = gan_zhi_num % 10;
    let branch_index = gan_zhi_num % 12;
    
    (format!("{}{}", stems[stem_index], branches[branch_index]), stem_index)
}

fn calculate_time_pillar(day_stem_index: usize, hour: i32) -> String {
    let stems = get_stems();
    let branches = get_branches();

    let branch_index = ((hour + 1) / 2).rem_euclid(12) as usize;

    let start_stem = (day_stem_index % 5) * 2;
    let stem_index = (start_stem + branch_index).rem_euclid(10);

    format!("{}{}", stems[stem_index], branches[branch_index])
}

#[get("/json")]
async fn return_json(info: web::Query<InputData>) -> impl Responder {
    let parts_date: Vec<&str> = info.date.split(|c: char| !c.is_numeric()).filter(|s| !s.is_empty()).collect();
    let parts_time: Vec<&str> = info.time.split(|c: char| !c.is_numeric()).filter(|s| !s.is_empty()).collect();

    let year = parts_date[0].trim().parse().unwrap_or(2000);
    let month = parts_date[1].trim().parse().unwrap_or(1);
    let day = parts_date[2].trim().parse().unwrap_or(1);
    let hour = parts_time[0].trim().parse().unwrap_or(12);

    let mut calc_year = year;
    let mut calc_month = month;

    if day < 4 {
        calc_month -= 1;
        if calc_month < 1 { calc_month = 12; calc_year -= 1; }
    } else {
        if calc_month == 1 { calc_year -= 1; }
    }

    let year_str = calculate_year_pillar(calc_year);
    let month_str = calculate_month_pillar(calc_year, calc_month);

    let (day_str, day_stem_idx) = calculate_day_pillar(year, month, day);
    let time_str = calculate_time_pillar(day_stem_idx, hour);

    let msg = format!("Fate for {} {}", info.date, info.time);
    
    let result = Calculation {
        message: msg,
        received_date: info.date.clone(),
        received_time: info.time.clone(),
        year_pillar: year_str,
        month_pillar: month_str,
        day_pillar: day_str,
        time_pillar: time_str,
    };
    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Engine starting on port 8080");
    HttpServer::new(|| {
        App::new().service(return_json)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}