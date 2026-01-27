/// Includes a UTF-8 encoded file under the "doc" directory as a string.
#[macro_export]
#[doc(hidden)]
macro_rules! include_doc {
    ( $( $name:expr ),+ ) => {
        $crate::include_file!("..", "doc", $( $name ),+ )
    };
}

/// Includes a UTF-8 encoded file as a string.
#[macro_export]
#[doc(hidden)]
macro_rules! include_file {
    ( $( $name:expr ),+ ) => {
        include_str!($crate::build_path!( $( $name ),+ ))
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! build_path {
    ( $name:expr $(,)? ) => {
        $name
    };
    ( $name1:expr , $( $name2:expr ),* ) => {
        concat!( $name1, $( $crate::path_delimiter!(), $name2 ),* )
    };
}

#[cfg(unix)]
#[macro_export]
#[doc(hidden)]
macro_rules! path_delimiter {
    () => { "/" };
}

#[cfg(windows)]
#[macro_export]
#[doc(hidden)]
macro_rules! path_delimiter {
    () => { "\\" };
}
