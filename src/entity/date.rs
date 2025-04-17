use super::{EntityExtractionRequest, EntityLabel, ExtractedData};
use crate::error::{DetError, DetResult};
use chrono::Local;
use human_date_parser::{ParseResult, from_human_time};

pub struct DetDate;

// TODO: Implement these functions as a common Trait
impl DetDate {
    pub fn extraction_data_type() -> String {
        "ISO 8601 Date".to_string()
    }

    pub fn process(
        _text: &str,
        extraction_request: &EntityExtractionRequest,
        extracted_data: &ExtractedData,
    ) -> DetResult<ExtractedData> {
        // This would take the `ExtractedEntity` and return a Rust native `Date` object
        let now = Local::now().naive_local();
        match from_human_time(&extracted_data.matched_text, now) {
            Ok(parse_result) => match (&extraction_request.entity_label, parse_result) {
                (EntityLabel::Date, ParseResult::Date(parsed_date)) => Ok(ExtractedData {
                    label: EntityLabel::Date,
                    matched_text: parsed_date.format("%Y-%m-%d").to_string(),
                    variable_name: extraction_request.variable_name.clone(),
                    starting_position: 34,
                }),
                (_, _) => Err(DetError::ParseError("Data cannot be parsed".to_string())),
            },
            Err(err) => Err(DetError::ParseError(err.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extracted_date() {
        let date_extraction_request = EntityExtractionRequest {
            entity_label: EntityLabel::Date,
            variable_name: "transaction_date".to_string(),
        };

        let mock_response = r#"
[
    {
        "label": "Date",
        "matched_text": "23 January 2022",
        "variable_name": "transaction_date",
        "starting_position": 31
    }
]"#;
        let response_from_ai = serde_json::from_str::<Vec<ExtractedData>>(mock_response).unwrap();
        let extracted_date = DetDate::process("", &date_extraction_request, &response_from_ai[0]);

        match extracted_date {
            Ok(extracted_data) => {
                assert_eq!(extracted_data.label, EntityLabel::Date);
                assert_eq!(extracted_data.matched_text, "2022-01-23");
                assert_eq!(extracted_data.variable_name, "transaction_date");
                assert_eq!(extracted_data.starting_position, 34);
            }
            Err(err) => panic!("Did not get expected entities: {}", err),
        }
    }
}
