use crate::types::databases::Database;
use actix_web::{web, Error as ActixError, HttpResponse};
use shared::SysInfoResponse;
use std::sync::Arc;
use std::time::Instant;
use sysinfo::System;

pub async fn get(db: web::Data<Arc<Database>>) -> Result<HttpResponse, ActixError> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let db_start = Instant::now();
    let _ = sqlx::query("SELECT 1").execute(&db.pool).await;
    let db_latency = db_start.elapsed().as_millis();

    let response = SysInfoResponse {
        uptime: System::uptime(),
        total_mem: sys.total_memory(),
        used_mem: sys.used_memory(),
        cpu_usage: sys.global_cpu_usage(),
        os_version: System::os_version(),
        db_latency_ms: db_latency,
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({ "data": response })))
}
