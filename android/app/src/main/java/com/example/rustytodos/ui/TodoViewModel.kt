package com.example.rustytodos.ui

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.example.rustytodos.data.Task
import com.example.rustytodos.data.TodoRepository
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch

// State for the UI
data class TodoUiState(val tasks: List<Task> = listOf())

class TodoViewModel(private val todoRepository: TodoRepository) : ViewModel() {

    val uiState: StateFlow<TodoUiState> =
        todoRepository.getAllTasksStream().map { TodoUiState(it) }
            .stateIn(
                scope = viewModelScope,
                started = SharingStarted.WhileSubscribed(TIMEOUT_MILLIS),
                initialValue = TodoUiState()
            )

    fun addTask(description: String) {
        viewModelScope.launch {
            todoRepository.insertTask(Task(description = description))
        }
    }

    fun toggleTaskCompletion(task: Task, completed: Boolean) {
        viewModelScope.launch {
            todoRepository.updateTask(task.copy(completed = completed))
        }
    }

    fun deleteTask(task: Task) {
        viewModelScope.launch {
            todoRepository.deleteTask(task)
        }
    }

    companion object {
        private const val TIMEOUT_MILLIS = 5_000L
    }
}
