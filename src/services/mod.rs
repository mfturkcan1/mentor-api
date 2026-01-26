use crate::establish_connection;
use crate::models::{NewRoutine, NewRoutinePart, Routine, RoutinePart};
use crate::repositories::{
    create_routine, create_routine_part, get_routine_parts, get_routine_parts_single, get_routines,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateRoutineRequest {
    title: String,
    create_date: DateTime<Utc>,
    parts: Vec<CreateRoutinePartRequest>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateRoutinePartRequest {
    description: String,
    start_hour: DateTime<Utc>,
    end_hour: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct RoutineSelectResponse {
    pub id: i32,
    pub title: String,
    pub create_date: DateTime<Utc>,
    pub parts: Vec<RoutinePart>,
}

pub fn create_routine_with_parts(
    create_routine_request: CreateRoutineRequest,
) -> Result<RoutineSelectResponse, diesel::result::Error> {
    let mut conn = establish_connection();
    let new_routine = NewRoutine {
        title: create_routine_request.title.as_str(),
        create_date: create_routine_request.create_date,
    };
    let routine_result = create_routine(&mut conn, new_routine)?;

    let routine_parts = create_routine_request
        .parts
        .iter()
        .map(|part| NewRoutinePart {
            description: part.description.as_str(),
            start_hour: part.start_hour,
            end_hour: part.end_hour,
            routine_id: routine_result.id,
        })
        .collect();

    let routine_part_results = create_routine_part(&mut conn, routine_parts)?;

    let result = RoutineSelectResponse {
        id: routine_result.id,
        title: routine_result.title,
        create_date: routine_result.create_date,
        parts: routine_part_results,
    };

    Ok(result)
}

pub fn select_routines() -> Result<Vec<RoutineSelectResponse>, diesel::result::Error> {
    let mut conn = establish_connection();

    let routines = get_routines(&mut conn)?;
    let routine_parts = get_routine_parts(&mut conn, &routines)?;

    let select_responses = routines
        .iter()
        .map(|routine| {
            let routine_belong_parts: Vec<RoutinePart> = routine_parts
                .iter()
                .filter(|rp| rp.routine_id == routine.id)
                .cloned()
                .collect();
            RoutineSelectResponse {
                id: routine.id,
                title: routine.title.clone(),
                create_date: routine.create_date,
                parts: routine_belong_parts,
            }
        })
        .collect();

    Ok(select_responses)
}

pub fn select_only_routines() -> Result<Vec<Routine>, diesel::result::Error> {
    let mut conn = establish_connection();
    get_routines(&mut conn)
}

pub fn select_routine_parts_by_id(
    routine_id: i32,
) -> Result<Vec<RoutinePart>, diesel::result::Error> {
    let mut conn = establish_connection();
    get_routine_parts_single(&mut conn, routine_id)
}
