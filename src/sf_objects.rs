use serde::{Serialize, Deserialize, Deserializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct SFObject {
  #[serde(rename = "childRelationships")]
  pub child_relationships: Vec<SFRelationship>,
  pub fields: Vec<SFField>,
  pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SFRelationship {
  #[serde(rename = "childSObject")]
  pub child_object: String,

  pub field: String,

  #[serde(rename = "relationshipName")]
  pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SFField {
  pub label: String,
  pub name: String,

  #[serde(rename = "picklistValues")]
  pub picklist: Vec<String>,

  #[serde(rename = "relationshipName")]
  pub relationship_name: Option<String>,

  #[serde(rename = "referenceTo")]
  pub reference_to: Vec<String>,

  #[serde(rename = "type")]
  pub field_type: String
}



impl<'de> Deserialize<'de> for SFField {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where D: Deserializer<'de>
  {

    #[derive(Debug, Deserialize)]
    struct Field {
      label: String,
      name: String,

      #[serde(rename = "picklistValues")]
      picklist: Vec<Picklist>,

      #[serde(rename = "relationshipName")]
      relationship_name: Option<String>,

      #[serde(rename = "referenceTo")]
      reference_to: Vec<String>,

      #[serde(rename = "type")]
      field_type: String
    }

    #[derive(Clone, Debug, Deserialize)]
    struct Picklist {
      active: bool,
      value: String
    }

    let field    = Field::deserialize(deserializer)?;
    let picklist = field.picklist
      .iter()
      .filter(|&p| p.active)
      .map(|p| p.value.to_owned())
      .collect::<Vec<_>>();

    Ok(SFField {
      label:             field.label,
      name:              field.name,
      picklist:          picklist,
      field_type:        field.field_type,
      reference_to:      field.reference_to,
      relationship_name: field.relationship_name,
    })
  }
}
