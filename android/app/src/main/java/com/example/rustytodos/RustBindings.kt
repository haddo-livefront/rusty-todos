package com.example.rustytodos

object RustBindings {
    external fun init(path: String)
    external fun list(): String
    external fun add(desc: String): String
    external fun complete(id: Long): String
    external fun version(): String
    init { System.loadLibrary("rusty_todo_jni") }
}