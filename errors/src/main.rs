mod unrecoverable;
mod recoverable;
mod to_panic_or_not;

fn main() {
    recoverable::main();
    to_panic_or_not::main();
    unrecoverable::main();
}
