package com.example.rustytodos.data

import android.content.Context

interface AppContainer {
    val todoRepository: TodoRepository
    val roomTodoRepository: TodoRepository
}

class AppDataContainer(private val context: Context) : AppContainer {
    override val todoRepository: TodoRepository by lazy {
        RustTodoRepository(context)
    }

    override val roomTodoRepository: TodoRepository by lazy {
        RoomTodoRepository(AppDatabase.getDatabase(context).taskDao())
    }
}
