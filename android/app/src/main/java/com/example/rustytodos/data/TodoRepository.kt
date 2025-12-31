package com.example.rustytodos.data

import kotlinx.coroutines.flow.Flow

interface TodoRepository {
    fun getAllTasksStream(): Flow<List<Task>>
    suspend fun insertTask(task: Task)
    suspend fun deleteTask(task: Task)
    suspend fun updateTask(task: Task)
    suspend fun getVersion(): String
}

class OfflineTodoRepository(private val taskDao: TaskDao) : TodoRepository {
    override fun getAllTasksStream(): Flow<List<Task>> = taskDao.getAllTasks()

    override suspend fun insertTask(task: Task) = taskDao.insertTask(task)

    override suspend fun deleteTask(task: Task) = taskDao.deleteTask(task)

    override suspend fun updateTask(task: Task) = taskDao.updateTask(task)

    override suspend fun getVersion(): String = "Offline Mode"
}
