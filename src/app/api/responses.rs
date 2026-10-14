use crate::api::schema::{ErrorBody, ErrorResponse, ResponseResult, SuccessResponse};

pub(crate) fn encode_success(id: String, result: ResponseResult) -> String {
    serde_json::to_string(&SuccessResponse {
        id: id.clone(),
        result,
    })
    .unwrap_or_else(|err| {
        encode_error(
            id,
            "serialization_failed",
            format!("failed to serialize success response: {err}"),
        )
    })
}

pub(crate) fn encode_error(id: String, code: &str, message: impl Into<String>) -> String {
    encode_error_body(
        id,
        ErrorBody {
            code: code.into(),
            message: message.into(),
        },
    )
}

pub(super) fn encode_error_body(id: String, error: ErrorBody) -> String {
    serde_json::to_string(&ErrorResponse {
        id: id.clone(),
        error,
    })
    .unwrap_or_else(|err| {
        format!(
            "{{\"id\":{id:?},\"error\":{{\"code\":\"serialization_failed\",\"message\":{err_msg:?}}}}}",
            id = id,
            err_msg = format!("failed to serialize error response: {err}")
        )
    })
}
