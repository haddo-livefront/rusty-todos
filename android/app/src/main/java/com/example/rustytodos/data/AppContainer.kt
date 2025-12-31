package com.example.rustytodos.data

import android.content.Context

interface AppContainer {
    val todoRepository: TodoRepository
}

class AppDataContainer(private val context: Context) : AppContainer {
    override val todoRepository: TodoRepository by lazy {
        RustTodoRepository(context)
    }
}
