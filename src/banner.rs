const LOGO: &[&str] = &[
    " ██████╗ ██╗ ███████╗ ███╗   ███╗",
    " ╚══███║ ╚═╝ ██╔════╝ ████╗ ████║",
    "   ███╔╝ ██╗ ███████╗ ██╔████╔██║",
    "  ███╔╝  ██║ ╚════██║ ██║╚██╔╝██║",
    " ██████╗ ██║ ███████║ ██║ ╚═╝ ██║",
    " ╚═════╝ ╚═╝ ╚══════╝ ╚═╝     ╚═╝",
];

const COLORS: &[&str] = &[
    "\x1b[38;5;27m", // blue
    "\x1b[38;5;33m", // dodger blue
    "\x1b[38;5;39m", // deep sky blue
    "\x1b[38;5;44m", // dark turquoise
    "\x1b[38;5;49m", // spring green
    "\x1b[38;5;48m", // medium spring green
];

const RESET: &str = "\x1b[0m";

// Compile-time assertion to ensure LOGO and COLORS have the same length
const _: () = assert!(
    LOGO.len() == COLORS.len(),
    "LOGO and COLORS must have the same length"
);

pub fn print_banner() {
    println!();
    let last = LOGO.len() - 1;
    for (i, (line, color)) in LOGO.iter().zip(COLORS).enumerate() {
        if i == last {
            println!("{color}{line}{RESET}  v{}", env!("CARGO_PKG_VERSION"));
        } else {
            println!("{color}{line}{RESET}");
        }
    }
    println!();
}
