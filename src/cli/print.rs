#[macro_export]
macro_rules! success {
    ( $pattern:expr, $( $x:expr ),* ) => {
        {
            use colored::*;
            print!("{}", "SUCCESS".bold().green());
            println!($pattern, $( $x ),*);
        }
    };
    ( $msg:expr ) => {
        {
            use colored::*;
            print!("{} ", "SUCCESS".bold().green());
            println!($msg);
        }
    };
}
