use std::sync::Arc;

use serde_json::{json, Value};
use warp::{Filter, reply::Json};

use crate::DbPool;
use crate::security::do_auth;
use crate::security::UserCtx;
use crate::with_db_pool;

pub fn todos_filter(db_pool: Arc<DbPool>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let todos_base = warp::path("todos");
        
    let list = todos_base
        .and(warp::get())
        .and(warp::path::end())
        .and(do_auth())
        .and(with_db_pool(db_pool.clone()))
        .and_then(todo_list);

    let get = todos_base
        .and(warp::get())
        .and(do_auth())
        .and(with_db_pool(db_pool.clone()))

        .and(warp::path::param())
        .and_then(todo_get);

    let create = todos_base
        .and(warp::post())
        .and(do_auth())
        .and(with_db_pool(db_pool.clone()))
        .and(warp::body::json())
        .and_then(todo_create);

    list.or(get).or(create)
}

async fn todo_list(_user_ctx: UserCtx,_db_pool: Arc<DbPool>) -> Result<Json, warp::Rejection> {

    let todos = json!([
        {"id":1,"task": "Learn Rust", "completed": true},
        {"id":2,"task": "Learn GraphQL", "completed": false}
    ]);

    let todos = warp::reply::json(&todos);
    
    Ok(todos)
}

async fn todo_get(_user_ctx: UserCtx,_db_pool: Arc<DbPool>, id: i64) -> Result<Json, warp::Rejection> {
    let todo = json!({"id":1,"task": "Learn Rust", "completed": true});
    let todo = warp::reply::json(&todo);
    Ok(todo)
}

async fn todo_create(_user_ctx: UserCtx,_db_pool: Arc<DbPool>, data: Value) -> Result<Json, warp::Rejection> {
    let todo = data;
    let todo = warp::reply::json(&todo);
    Ok(todo)
}




