// TypeScript branch of cast's examples corpus. Pointed at by the
// `typescript_examples` concept via cast::io::continues_in!.

function greet(name: string): string {
    return `Hello, ${name}!`;
}

function main(): void {
    console.log(greet("World"));
}

interface Greeter {
    language: string;
    salutation: string;
}

const enGreeter: Greeter = { language: "en", salutation: "Hello" };
