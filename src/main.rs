use actix_web::{web, App, HttpServer};
use deno_core::anyhow::Error;
use deno_core::op;
use deno_core::Extension;
use std::rc::Rc;

#[op]
async fn op_read_file(path: String) -> Result<String, Error> {
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}

#[op]
async fn op_write_file(path: String, contents: String) -> Result<(), Error> {
    tokio::fs::write(path, contents).await?;
    Ok(())
}

#[op]
fn op_remove_file(path: String) -> Result<(), Error> {
    std::fs::remove_file(path)?;
    Ok(())
}

async fn run_js(code: &str) -> Result<(), Error> {
    let runjs_extension = Extension::builder()
        .ops(vec![
            op_read_file::decl(),
            op_write_file::decl(),
            op_remove_file::decl(),
        ])
        .build();
    let mut runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions: vec![runjs_extension],
        ..Default::default()
    });

    // Init runtime.
    runtime
        .execute_script("[matcha:runtime.js]", include_str!("./runtime.js"))
        .unwrap();

    // Load the javascript code as a module to allow async / await
    let spec = deno_core::resolve_url("file:///main.js").unwrap();
    let mod_id = runtime
        .load_main_module(&spec, Some(code.to_owned()))
        .await?;
    let result = runtime.mod_evaluate(mod_id);
    runtime.run_event_loop(false).await?;
    result.await?
}

async fn run_js_route(req_body: String) -> actix_web::Result<&'static str> {
    if let Err(e) = run_js(req_body.as_str()).await {
        return Err(actix_web::error::ErrorBadRequest(e.to_string()));
    }
    Ok("Enjoy a cup of matcha!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::post().to(run_js_route)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
