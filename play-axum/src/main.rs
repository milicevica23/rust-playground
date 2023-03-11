use std::sync::{Mutex, Arc};

use axum::{Router,Server, routing::{get}, extract::State};
use sysinfo::{System, SystemExt, CpuExt};


#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(root_get))
    .route("/api/cpus",get(get_cpu))
    .with_state(AppState{
        sys: Arc::new(Mutex::new(System::new()))
    });
    let ser = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());
    ser.await.unwrap();
}

#[derive(Clone)]
struct AppState{
    sys: Arc<Mutex<System>>
}

async fn root_get() -> &'static str{
    "Hi from axum"
}

async fn get_cpu(State(state):State<AppState>) -> String {
    let mut s = String::new();
    use std::fmt::Write;

    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();
    for (i, cpu) in sys.cpus().iter().enumerate(){
        let i = i+1;

        let usage = cpu.cpu_usage();
        writeln!(&mut s, "CPU {i} usage {usage}%").unwrap();
    }
    s
}