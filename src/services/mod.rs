pub mod goal_service;

use crate::models::{
    NewCategory, NewRoutine, NewRoutinePart, Routine, RoutinePart, RoutinePartUsageRow,
};
use crate::repositories::{
    create_categories, create_routine, create_routine_part, delete_routine, delete_routine_part,
    get_category_names, get_routine_by_id, get_routine_parts, get_routine_parts_by_routine_id,
    get_routine_parts_single, get_routines, select_routine_parts_usage,
};
use crate::{establish_async_connection, establish_connection};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize)]
pub struct CreateRoutineRequest {
    title: String,
    create_date: DateTime<Utc>,
    update_date: Option<DateTime<Utc>>,
    delete_date: Option<DateTime<Utc>>,
    parts: Vec<CreateRoutinePartRequest>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateRoutinePartRequest {
    description: String,
    start_hour: DateTime<Utc>,
    end_hour: DateTime<Utc>,
    save_as_category: Option<bool>,
}

#[derive(Serialize)]
pub struct RoutineSelectResponse {
    pub id: i32,
    pub title: String,
    pub create_date: DateTime<Utc>,
    pub update_date: Option<DateTime<Utc>>,
    pub delete_date: Option<DateTime<Utc>>,
    pub parts: Vec<RoutinePart>,
}

pub fn create_routine_with_parts(
    create_routine_request: CreateRoutineRequest,
) -> Result<RoutineSelectResponse, diesel::result::Error> {
    let mut conn = establish_connection();

    let update_date = create_routine_request
        .update_date
        .unwrap_or_else(|| Utc::now());

    let new_routine = NewRoutine {
        title: create_routine_request.title.as_str(),
        create_date: create_routine_request.create_date,
        update_date,
        delete_date: None,
    };

    let routine_result = create_routine(&mut conn, new_routine)?;

    let mut routine_parts = Vec::new();
    let mut categories = HashSet::new();
    for part in create_routine_request.parts.iter() {
        let new_routine_part = NewRoutinePart {
            description: part.description.as_str(),
            start_hour: part.start_hour,
            end_hour: part.end_hour,
            routine_id: routine_result.id,
        };

        let new_category = NewCategory {
            name: part.description.clone(),
        };

        routine_parts.push(new_routine_part);
        categories.insert(new_category);
    }

    let routine_part_results = create_routine_part(&mut conn, routine_parts)?;

    let result = RoutineSelectResponse {
        id: routine_result.id,
        title: routine_result.title,
        create_date: routine_result.create_date,
        update_date: routine_result.update_date,
        delete_date: routine_result.delete_date,
        parts: routine_part_results,
    };

    if !categories.is_empty() {
        _ = create_categories(&mut conn, categories)?;
    }

    Ok(result)
}

pub fn create_routine_parts(
    parts: Vec<CreateRoutinePartRequest>,
    route_id: i32,
) -> Result<(), diesel::result::Error> {
    let mut conn = establish_connection();

    let mut new_routine_parts = Vec::new();
    let mut categories = HashSet::new();
    for part in parts.iter() {
        let routine_part = NewRoutinePart {
            description: part.description.as_str(),
            start_hour: part.start_hour,
            end_hour: part.end_hour,
            routine_id: route_id,
        };
        let new_category = NewCategory {
            name: part.description.clone(),
        };

        new_routine_parts.push(routine_part);
        categories.insert(new_category);
    }

    _ = create_routine_part(&mut conn, new_routine_parts)?;

    if !categories.is_empty() {
        _ = create_categories(&mut conn, categories)?;
    }

    Ok(())
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
                update_date: routine.update_date,
                delete_date: routine.delete_date,
                parts: routine_belong_parts,
            }
        })
        .collect();

    Ok(select_responses)
}

pub fn select_routine_by_id(
    routine_id: i32,
) -> Result<RoutineSelectResponse, diesel::result::Error> {
    let mut conn = establish_connection();
    let routine = get_routine_by_id(&mut conn, routine_id)?;
    let routine_belong_parts: Vec<RoutinePart> =
        get_routine_parts_by_routine_id(&mut conn, routine.id)?;

    Ok(RoutineSelectResponse {
        id: routine_id,
        title: routine.title,
        create_date: routine.create_date,
        update_date: routine.update_date,
        delete_date: routine.delete_date,
        parts: routine_belong_parts,
    })
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

pub fn select_category_names() -> Result<Vec<String>, diesel::result::Error> {
    let mut conn = establish_connection();
    get_category_names(&mut conn)
}

pub fn remove_routine(id: i32) -> Result<usize, diesel::result::Error> {
    let mut conn = establish_connection();
    delete_routine(&mut conn, id)
}

pub fn remove_routine_part(id: i32) -> Result<usize, diesel::result::Error> {
    let mut conn = establish_connection();
    delete_routine_part(&mut conn, id)
}

pub fn select_routine_parts_group_by_result()
-> Result<HashMap<DateTime<Utc>, Vec<RoutinePartUsageRow>>, diesel::result::Error> {
    let mut conn = establish_connection();
    select_routine_parts_usage(&mut conn)
}
