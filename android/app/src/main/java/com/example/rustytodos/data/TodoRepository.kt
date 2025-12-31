package com.example.rustytodos.data

import kotlinx.coroutines.flow.Flow

interface TodoRepository {
    fun getAllTasksStream(): Flow<List<Task>>
    suspend fun insertTask(task: Task)
    suspend fun deleteTask(task: Task)
    suspend fun updateTask(task: Task)
    suspend fun markTask(task: Task)
    suspend fun getVersion(): String
}

