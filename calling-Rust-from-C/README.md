# Calling rust from C

Generate `.h` file:

```bash
cbindgen --lang c --crate simplelib --output simplelib.h
```

Build library:

```bash
cargo build
```

Build the C app:

```bash
cl main.c .\target\debug\simplelib.lib ws2_32.lib ntdll.lib userenv.lib
```

Run the app:

```bash
.\app.exe
```