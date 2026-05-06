// Kotlin branch of cast's examples corpus. Pointed at by the
// `kotlin_examples` concept via cast::io::continues_in!.

fun greet(name: String): String = "Hello, $name!"

fun main() {
    println(greet("World"))
}

class Greeter(val language: String, val salutation: String) {
    fun render(name: String): String = "$salutation, $name!"
}
