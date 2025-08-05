use std::collections::HashMap;
use crate::request::Request;
use crate::response::Response;

pub type Handler = Box<dyn Fn(&Request) -> Response + Send + Sync>;

pub struct Route {
    pub method: String,
    pub path: String,
    pub handler: Handler,
}

pub struct Router {
    routes: Vec<Route>,
    static_routes: HashMap<String, String>,
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: Vec::new(),
            static_routes: HashMap::new(),
        }
    }

    pub fn get<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.routes.push(Route {
            method: "GET".to_string(),
            path: path.to_string(),
            handler: Box::new(handler),
        });
    }

    pub fn post<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.routes.push(Route {
            method: "POST".to_string(),
            path: path.to_string(),
            handler: Box::new(handler),
        });
    }

    pub fn put<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.routes.push(Route {
            method: "PUT".to_string(),
            path: path.to_string(),
            handler: Box::new(handler),
        });
    }

    pub fn delete<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.routes.push(Route {
            method: "DELETE".to_string(),
            path: path.to_string(),
            handler: Box::new(handler),
        });
    }

    pub fn static_files(&mut self, route_path: &str, file_path: &str) {
        self.static_routes.insert(route_path.to_string(), file_path.to_string());
    }

    pub fn handle_request(&self, request: &Request) -> Response {
        for route in &self.routes {
            if route.method == request.method && self.path_matches(&route.path, &request.path) {
                return (route.handler)(request);
            }
        }

        for (route_path, file_path) in &self.static_routes {
            if request.path.starts_with(route_path) {
                return self.serve_static_file(&request.path, file_path);
            }
        }

        Response::new(404, "Not Found", "Page not found")
    }

    fn path_matches(&self, route_path: &str, request_path: &str) -> bool {
        route_path == request_path
    }

    fn serve_static_file(&self, request_path: &str, base_path: &str) -> Response {
        use std::fs;
        use std::path::Path;

        let file_path = format!("{}/{}", base_path, request_path.trim_start_matches('/'));
        let path = Path::new(&file_path);

        if path.exists() && path.is_file() {
            match fs::read_to_string(path) {
                Ok(content) => {
                    let mut response = Response::new(200, "OK", &content);

                    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                        let content_type = match ext {
                            "html" => "text/html",
                            "css" => "text/css",
                            "js" => "application/javascript",
                            "json" => "application/json",
                            "png" => "image/png",
                            "jpg" | "jpeg" => "image/jpeg",
                            "gif" => "image/gif",
                            _ => "text/plain",
                        };
                        response.set_header("Content-Type", content_type);
                    }

                    response
                }
                Err(_) => Response::new(500, "Internal Server Error", "Could not read the file"),
            }
        } else {
            Response::new(404, "Not Found", "File not found")
        }
    }
}

#[macro_export]
macro_rules! route {
    ($router:expr, GET $path:literal => $handler:expr) => {
        $router.get($path, $handler);
    };
    ($router:expr, POST $path:literal => $handler:expr) => {
        $router.post($path, $handler);
    };
    ($router:expr, PUT $path:literal => $handler:expr) => {
        $router.put($path, $handler);
    };
    ($router:expr, DELETE $path:literal => $handler:expr) => {
        $router.delete($path, $handler);
    };
}