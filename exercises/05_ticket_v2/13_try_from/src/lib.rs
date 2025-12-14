// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(thiserror::Error, Debug)]
enum ParseStatusError {
    #[error("String: {0} is not a valid status")]
    TryFromStringError(String),
    #[error("&str value: {0} is not a valid status")]
    TryFromSTRError(String),
}
impl TryFrom<String> for Status {
    type Error = ParseStatusError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let status_string = value.trim().to_lowercase();

        match &*status_string {
            "todo" => Ok(Self::ToDo),
            "inprogress" => Ok(Self::InProgress),
            "done" => Ok(Self::Done),
            _ => Err(ParseStatusError::TryFromStringError(value.to_string())),
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = ParseStatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let status_string = value.trim().to_lowercase();

        match status_string.as_str() {
            "todo" => Ok(Self::ToDo),
            "inprogress" => Ok(Self::InProgress),
            "done" => Ok(Self::Done),
            _ => Err(ParseStatusError::TryFromSTRError(value.to_string())),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
