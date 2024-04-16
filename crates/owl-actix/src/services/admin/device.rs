use actix_web::{
    delete, get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use tracing::debug;

use crate::{AppState, ErrorStatus, QueryReturnMessage};

use super::{
    DeviceForCreateWrapper, DeviceForSelect, DeviceForSelectOne, DeviceForSelectOneWrapper,
    DeviceForSelectWrapper, DeviceForUpdate, DeviceForUpdateWrapper,
};

#[post("/create")]
pub async fn create_device(
    state: Data<AppState>,
    body: Json<DeviceForCreateWrapper>,
) -> impl Responder {
    debug!("{:<12} - create_device", "HANDLER");

    let query = "insert into device_info (device_name,device_tower,device_room,device_state) values (?,?,?,?)";

    let device: DeviceForCreateWrapper = body.into_inner();

    match sqlx::query(query)
        .bind(device.device.device_name)
        .bind(device.device.device_tower)
        .bind(device.device.device_room)
        .bind(device.device.device_state)
        .execute(&state.db)
        .await
    {
        Ok(_) => {
            let res = QueryReturnMessage {
                message: "Device created successfull".to_string(),
            };
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
pub async fn select_one_device(state: Data<AppState>, id: Path<i64>) -> impl Responder {
    debug!("{:<12} - select_one_device", "HANDLER");

    let query = "select id, device_name, device_tower, device_room, device_state, date_format(ctime, '%M %e, %Y') as created_at, date_format(mtime, '%M %e, %Y') as modified_at from device_info where id = ?";

    match sqlx::query_as::<_, DeviceForSelectOne>(query)
        .bind(*id)
        .fetch_one(&state.db)
        .await
    {
        Ok(device) => {
            let res = DeviceForSelectOneWrapper { device };
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

#[get("/all")]
pub async fn select_devices(state: Data<AppState>) -> impl Responder {
    debug!("{:<12} - select_devices", "HANDLER");

    let query = "select id, device_name, device_tower, device_room, device_state, date_format(ctime, '%M %e, %Y') as created_at, date_format(mtime, '%M %e, %Y') as modified_at from device_info where device_state != 'Removed' order by ctime asc";

    match sqlx::query_as::<_, DeviceForSelect>(query)
        .fetch_all(&state.db)
        .await
    {
        Ok(x) => {
            let devices = DeviceForSelectWrapper { devices: x };
            HttpResponse::Ok().json(devices)
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

#[post("/update")]
pub async fn update_device(
    state: Data<AppState>,
    body: Json<DeviceForUpdateWrapper>,
) -> impl Responder {
    debug!("{:<12} - update_device", "HANDLER");

    let query = "update device_info set device_state = ? where id = ?";
    let device: DeviceForUpdate = body.into_inner().device;

    match sqlx::query(query)
        .bind(device.device_state)
        .bind(device.id)
        .execute(&state.db)
        .await
    {
        Ok(_) => {
            let res = QueryReturnMessage {
                message: "Device state changed successfully".to_string(),
            };
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

#[delete("/delete/{id}")]
pub async fn delete_device(state: Data<AppState>, id: Path<i64>) -> impl Responder {
    debug!("{:<12} - update_device", "HANDLER");

    let query = "update device_info set device_state = 'Removed' where id = ?";

    match sqlx::query(query).bind(*id).execute(&state.db).await {
        Ok(_) => {
            let res = QueryReturnMessage {
                message: "Device state changed successfully".to_string(),
            };
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
