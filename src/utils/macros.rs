macro_rules! print_flush {

    ($($args:tt)*) => {
        print!($($args)*);
        ::std::io::Write::flush(&mut ::std::io::stdout())?;
    };
}
