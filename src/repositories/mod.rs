use crate::models::{NewRoutine, NewRoutinePart, Routine, RoutinePart};
use crate::schema::routine_parts::start_hour;
use crate::schema::routines::create_date;
use crate::schema::{routine_parts, routines};
use diesel::{
    BelongingToDsl, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};

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

pub fn get_routines(conn: &mut PgConnection) -> Result<Vec<Routine>, diesel::result::Error> {
    routines::table
        .select(Routine::as_select())
        .order_by(create_date.desc())
        .load(conn)
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
