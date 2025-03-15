use reqwest::{
    Error as HttpError,
    StatusCode,
    Url,
};

/// Authentication Errors.
#[derive(Debug)]
pub enum AuthError {
    /// Authentication failed because of an HTTP client error.
    Http(HttpError),

    /// Authentication failed because of an HTTP header could not be formatted.
    HttpHeader(reqwest::header::ToStrError),

    /// A required parameter was missing from an API response.
    MissingParam(&'static str),

    /// The device authentication flow took too long to complete.
    Timeout,

    /// Unexpected status from the Auth API.
    UnexpectedStatus(StatusCode, Url),

    /// Failed to parse a URL.
    UrlParse(url::ParseError),
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::Http(inner) => std::fmt::Display::fmt(inner, f),
            AuthError::HttpHeader(inner) => std::fmt::Display::fmt(inner, f),
            AuthError::Timeout => write!(f, "device auth flow took too long to complete"),
            AuthError::MissingParam(name) => write!(f, "missing required parameter {}", name),
            AuthError::UnexpectedStatus(status, url) => write!(
                f, "unexpected auth API status {} for URL {}",
                status, url,
            ),
            AuthError::UrlParse(inner) => std::fmt::Display::fmt(inner, f),
        }
    }
}

impl std::error::Error for AuthError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AuthError::Http(ref inner) => Some(inner),
            AuthError::HttpHeader(ref inner) => Some(inner),
            AuthError::MissingParam(_) => None,
            AuthError::Timeout => None,
            AuthError::UnexpectedStatus(_, _) => None,
            AuthError::UrlParse(ref inner) => Some(inner),
        }
    }
}

impl From<HttpError> for AuthError {
    fn from(value: HttpError) -> Self {
        AuthError::Http(value)
    }
}

impl From<url::ParseError> for AuthError {
    fn from(value: url::ParseError) -> Self {
        AuthError::UrlParse(value)
    }
}

impl From<reqwest::header::ToStrError> for AuthError {
    fn from(value: reqwest::header::ToStrError) -> Self {
        AuthError::HttpHeader(value)
    }
}
