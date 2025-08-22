## Instructions

To run the REPL:

1.  Install Rust.
   
2.  Open a terminal at the root of the package, which is the parent directory of the `Cargo.toml` file.
   
3.  Execute the following command:

    ```bash
    cargo run --release path_to_wat_file
    ```
    The above should build the executable, including downloading any dependencies and start the REPL
    This command will start the REPL. If the provided `.wat` file represents a well-formed module, as defined in the design and specification chapter, the REPL will prompt the user to enter a command.

### Accepted REPL Commands

The following commands are supported by the REPL:

| Command                     | Feature                                                                 |
| :-------------------------- | :---------------------------------------------------------------------- |
| `exports.functions.fn_name(args*)` | Executes the corresponding exported function and prints the result to the terminal. |
| `exports.globals.get(global_name)`   | Prints the value of the corresponding exported global variable to the terminal.    |
| `exports.memory.get(memory_name)`    | Prints the value of the corresponding exported memory to the terminal.     |

If there was an error with the command, such as there is no such export, or the command does not exist, the user will be prompted again. However, if during execution there was a logical error, the REPL will terminate. 

Examples can be found in the examples directory. Note that factorial program will return zero for factorials that result in anything bigger than 2,147,483,647, i.e. (2^32 -1)

Example Tic-Tac-Toe:
    ```bash
    cargo run --release examples/tictactoe.wat
    ```
    exports.functions.initGame()
    exports.functions.takeTurn(1,2)
    exports.memory.get(memory)
    exports.globals.get(x)
