package com.example.rustytodos.data

import android.content.Context
import android.util.Log
import com.example.rustytodos.RustBindings
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import org.json.JSONArray

class RustTodoRepository(context: Context) : TodoRepository {

    // Backing flow to emit updates
    private val _tasksFlow = MutableStateFlow<List<Task>>(emptyList())

    init {
        // Initialize Rust backend with the files directory
        val path = context.filesDir.absolutePath
        Log.d("RustTodoRepository", "Initializing Rust with path: $path")
        RustBindings.init(path)
        
        // Initial load
        refreshTasks()
    }

    override fun getAllTasksStream(): Flow<List<Task>> = _tasksFlow.asStateFlow()

    override suspend fun insertTask(task: Task) {
        Log.d("RustTodoRepository", "Adding task: ${task.description}")
        RustBindings.add(task.description)
        refreshTasks()
    }

    override suspend fun deleteTask(task: Task) {
        Log.d("RustTodoRepository", "Deleting task id: ${task.id}")
        RustBindings.delete(task.id.toLong())
        refreshTasks()
    }

    override suspend fun updateTask(task: Task) {
        Log.d("RustTodoRepository", "Updating task id: ${task.id}")
        RustBindings.edit(task.id.toLong(), task.description)
        val id = task.id.toLong()
        if (task.completed) {
             RustBindings.complete(id)
        } else {
             RustBindings.uncomplete(id)
        }
        refreshTasks()
    }

    override suspend fun getVersion(): String {
        return try {
            RustBindings.version()
        } catch (e: Exception) {
            Log.e("RustTodoRepository", "Error fetching version", e)
            "Unknown"
        }
    }

    private fun refreshTasks() {
        try {
            val jsonString = RustBindings.list()
            Log.d("RustTodoRepository", "Raw JSON from Rust: $jsonString")
            
            val jsonArray = JSONArray(jsonString)
            val tasks = mutableListOf<Task>()
            
            for (i in 0 until jsonArray.length()) {
                val obj = jsonArray.getJSONObject(i)
                tasks.add(Task(
                    id = i + 1, // 1-based ID to match Rust CLI behavior
                    description = obj.getString("description"),
                    completed = obj.getBoolean("completed")
                ))
            }
            
            _tasksFlow.value = tasks
        } catch (e: Exception) {
            Log.e("RustTodoRepository", "Error parsing tasks", e)
        }
    }
}
