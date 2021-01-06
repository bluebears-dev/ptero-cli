use fern::{
    colors::{Color, ColoredLevelConfig},
    Dispatch,
};
use log::{Level, LevelFilter};

/// Converts verbosity number to [LevelFilter](log::LevelFilter) enum.
/// Used for configuring the logging level.
/// # Arguments
///
/// * `verbosity` - verbosity level described by number `u8`
///
/// # Examples
/// ## Converts verbosity number
/// ```
/// use ptero::log::verbosity_to_level_filter;
/// use log::{LevelFilter};
///
/// assert_eq!(verbosity_to_level_filter(0), LevelFilter::Off);
/// assert_eq!(verbosity_to_level_filter(1), LevelFilter::Warn);
/// assert_eq!(verbosity_to_level_filter(2), LevelFilter::Info);
/// assert_eq!(verbosity_to_level_filter(3), LevelFilter::Debug);
/// ```
/// ## Unrecognized verbosity defaults to trace
/// ```
/// use ptero::log::verbosity_to_level_filter;
/// use log::{LevelFilter};
///
/// assert_eq!(verbosity_to_level_filter(4), LevelFilter::Trace);
/// assert_eq!(verbosity_to_level_filter(100), LevelFilter::Trace);
/// assert_eq!(verbosity_to_level_filter(255), LevelFilter::Trace);
/// ```
pub fn verbosity_to_level_filter(verbosity: u8) -> LevelFilter {
    match verbosity {
        0 => LevelFilter::Off,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    }
}

/// Returns pre-configured [ColoredLevelConfig](fern::colors::ColoredLevelConfig) used to color
/// logging level.
fn get_logging_colors() -> ColoredLevelConfig {
    ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::BrightYellow)
        .debug(Color::Magenta)
        .trace(Color::BrightBlack)
}

/// Returns text which will be shown before the message. Used only in stdout formatter.
fn get_level_text(level: &Level) -> &str {
    match level {
        Level::Error => "ERROR",
        Level::Warn => " WARN",
        Level::Info => " INFO",
        Level::Debug => "DEBUG",
        Level::Trace => "TRACE",
    }
}

/// Returns pre-configured stdout logger.
/// It only shows info relevant to user like message and logging level.
/// Uses coloring unlike file logger.
///
/// # Arguments
/// * `log_level` - level filter which is used to restrict amount of logs to user
pub fn get_stdout_logger() -> Dispatch {
    let colors = get_logging_colors();

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}{level_txt}\x1B[0m  {message}",
                level_txt = get_level_text(&record.level()),
                color_line =
                    format_args!("\x1B[{}m", colors.get_color(&record.level()).to_fg_str()),
                message = message,
            ));
        })
        .chain(std::io::stderr())
}

/// Returns pre-configured file logger.
/// This logger does not used coloring and adds additional info like date time or module path.
/// It doesn't restrict logging - saves everything beginning from `TRACE` level.
///
/// # Arguments
/// * `log_path` - path to the log file which will be used to store logs
pub fn get_file_logger(log_path: &str) -> Dispatch {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] - {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                &record.target(),
                &record.level(),
                message,
            ));
        })
        .chain(fern::log_file(&log_path).unwrap())
}
