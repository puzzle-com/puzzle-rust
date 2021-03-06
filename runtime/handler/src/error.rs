use {multihash, cid, http};
use route::Out;

pub type Result<T> = ::std::result::Result<T, Error>;

/// Handler server error
#[derive(Debug)]
pub enum ServerError {
	/// Wrapped `std::io::Error`
	IoError(::std::io::Error),
	/// Other `hyper` error
	Other(http::hyper::error::Error),
	/// Invalid --ipfs-api-interface
	InvalidInterface
}

/// Handle IO errors (ports taken when starting the server).
impl From<::std::io::Error> for ServerError {
	fn from(err: ::std::io::Error) -> ServerError {
		ServerError::IoError(err)
	}
}

impl From<http::hyper::error::Error> for ServerError {
	fn from(err: http::hyper::error::Error) -> ServerError {
		ServerError::Other(err)
	}
}

impl From<ServerError> for String {
	fn from(err: ServerError) -> String {
		match err {
			ServerError::IoError(err) => err.to_string(),
			ServerError::Other(err) => err.to_string(),
			ServerError::InvalidInterface => "Invalid --ipfs-api-interface parameter".into(),
		}
	}
}

impl ::std::fmt::Display for ServerError {
	fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
        	ServerError::IoError(err) => write!(f, "Io Error: {}", err),
        	ServerError::Other(err) => write!(f, "Other error: {}", err),
        	ServerError::InvalidInterface => write!(f, "Invalid interface"),
        }
    }
}

impl ::std::error::Error for ServerError {}

#[derive(Debug, PartialEq)]
pub enum Error {
	CidParsingFailed,
	UnsupportedHash,
	UnsupportedCid,
	BlockNotFound,
	TransactionNotFound,
	StateRootNotFound,
	ContractNotFound,
}

/// Convert Error into Out, handy when switching from Rust's Result-based
/// error handling to Hyper's request handling.
impl From<Error> for Out {
	fn from(err: Error) -> Out {
		use self::Error::*;

		match err {
			UnsupportedHash => Out::Bad("Hash must be Keccak-256"),
			UnsupportedCid => Out::Bad("CID codec not supported"),
			CidParsingFailed => Out::Bad("CID parsing failed"),
			BlockNotFound => Out::NotFound("Block not found"),
			TransactionNotFound => Out::NotFound("Transaction not found"),
			StateRootNotFound => Out::NotFound("State root not found"),
			ContractNotFound => Out::NotFound("Contract not found"),
		}
	}
}

/// Convert Content ID errors.
impl From<cid::Error> for Error {
	fn from(_: cid::Error) -> Error {
		Error::CidParsingFailed
	}
}

/// Convert multihash errors (multihash being part of CID).
impl From<multihash::Error> for Error {
	fn from(_: multihash::Error) -> Error {
		Error::CidParsingFailed
	}
}
