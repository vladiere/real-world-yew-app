use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use tracing::debug;

use crate::{
    admin::{
        MonitorForSelect, MonitorForSelectOne, MonitorForSelectOneWrapper, MonitorForSelectWrapper,
    },
    AppState, ErrorStatus,
};

#[get("/all")]
pub async fn monitor_select(state: Data<AppState>) -> impl Responder {
    debug!("{:<12} - monitor_select", "HANDLER");

    let query = "select id, client_name, building_tower, building_room, monitor_state, date_format(ctime,'%M %e, %Y') as date_begin,  date_format(mtime,'%M %e, %Y') as date_modified from monitoring_table order by mtime asc";

    match sqlx::query_as::<_, MonitorForSelect>(query)
        .fetch_all(&state.db)
        .await
    {
        Ok(monitors) => {
            let res = MonitorForSelectWrapper { monitors };
            HttpResponse::Ok().json(res)
        }
        Err(error) => {
            let error = ErrorStatus {
                message: format!("{error:?}"),
                status: 500,
            };
            HttpResponse::InternalServerError().json(error)
        }
    }
}

#[get("/one/{id}")]
pub async fn monitor_select_one(state: Data<AppState>, id: Path<i64>) -> impl Responder {
    debug!("{:<12} - monitor_select_one", "HANDLER");

    let query = "select id, client_name, building_tower, building_room, monitor_state, date_format(ctime,'%M %e, %Y') as date_begin,  date_format(mtime,'%M %e, %Y') as date_modified where id = ?";

    match sqlx::query_as::<_, MonitorForSelectOne>(query)
        .bind(*id)
        .fetch_one(&state.db)
        .await
    {
        Ok(monitor) => {
            let res = MonitorForSelectOneWrapper { monitor };
            HttpResponse::Ok().json(res)
        }
        Err(error) => {
            let error = ErrorStatus {
                message: format!("{error:?}"),
                status: 500,
            };
            HttpResponse::InternalServerError().json(error)
        }
    }
}
