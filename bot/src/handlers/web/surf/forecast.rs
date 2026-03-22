use crate::types::databases::Database;
use crate::{config::Config, env::Env};
use actix_web::web::Data;
use actix_web::{web, Error as ActixError, HttpRequest, HttpResponse};
use serde::Deserialize;
use shared::{SurfDailyReport, SurfReportResponse, TransactionQuery};
use std::sync::Arc;

#[derive(Deserialize)]
struct MarineRes {
    hourly: MarineHourly,
}

#[derive(Deserialize)]
struct MarineHourly {
    time: Vec<String>,
    swell_wave_height: Vec<Option<f32>>,
    swell_wave_period: Vec<Option<f32>>,
    swell_wave_direction: Vec<Option<f32>>,
    sea_level_height_msl: Vec<Option<f32>>,
}

#[derive(Deserialize)]
struct WeatherRes {
    hourly: WeatherHourly,
    daily: WeatherDaily,
}

#[derive(Deserialize)]
struct WeatherHourly {
    wind_speed_10m: Vec<Option<f32>>,
    wind_direction_10m: Vec<Option<f32>>,
}

#[derive(Deserialize)]
struct WeatherDaily {
    sunrise: Vec<String>,
    sunset: Vec<String>,
}

pub async fn get(
    _req: HttpRequest,
    _query: web::Query<TransactionQuery>,
    _jwt_secret: web::Data<String>,
    _env: web::Data<Arc<Env>>,
    config: web::Data<Arc<Config>>,
    _db: Data<Arc<Database>>,
) -> Result<HttpResponse, ActixError> {
    let marine_url = format!(
        "https://marine-api.open-meteo.com/v1/marine?latitude={}&longitude={}&hourly=swell_wave_height,swell_wave_period,swell_wave_direction,sea_level_height_msl&forecast_days=16&timezone=auto",
        config.surf.lat, config.surf.lon
    );

    let weather_url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=wind_speed_10m,wind_direction_10m&daily=sunrise,sunset&forecast_days=16&wind_speed_unit=ms&timezone=auto",
        config.surf.lat, config.surf.lon
    );

    let client = reqwest::Client::new();

    let (m_res, w_res) = tokio::try_join!(
        client.get(marine_url).send(),
        client.get(weather_url).send()
    )
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let m_data: MarineRes = m_res
        .json()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let w_data: WeatherRes = w_res
        .json()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let mut daily_reports: Vec<SurfDailyReport> = Vec::new();

    for i in 0..10 {
        let idx = i * 24 + 12;
        let day_start = i * 24;
        let day_end = day_start + 24;

        if idx < m_data.hourly.time.len() && idx < w_data.hourly.wind_speed_10m.len() && i < w_data.daily.sunrise.len() {
            let h = m_data.hourly.swell_wave_height[idx].unwrap_or(0.0);
            let p = m_data.hourly.swell_wave_period[idx].unwrap_or(0.0);

            let h_feet = h / 0.3048;
            let energy = h_feet.powi(2) * p;

            let tides = m_data.hourly.sea_level_height_msl[day_start..day_end.min(m_data.hourly.sea_level_height_msl.len())]
                .iter()
                .map(|v| v.unwrap_or(0.0))
                .collect::<Vec<f32>>();

            daily_reports.push(SurfDailyReport {
                swell_height: h,
                swell_period: p,
                swell_direction: m_data.hourly.swell_wave_direction[idx].unwrap_or(0.0),
                swell_energy: (energy * 100.0).round() / 100.0,
                wind_speed: w_data.hourly.wind_speed_10m[idx].unwrap_or(0.0),
                wind_direction: w_data.hourly.wind_direction_10m[idx].unwrap_or(0.0),
                timestamp: m_data.hourly.time[idx].clone(),
                sunrise: w_data.daily.sunrise[i].clone(),
                sunset: w_data.daily.sunset[i].clone(),
                hourly_tides: tides,
            });
        }
    }

    let response = SurfReportResponse {
        location: config.surf.location.to_string(),
        daily: daily_reports,
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({ "data": response })))
}
