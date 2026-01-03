package com.example.rustytodos.data

import androidx.room.Entity
import androidx.room.PrimaryKey

@Entity(tableName = "tasks")
data class Task(
    @PrimaryKey(autoGenerate = false)
    val id: String,
    val description: String,
    val completed: Boolean = false
)
