use std::any::Any;

use common_error::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Failed to connect Datanode at {}, source: {}", addr, source))]
    ConnectDatanode {
        addr: String,
        #[snafu(backtrace)]
        source: client::Error,
    },

    #[snafu(display("Failed to request Datanode, source: {}", source))]
    RequestDatanode {
        #[snafu(backtrace)]
        source: client::Error,
    },

    #[snafu(display("Runtime resource error, source: {}", source))]
    RuntimeResource {
        #[snafu(backtrace)]
        source: common_runtime::error::Error,
    },

    #[snafu(display("Failed to start server, source: {}", source))]
    StartServer {
        #[snafu(backtrace)]
        source: servers::error::Error,
    },

    #[snafu(display("Failed to parse address {}, source: {}", addr, source))]
    ParseAddr {
        addr: String,
        source: std::net::AddrParseError,
    },

    #[snafu(display("Failed to parse SQL, source: {}", source))]
    ParseSql {
        #[snafu(backtrace)]
        source: sql::error::Error,
    },

    #[snafu(display("Column datatype error, source: {}", source))]
    ColumnDataType {
        #[snafu(backtrace)]
        source: api::error::Error,
    },

    #[snafu(display(
        "Failed to convert column default constraint, column: {}, source: {}",
        column_name,
        source
    ))]
    ConvertColumnDefaultConstraint {
        column_name: String,
        #[snafu(backtrace)]
        source: datatypes::error::Error,
    },

    #[snafu(display("Invalid SQL, error: {}", err_msg))]
    InvalidSql {
        err_msg: String,
        backtrace: Backtrace,
    },

    #[snafu(display("Illegal Frontend state: {}", err_msg))]
    IllegalFrontendState {
        err_msg: String,
        backtrace: Backtrace,
    },

    #[snafu(display("Incomplete GRPC result: {}", err_msg))]
    IncompleteGrpcResult {
        err_msg: String,
        backtrace: Backtrace,
    },

    #[snafu(display("Failed to execute OpenTSDB put, reason: {}", reason))]
    ExecOpentsdbPut {
        reason: String,
        backtrace: Backtrace,
    },

    #[snafu(display("Failed to find partition column: {}", column_name))]
    FindPartitionColumn {
        column_name: String,
        backtrace: Backtrace,
    },

    #[snafu(display("Failed to find region, reason: {}", reason))]
    FindRegion {
        reason: String,
        backtrace: Backtrace,
    },

    #[snafu(display("Invaild InsertRequest, reason: {}", reason))]
    InvalidInsertRequest {
        reason: String,
        backtrace: Backtrace,
    },

    #[snafu(display("Expect {} region keys, actual {}", expect, actual))]
    RegionKeysSize {
        expect: usize,
        actual: usize,
        backtrace: Backtrace,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

impl ErrorExt for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::ConnectDatanode { .. }
            | Error::ParseAddr { .. }
            | Error::InvalidSql { .. }
            | Error::FindRegion { .. }
            | Error::InvalidInsertRequest { .. }
            | Error::FindPartitionColumn { .. }
            | Error::RegionKeysSize { .. } => StatusCode::InvalidArguments,
            Error::RuntimeResource { source, .. } => source.status_code(),
            Error::StartServer { source, .. } => source.status_code(),
            Error::ParseSql { source } => source.status_code(),
            Error::ConvertColumnDefaultConstraint { source, .. } => source.status_code(),
            Error::RequestDatanode { source } => source.status_code(),
            Error::ColumnDataType { .. } => StatusCode::Internal,
            Error::IllegalFrontendState { .. } | Error::IncompleteGrpcResult { .. } => {
                StatusCode::Unexpected
            }
            Error::ExecOpentsdbPut { .. } => StatusCode::Internal,
        }
    }

    fn backtrace_opt(&self) -> Option<&Backtrace> {
        ErrorCompat::backtrace(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}