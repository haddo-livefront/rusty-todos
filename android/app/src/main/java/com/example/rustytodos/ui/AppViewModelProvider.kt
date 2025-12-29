package com.example.rustytodos.ui

import androidx.lifecycle.ViewModelProvider
import androidx.lifecycle.viewmodel.CreationExtras
import androidx.lifecycle.viewmodel.initializer
import androidx.lifecycle.viewmodel.viewModelFactory
import com.example.rustytodos.RustyTodosApplication

object AppViewModelProvider {
    val Factory = viewModelFactory {
        initializer {
            TodoViewModel(rustyTodosApplication().container.todoRepository)
        }
    }
}

fun CreationExtras.rustyTodosApplication(): RustyTodosApplication =
    (this[ViewModelProvider.AndroidViewModelFactory.APPLICATION_KEY] as RustyTodosApplication)
