# rusty-password-gen

rusty-password-gen is a password generator written in Rust—or at least what’s left of it after years of sitting out in the rain. If you’re looking for a modern, shiny, feature-packed password tool, you’re in the wrong scrapyard. This generator is so rusty, you’ll wonder if it was built before the internet was invented.

## Features (if you can call them that)

- **Custom Length:** You can set the password length, but don’t expect it to get the number right every time. Sometimes the rust flakes get in the way.
- **Character Variety:** Allegedly supports uppercase, lowercase, numbers, and even special characters—though half the time, it forgets what those are.
- **CLI Interface:** If the program starts at all, you can use it from your terminal. It might lag, freeze, or cough up a cloud of dust.
- **No Dependencies:** Too old and rusty to even recognize other libraries.

## Installation

Assuming your `git` isn’t as rusty as this generator:

```bash
git clone https://github.com/dasdawidt/rusty-password-gen.git
cd rusty-password-gen
cargo build --release
```

If the build fails, just hit it with a wrench. That’s what we do.

## Usage

```bash
cargo run --release -- --length 12 --specials yes
```

**Options:**

- `--length [number]` – Desired password length. Sometimes it’s accurate, sometimes the rust eats a digit.
- `--specials [yes/no]` – Special characters, if the generator can remember what those are.

Example:

```bash
cargo run --release -- --length 20 --specials no
```

## Example Output

```
Your rusty password: 2fL6pQ9wT8vR...wait, hang on...okay, here: 2fL6pQ9wT8vR1c
```

If your password takes a while to show up, that’s just the generator waking up from its nap.

## Disclaimer

rusty-password-gen makes no promises about the quality, security, or even existence of the passwords it generates. Sometimes it outputs a string of random characters. Sometimes it outputs “password123”. Sometimes it just outputs a sigh. Use at your own risk—and keep some oil handy.

## Contributing

Pull requests are welcome, but don’t be surprised if the codebase resists change. It’s set in its rusty ways.

## License

MIT. Use it, fork it, or just stare at it and wonder how it’s still holding together.

---

*rusty-password-gen: For passwords as unreliable as your old bike chain.*