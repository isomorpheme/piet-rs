use crate::command::Command;

error_chain! {
    errors {
        EmptyStack {
            description("the stack was popped, but it is empty")
        }

        CommandError(command: Command) {
            description("there was an error while executing a command")
            display("there was an error executing command: {:?}", command)
        }
    }
}
