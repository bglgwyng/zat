package com.example

class Service(val name: String) {
    private val secret: String = ""
    var value: Int = 0

    fun getName(): String {
        return name
    }

    private fun internal() {
        // hidden
    }

    companion object {
        fun create(name: String): Service {
            return Service(name)
        }
    }
}

object Singleton {
    fun doWork() {
        println("working")
    }
}

enum class Color {
    RED,
    GREEN,
    BLUE;

    fun label(): String {
        return name.lowercase()
    }
}

typealias Callback = (String) -> Unit

fun greet(name: String): String {
    return "Hello, $name!"
}

val MAX_SIZE = 1024
