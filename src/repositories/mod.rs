use crate::models::{NewCategory, NewRoutine, NewRoutinePart, Routine, RoutinePart};
use crate::schema::routine_parts::start_hour;
use crate::schema::routines::create_date;
use crate::schema::{categories, routine_parts, routines};
use diesel::{
    BelongingToDsl, ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl,
    SelectableHelper,
};
use std::collections::HashSet;

pub fn create_routine(
    conn: &mut PgConnection,
    new_routine: NewRoutine,
) -> Result<Routine, diesel::result::Error> {
    diesel::insert_into(routines::table)
        .values(&new_routine)
        .returning(Routine::as_returning())
        .get_result(conn)
}

pub fn create_routine_part(
    conn: &mut PgConnection,
    new_routine_parts: Vec<NewRoutinePart>,
) -> Result<Vec<RoutinePart>, diesel::result::Error> {
    diesel::insert_into(routine_parts::table)
        .values(&new_routine_parts)
        .returning(RoutinePart::as_returning())
        .get_results(conn)
}

pub fn create_categories(
    conn: &mut PgConnection,
    new_categories: HashSet<NewCategory>,
) -> Result<QueryResult<usize>, diesel::result::Error> {
    let category_names = new_categories
        .iter()
        .map(|c| &c.name)
        .collect::<Vec<&String>>();

    let exists_categories = categories::table
        .select(categories::name)
        .filter(categories::name.eq_any(category_names))
        .load::<String>(conn)?;

    let will_be_inserted_categories = new_categories
        .into_iter()
        .filter(|c| !exists_categories.contains(&c.name))
        .map(|c| NewCategory { name: c.name })
        .collect::<Vec<NewCategory>>();

    Ok(diesel::insert_into(categories::table)
        .values(&will_be_inserted_categories)
        .execute(conn))
}

pub fn get_routines(conn: &mut PgConnection) -> Result<Vec<Routine>, diesel::result::Error> {
    routines::table
        .select(Routine::as_select())
        .order_by(create_date.desc())
        .load(conn)
}

pub fn get_routine_by_id(conn: &mut PgConnection, id: i32) -> Result<Routine, diesel::result::Error> {
    routines::table.find(id).get_result::<Routine>(conn)
}

pub fn get_routine_parts_by_routine_id(conn: &mut PgConnection, routine_id: i32) -> Result<Vec<RoutinePart>, diesel::result::Error> {
    routine_parts::table.filter(
        routine_parts::routine_id.eq(routine_id),
    ).get_results(conn)
}

pub fn get_routine_parts(
    conn: &mut PgConnection,
    routines: &Vec<Routine>,
) -> Result<Vec<RoutinePart>, diesel::result::Error> {
    RoutinePart::belonging_to(routines)
        .select(RoutinePart::as_select())
        .load(conn)
}

pub fn get_routine_parts_single(
    conn: &mut PgConnection,
    routine_id: i32,
) -> Result<Vec<RoutinePart>, diesel::result::Error> {
    routine_parts::table
        .filter(routine_parts::routine_id.eq(routine_id))
        .select(RoutinePart::as_select())
        .order_by(start_hour::asc(Default::default()))
        .load(conn)
}

pub fn get_category_names(conn: &mut PgConnection) -> Result<Vec<String>, diesel::result::Error> {
    categories::table
        .select(categories::name)
        .distinct()
        .get_results(conn)
}
