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
data class TodoUiState(
    val tasks: List<Task> = listOf(),
    val version: String = ""
)

class TodoViewModel(private val todoRepository: TodoRepository) : ViewModel() {

    private val _version = kotlinx.coroutines.flow.MutableStateFlow("")

    val uiState: StateFlow<TodoUiState> =
        kotlinx.coroutines.flow.combine(
            todoRepository.getAllTasksStream(),
            _version
        ) { tasks, ver ->
            TodoUiState(tasks, ver)
        }.stateIn(
            scope = viewModelScope,
            started = SharingStarted.WhileSubscribed(TIMEOUT_MILLIS),
            initialValue = TodoUiState()
        )

    init {
        viewModelScope.launch {
            _version.value = todoRepository.getVersion()
        }
    }

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

    fun editTask(task: Task, newDescription: String) {
        viewModelScope.launch {
            // Use updateTask which we modified to handle editing
            todoRepository.updateTask(task.copy(description = newDescription))
        }
    }

    companion object {
        private const val TIMEOUT_MILLIS = 5_000L
    }
}
