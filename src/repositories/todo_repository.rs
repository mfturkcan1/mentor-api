use crate::dto::todo_dto::TodoSelectDto;
use crate::models::{NewTodo, Todo};
use crate::schema::todos;
use diesel::QueryDsl;
use diesel::{ExpressionMethods, SelectableHelper};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub async fn insert_todo(
    conn: &mut AsyncPgConnection,
    todo: NewTodo,
) -> Result<TodoSelectDto, diesel::result::Error> {
    let result = diesel::insert_into(todos::table)
        .values(todo)
        .returning(Todo::as_returning())
        .get_result(conn)
        .await?;

    Ok(convert_todo_select_dto(result))
}

pub async fn get_todos(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<TodoSelectDto>, diesel::result::Error> {
    let results = todos::table
        .filter(todos::delete_date.is_null())
        .select(Todo::as_select())
        .get_results(conn)
        .await?;

    let todo_selected_vec = results
        .into_iter()
        .map(|t| convert_todo_select_dto(t))
        .collect();

    Ok(todo_selected_vec)
}

fn convert_todo_select_dto(todo: Todo) -> TodoSelectDto {
    TodoSelectDto {
        id: todo.id.to_string(),
        title: todo.title,
        description: todo.description,
        parent_goal_id: todo.parent_goal_id,
        completed: todo.completed,
        completed_date: todo.completed_date,
        deadline_date: todo.deadline_date,
        create_date: todo.create_date,
        update_date: todo.update_date,
        delete_date: todo.delete_date,
    }
}
