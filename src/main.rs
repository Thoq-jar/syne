use syne::{listen_with_router, route};
use syne::router::Router;
use syne::request::Request;
use syne::response::Response;
use syne::template::{TemplateEngine, TemplateContext};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
    age: u32,
}

#[derive(Clone, Debug)]
struct Post {
    id: u32,
    title: String,
    content: String,
    author: String,
}

struct AppState {
    template_engine: TemplateEngine,
    users: Vec<User>,
    posts: Vec<Post>,
}

fn main() {
    let app_state = Arc::new(Mutex::new(AppState {
        template_engine: TemplateEngine::new("templates"),
        users: sample_users(),
        posts: sample_posts(),
    }));

    let mut router = Router::new();

    {
        let state = Arc::clone(&app_state);
        route!(router, GET "/" => move |req| home_handler(req, Arc::clone(&state)));
    }
    {
        let state = Arc::clone(&app_state);
        route!(router, GET "/users" => move |req| users_handler(req, Arc::clone(&state)));
    }
    {
        let state = Arc::clone(&app_state);
        route!(router, GET "/posts" => move |req| posts_handler(req, Arc::clone(&state)));
    }
    {
        let state = Arc::clone(&app_state);
        route!(router, GET "/about" => move |req| about_handler(req, Arc::clone(&state)));
    }
    {
        let state = Arc::clone(&app_state);
        route!(router, GET "/api/users" => move |req| api_users_handler(req, Arc::clone(&state)));
    }
    {
        let state = Arc::clone(&app_state);
        route!(router, GET "/api/posts" => move |req| api_posts_handler(req, Arc::clone(&state)));
    }
    {
        let state = Arc::clone(&app_state);
        route!(router, POST "/api/users" => move |req| api_create_user_handler(req, Arc::clone(&state)));
    }
    router.static_files("/static", "./static");

    listen_with_router("127.0.0.1", 9595, true, router);
}

fn sample_users() -> Vec<User> {
    vec![
        User { id: 1, name: "Alice Johnson".to_string(), email: "alice@example.com".to_string(), age: 28 },
        User { id: 2, name: "Bob Smith".to_string(), email: "bob@example.com".to_string(), age: 34 },
        User { id: 3, name: "Charlie Brown".to_string(), email: "charlie@example.com".to_string(), age: 25 },
        User { id: 4, name: "Diana Prince".to_string(), email: "diana@example.com".to_string(), age: 30 },
        User { id: 5, name: "Eve Wilson".to_string(), email: "eve@example.com".to_string(), age: 27 },
    ]
}

fn sample_posts() -> Vec<Post> {
    vec![
        Post {
            id: 1,
            title: "Getting Started with Rust".to_string(),
            content: "Rust is a systems programming language that runs blazingly fast...".to_string(),
            author: "Alice Johnson".to_string()
        },
        Post {
            id: 2,
            title: "Web Development in 2024".to_string(),
            content: "The landscape of web development continues to evolve rapidly...".to_string(),
            author: "Bob Smith".to_string()
        },
        Post {
            id: 3,
            title: "Building Web Frameworks".to_string(),
            content: "Creating a web framework from scratch teaches you fundamental concepts...".to_string(),
            author: "Charlie Brown".to_string()
        },
    ]
}

fn home_handler(_req: &Request, app_state: Arc<Mutex<AppState>>) -> Response {
    let mut state = app_state.lock().unwrap();
    let mut context = TemplateContext::new();

    context.set("title", "Home");
    context.set("user_count", &state.users.len().to_string());
    context.set("post_count", &state.posts.len().to_string());
    context.set("version", "0.1.0");

    match state.template_engine.render("home", &context) {
        Ok(content) => {
            let mut base_context = HashMap::new();
            base_context.insert("title".to_string(), "Home".to_string());
            base_context.insert("content".to_string(), content);

            match state.template_engine.render_simple("base", &base_context) {
                Ok(html) => Response::html(200, &html),
                Err(e) => {
                    println!("Template error: {e:?}");
                    Response::new(500, "Internal Server Error", "Template error")
                }
            }
        }
        Err(e) => {
            println!("Template wasn't found: {e:?}");
            Response::new(500, "Internal Server Error", "Template not found")
        }
    }
}

