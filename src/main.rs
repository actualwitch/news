#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        body::Body as AxumBody,
        extract::{Path, State},
        http::Request,
        response::IntoResponse,
        routing::{get, post},
        Router,
    };
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
    use news::app::*;
    use news::model::ssr::AppState;
    use opentelemetry::global::ObjectSafeSpan;
    use opentelemetry::trace::{SpanKind, Status};
    use opentelemetry::{global, trace::Tracer};
    use opentelemetry_sdk::propagation::TraceContextPropagator;
    use opentelemetry_sdk::trace::SdkTracerProvider;
    use opentelemetry_stdout::SpanExporter;
    use sqlx::PgPool;

    let database_url = std::option_env!("DATABASE_URL").expect("Missing DATABASE_URL");
    let pool = PgPool::connect(database_url)
        .await
        .expect("Failed to create Postgres pool");

    if let Err(e) = sqlx::migrate!().run(&pool).await {
        eprintln!("{e:?}");
    } else {
        log!("Migrations complete.");
    }

    async fn server_fn_handler(
        State(app_state): State<AppState>,
        path: Path<String>,
        request: Request<AxumBody>,
    ) -> impl IntoResponse {
        log!("{:?}", path);

        handle_server_fns_with_context(
            move || {
                provide_context(app_state.pool.clone());
            },
            request,
        )
        .await
    }

    pub async fn leptos_routes_handler(
        state: State<AppState>,
        request: Request<AxumBody>,
    ) -> axum::response::Response {
        let State(app_state) = state.clone();
        let handler = leptos_axum::render_route_with_context(
            app_state.routes.clone(),
            move || {
                provide_context(app_state.pool.clone());
            },
            move || shell(app_state.leptos_options.clone()),
        );
        handler(state, request).await.into_response()
    }

    global::set_text_map_propagator(TraceContextPropagator::new());
    let provider = SdkTracerProvider::builder()
        .with_simple_exporter(SpanExporter::default())
        .build();
    global::set_tracer_provider(provider);

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app_state = AppState {
        leptos_options,
        pool: pool.clone(),
        routes: routes.clone(),
    };

    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .expect("server failed");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
