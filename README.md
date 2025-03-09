This is a simple interactive lab that lets you play around with the
Perceptron, a remarkably elegant neural network invented by
Frank Rosenblatt in 1958.

The program uses the algorithm described in page 51 of [Why Machines Learn][]
by Anil Ananthaswamy.

## Example

Below is a screencast of the program converging upon a solution.

The Perceptron is effectively trying to draw a line that separates the green
dots from the purple ones. It does this by iterating through the dots and
updating the line whenever it finds a dot that's on the wrong side of it.

https://github.com/user-attachments/assets/1b49ed9b-d010-45ed-9e1f-53379834a50d

## Quick start

To run it, you'll need to [install Rust](https://www.rust-lang.org/tools/install)
and run:

```
cargo run
```

Once the window opens, you can press `H` for help.

[Why Machines Learn]: http://anilananthaswamy.com/why-machines-learn

## License

Everything in this repository is licensed under [CC0 1.0 Universal](./LICENSE.md) (public domain).
