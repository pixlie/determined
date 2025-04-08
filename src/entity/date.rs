use super::{EntityExtractionRequest, EntityLabel, ExtractedData, ExtractedEntity};
use crate::error::DetResult;
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
    ) -> DetResult<ExtractedEntity> {
        // This would take the `ExtractedEntity` and return a Rust native `Date` object
        Ok(ExtractedEntity {
            entity_label: EntityLabel::Date,
            matched_text: "Jan 23, 2022".to_string(),
            variable_name: extraction_request.variable_name.clone(),
            start: 34,
        })
    }
}
