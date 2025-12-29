package com.example.rustytodos

import android.app.Application
import com.example.rustytodos.data.AppContainer
import com.example.rustytodos.data.AppDataContainer

class RustyTodosApplication : Application() {
    lateinit var container: AppContainer

    override fun onCreate() {
        super.onCreate()
        container = AppDataContainer(this)
    }
}
