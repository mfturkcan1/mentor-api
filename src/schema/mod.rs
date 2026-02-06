diesel::table! {
    routines (id) {
        id -> Int4,
        title -> Text,
        create_date -> Timestamptz,
        update_date -> Nullable<Timestamptz>,
        delete_date -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    routine_parts(id){
        id -> Int4,
        description -> Text,
        start_hour -> Timestamptz,
        end_hour -> Timestamptz,
        routine_id -> Int4,
        delete_date -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(routine_parts -> routines (routine_id));

diesel::allow_tables_to_appear_in_same_query!(routines, routine_parts);

diesel::table! {
    categories(id){
        id -> Int4,
        name ->  Text
    }
}

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType, diesel::query_builder::QueryId)]
    #[diesel(postgres_type(name = "goal_period_type"))]
    pub struct GoalPeriodType;

    #[derive(diesel::sql_types::SqlType, diesel::query_builder::QueryId)]
    #[diesel(postgres_type(name = "goal_status"))]
    pub struct GoalStatus;

    #[derive(diesel::sql_types::SqlType, diesel::query_builder::QueryId)]
    #[diesel(postgres_type(name = "goal_priority"))]
    pub struct GoalPriority;

    // CREATE TYPE goal_life_cycle AS ENUM ('SHORT_TERM', 'MEDIUM_TERM', 'LONG_TERM', 'LIFE_TIME');

    #[derive(diesel::sql_types::SqlType, diesel::query_builder::QueryId)]
    #[diesel(postgres_type(name = "goal_life_cycle"))]
    pub struct GoalLifeCycle;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::{GoalPeriodType, GoalStatus, GoalPriority, GoalLifeCycle};

    goals(id){
        id -> Int4,
        title -> Text,
        description-> Nullable<Text>,
        period_type -> GoalPeriodType,
        deadline_at -> Nullable<Timestamptz>,
        period_start -> Nullable<Timestamptz>,
        period_end -> Nullable<Timestamptz>,
        status -> GoalStatus,
        priority -> GoalPriority,
        target_value -> Nullable<Int4>,
        current_value -> Int4,
        unit -> Nullable<Text>,
        parent_goal_id -> Nullable<Int4>,
        goal_cycle -> GoalLifeCycle,
        create_date -> Timestamptz,
        update_date -> Timestamptz,
        delete_date -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;

    todos(id){
        id -> Uuid,
        title -> Text,
        description -> Nullable<Text>,
        parent_goal_id -> Nullable<Int4>,
        completed -> Bool,
        completed_date -> Nullable<Timestamptz>,
        create_date -> Timestamptz,
        update_date -> Timestamptz,
        delete_date -> Nullable<Timestamptz>,
    }
}