fn users_handler(_req: &Request, app_state: Arc<Mutex<AppState>>) -> Response {
    let mut state = app_state.lock().unwrap();
    let mut context = TemplateContext::new();

    context.set("title", "Users");
    context.set("user_count", &state.users.len().to_string());

    let user_names: Vec<String> = state.users.iter().map(|u| u.name.clone()).collect();
    context.set_list("users", user_names);

    match state.template_engine.render("users", &context) {
        Ok(content) => {
            let mut base_context = HashMap::new();
            base_context.insert("title".to_string(), "Users".to_string());
            base_context.insert("content".to_string(), content);

            match state.template_engine.render_simple("base", &base_context) {
                Ok(html) => Response::html(200, &html),
                Err(_) => Response::new(500, "Internal Server Error", "Template error")
            }
        }
        Err(_) => Response::new(500, "Internal Server Error", "Template not found")
    }
}

fn posts_handler(_req: &Request, app_state: Arc<Mutex<AppState>>) -> Response {
    let mut state = app_state.lock().unwrap();
    let mut context = TemplateContext::new();

    context.set("title", "Posts");
    context.set("post_count", &state.posts.len().to_string());

    let post_titles: Vec<String> = state.posts.iter().map(|p| p.title.clone()).collect();
    context.set_list("posts", post_titles);

    match state.template_engine.render("posts", &context) {
        Ok(content) => {
            let mut base_context = HashMap::new();
            base_context.insert("title".to_string(), "Posts".to_string());
            base_context.insert("content".to_string(), content);

            match state.template_engine.render_simple("base", &base_context) {
                Ok(html) => Response::html(200, &html),
                Err(_) => Response::new(500, "Internal Server Error", "Template error")
            }
        }
        Err(_) => Response::new(500, "Internal Server Error", "Template not found")
    }
}

fn about_handler(_req: &Request, app_state: Arc<Mutex<AppState>>) -> Response {
    let mut state = app_state.lock().unwrap();
    let mut context = TemplateContext::new();

    context.set("title", "About");
    context.set("rust_version", "1.89.0");

    match state.template_engine.render("about", &context) {
        Ok(content) => {
            let mut base_context = HashMap::new();
            base_context.insert("title".to_string(), "About".to_string());
            base_context.insert("content".to_string(), content);

            match state.template_engine.render_simple("base", &base_context) {
                Ok(html) => Response::html(200, &html),
                Err(_) => Response::new(500, "Internal Server Error", "Template error")
            }
        }
        Err(_) => Response::new(500, "Internal Server Error", "Template not found")
    }
}

fn api_users_handler(_req: &Request, app_state: Arc<Mutex<AppState>>) -> Response {
    let state = app_state.lock().unwrap();

    let users_json: Vec<String> = state.users.iter().map(|user| {
        format!(
            r#"{{"id": {}, "name": "{}", "email": "{}", "age": {}}}"#,
            user.id, user.name, user.email, user.age
        )
    }).collect();

    let json_response = format!(
        r#"{{"users": [{}], "total": {}}}"#,
        users_json.join(","),
        state.users.len()
    );

    Response::json(200, &json_response)
}

fn api_posts_handler(_req: &Request, app_state: Arc<Mutex<AppState>>) -> Response {
    let state = app_state.lock().unwrap();

    let posts_json: Vec<String> = state.posts.iter().map(|post| {
        format!(
            r#"{{"id": {}, "title": "{}", "author": "{}", "content": "{}"}}"#,
            post.id, post.title, post.author,
            post.content.chars().take(100).collect::<String>() + "..."
        )
    }).collect();

    let json_response = format!(
        r#"{{"posts": [{}], "total": {}}}"#,
        posts_json.join(","),
        state.posts.len()
    );

    Response::json(200, &json_response)
}

fn api_create_user_handler(_req: &Request, _app_state: Arc<Mutex<AppState>>) -> Response {
    let response = r#"{"message": "User created successfully", "id": 6}"#;
    Response::json(201, response)
}
