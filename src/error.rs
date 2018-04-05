pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Fail)]
#[must_use]
pub enum Error {
    #[fail(display = "io error: {:?}", _0)]
    Io(::std::io::Error),

    #[fail(display = "utf8 error: {:?}", _0)]
    Utf8(::std::string::FromUtf8Error),

    #[fail(display = "set atom error: {:?}", _0)]
    Set(::std::sync::mpsc::SendError<::xcb::Atom>),

    #[fail(display = "xcb connection error: {:?}", _0)]
    XcbConn(::xcb::base::ConnError),

    #[fail(display = "xcb generic error: {:?}", _0)]
    XcbGeneric(::xcb::base::GenericError),

    #[fail(display = "store lock poison")]
    Lock,
    #[fail(display = "load selection timeout")]
    Timeout,
    #[fail(display = "set selection owner fail")]
    Owner
}

macro_rules! define_from {
    ( $item:ident from $err:ty ) => {
        impl From<$err> for Error {
            fn from(err: $err) -> Error {
                Error::$item(err)
            }
        }
    }
}

define_from!(Io from ::std::io::Error);
define_from!(Utf8 from ::std::string::FromUtf8Error);
define_from!(Set from ::std::sync::mpsc::SendError<::xcb::Atom>);
define_from!(XcbConn from ::xcb::base::ConnError);
define_from!(XcbGeneric from ::xcb::base::GenericError);
