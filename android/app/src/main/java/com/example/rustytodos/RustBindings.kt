package com.example.rustytodos

object RustBindings {
    external fun init(path: String)
    external fun list(): String
    external fun add(desc: String): String
    external fun complete(id: String): String
    external fun delete(id: String): String
    external fun edit(id: String, desc: String): String
    external fun uncomplete(id: String): String
    external fun version(): String
    // Loads the native library on initialization
    // The name passed as argument should map to the
    // original library name of the Rust project.
    // In this case it's rusty_todo_jni
    init { System.loadLibrary("rusty_todo_jni") }
}