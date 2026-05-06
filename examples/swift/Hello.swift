// Swift branch of cast's examples corpus. Pointed at by the
// `swift_examples` concept via cast::io::continues_in!.

func greet(name: String) -> String {
    return "Hello, \(name)!"
}

func main() {
    print(greet(name: "World"))
}

protocol Greeter {
    var language: String { get }
    var salutation: String { get }
}
