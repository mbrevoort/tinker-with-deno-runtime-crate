use std::ffi::CStr;
use std::rc::Rc;
use std::sync::Arc;

use deno_runtime::permissions::Permissions;
use deno_runtime::permissions::PermissionsOptions;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_core::error::AnyError;
use deno_core::resolve_url_or_path;
use deno_web::BlobStore; 
use deno_broadcast_channel::InMemoryBroadcastChannel;

#[no_mangle]
pub extern "C" fn run(path: *const libc::c_char) {
    let buf_path = unsafe { CStr::from_ptr(path).to_bytes() };
    let str_path = String::from_utf8(buf_path.to_vec()).unwrap();
    let source_path: &str = &str_path[..];
    run_local(source_path);
}

pub fn run_local(path: &str) {
  let source_path: &str = &path[..];

  let runtime = tokio::runtime::Builder::new_current_thread()
  .enable_io()
  .enable_time()
  .max_blocking_threads(32)
  .build()
  .unwrap();

  let local = tokio::task::LocalSet::new();
  if let Err(e) = local.block_on(&runtime, run_source(&source_path)) {
    eprintln!("error: {:?}", e);
    std::process::exit(1);
  }
}

async fn run_source(source_path: &str) -> Result<(), AnyError> {
  let main_module = resolve_url_or_path(source_path)?;

  // Here's where we can set permissions, like only allowing access to `dinoipsum.herokuapp.com`
  let opts = PermissionsOptions{
    allow_env: None,
    allow_hrtime: false,
    allow_net: Some(vec!["dinoipsum.herokuapp.com".to_string()]),
    allow_ffi: None,
    allow_read: None,
    allow_run: None,
    allow_write: None,
    prompt: false,  
  };
  let permissions = Permissions::from_options(&opts);

  // That's a whole-lot-a worker options, some are obvious, others less so
  let options = WorkerOptions {
    apply_source_maps: false,
    user_agent: "theno".to_string(),
    args: vec![],
    debug_flag: false,
    unstable: false,
    enable_testing_features: false,
    unsafely_ignore_certificate_errors: None,
    root_cert_store: None,
    seed: None,
    js_error_create_fn: None,
    create_web_worker_cb: Arc::new(|_| todo!("Sadly, web workers are not supported here")),
    maybe_inspector_server: None,
    should_break_on_first_statement: false,
    module_loader: Rc::new(deno_core::FsModuleLoader),
    runtime_version: "¯\\_(ツ)_/¯".to_string(),
    ts_version: "¯\\_(ツ)_/¯".to_string(),
    no_color: false,
    get_error_class_fn: None,
    location: None,
    origin_storage_dir: None,
    blob_store: BlobStore::default(),
    broadcast_channel: InMemoryBroadcastChannel::default(),
    shared_array_buffer_store: None,
    cpu_count: 1,
  };
  
  // execute the source in the main worker
  let mut worker = MainWorker::from_options(main_module.clone(), permissions, &options);
  worker.bootstrap(&options);
  worker.execute_module(&main_module).await?;
  worker.execute_script(source_path, "window.dispatchEvent(new Event('load'))")?;
  worker.run_event_loop(true).await?;
  worker.execute_script(source_path, "window.dispatchEvent(new Event('unload'))")?;
  return Ok(());
}
