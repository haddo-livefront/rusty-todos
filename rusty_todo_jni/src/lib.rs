use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jstring, jint, jlong};
use std::ffi::{CStr, CString};
use rusty_todo::{AppContext, load_tasks, Command, CommandResult};
use std::sync::Mutex;
use lazy_static::lazy_static;

// Global AppContext to persist state between JNI calls
// Note: In a real Android app, be careful with lifecycle.
// For now, we'll lazily initialize it. We might need a way to reload/re-init.
lazy_static! {
    static ref APP_CONTEXT: Mutex<Option<AppContext>> = Mutex::new(None);
}

// Allow non-snake case for JNI functions
#[allow(non_snake_case)]
#[no_mangle] // This keeps Rust from "mangling" the name so it is unique (crate).
pub extern "system" fn Java_com_example_rustytodos_RustBindings_init(
    mut env: JNIEnv,
    _class: JClass,
    path: JString,
) {
    let path: String = env.get_string(&path).expect("Couldn't get java string!").into();
    
    // Set current directory to the path provided by Android
    if let Err(e) = std::env::set_current_dir(&path) {
        // Log error? For now print to stderr
        eprintln!("Failed to set current dir to {}: {}", path, e);
    }
    
    // Initialize context
    let tasks = load_tasks().unwrap_or_else(|_| Vec::new());
    let mut context = APP_CONTEXT.lock().unwrap();
    *context = Some(AppContext::new(tasks));
}

fn execute_command(command: Command) -> String {
    let mut context_guard = APP_CONTEXT.lock().unwrap();
    
    if let Some(ref mut context) = *context_guard {
        match context.execute(command) {
            Ok(result) => match result {
                CommandResult::Message(msg) => msg,
                CommandResult::Tasks(tasks) => {
                    serde_json::to_string(&tasks).unwrap_or_else(|_| "[]".to_string())
                }
            },
            Err(e) => format!("Error: {}", e),
        }
    } else {
        "Error: AppContext not initialized. Call init() first.".to_string()
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_com_example_rustytodos_RustBindings_list(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    let output = execute_command(Command::List);
    let output_ptr = CString::new(output).unwrap();
    env.new_string(output_ptr.to_str().unwrap()).expect("Couldn't create java string!").into_raw()
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_com_example_rustytodos_RustBindings_add(
    mut env: JNIEnv,
    _class: JClass,
    desc: JString,
) -> jstring {
    let desc: String = env.get_string(&desc).expect("Couldn't get java string!").into();
    let output = execute_command(Command::Add(desc));
    let output_ptr = CString::new(output).unwrap();
    env.new_string(output_ptr.to_str().unwrap()).expect("Couldn't create java string!").into_raw()
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_com_example_rustytodos_RustBindings_complete(
    env: JNIEnv,
    _class: JClass,
    id: jlong,
) -> jstring {
    // Cast jlong (i64) to usize. Be careful with overflow/casting.
    let id_usize = id as usize;
    let output = execute_command(Command::Complete(id_usize));
    let output_ptr = CString::new(output).unwrap();
    env.new_string(output_ptr.to_str().unwrap()).expect("Couldn't create java string!").into_raw()
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_com_example_rustytodos_RustBindings_version(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    let output = execute_command(Command::Version);
    let output_ptr = CString::new(output).unwrap();
    env.new_string(output_ptr.to_str().unwrap()).expect("Couldn't create java string!").into_raw()
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_com_example_rustytodos_RustBindings_delete(
    env: JNIEnv,
    _class: JClass,
    id: jlong,
) -> jstring {
    let id_usize = id as usize;
    let output = execute_command(Command::Delete(id_usize));
    let output_ptr = CString::new(output).unwrap();
    env.new_string(output_ptr.to_str().unwrap()).expect("Couldn't create java string!").into_raw()
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn Java_com_example_rustytodos_RustBindings_edit(
    mut env: JNIEnv,
    _class: JClass,
    id: jlong,
    desc: JString,
) -> jstring {
    let id_usize = id as usize;
    let desc: String = env.get_string(&desc).expect("Couldn't get java string!").into();
    let output = execute_command(Command::Edit(id_usize, desc));
    let output_ptr = CString::new(output).unwrap();
    env.new_string(output_ptr.to_str().unwrap()).expect("Couldn't create java string!").into_raw()
}
