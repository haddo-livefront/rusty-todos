package com.example.rustytodos.data

import kotlinx.coroutines.flow.Flow

class RoomTodoRepository(private val taskDao: TaskDao) : TodoRepository {
    override fun getAllTasksStream(): Flow<List<Task>> = taskDao.getAllTasks()

    override suspend fun insertTask(task: Task) = taskDao.insertTask(task)

    override suspend fun deleteTask(task: Task) = taskDao.deleteTask(task)

    override suspend fun updateTask(task: Task) = taskDao.updateTask(task)

    override suspend fun markTask(task: Task) = taskDao.updateTask(task)

    override suspend fun getVersion(): String = "Room Offline Mode"
}
