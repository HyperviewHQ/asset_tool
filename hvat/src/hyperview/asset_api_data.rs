use serde::{Deserialize, Serialize};
use std::fmt;

use super::cli_data::{RackPosition, RackSide};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetDto {
    pub id: String,
    pub name: String,
    #[serde(alias = "assetLifecycleState")]
    pub asset_lifecycle_state: String,
    #[serde(alias = "assetTypeId")]
    pub asset_type_id: String,
    #[serde(alias = "manufacturerId")]
    pub manufacturer_id: String,
    #[serde(alias = "manufacturerName")]
    pub manufacturer_name: String,
    #[serde(alias = "monitoringState")]
    pub monitoring_state: String,
    #[serde(alias = "parentId")]
    pub parent_id: String,
    #[serde(alias = "parentName")]
    pub parent_name: String,
    #[serde(alias = "productId")]
    pub product_id: String,
    #[serde(alias = "productName")]
    pub product_name: String,
    pub status: String,
    pub path: String,
    #[serde(alias = "serialNumber")]
    pub serial_number: String,
    pub property: Option<String>,
}

impl fmt::Display for AssetDto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut asset_record = format!(
            "id: {}\nname: {}\nasset_lifecycle_state: {}\nasset_type_id: {}\nmanufacturer_id: {}\nmanufacturer_name: {}\nmonitoring_state: {}\nparent_id: {}\nparent_name: {}\nproduct_id: {}\nproduct_name: {}\nstatus: {}\npath: {}\nserial_number: {}",
            self.id,
            self.name,
            self.asset_lifecycle_state,
            self.asset_type_id,
            self.manufacturer_id,
            self.manufacturer_name,
            self.monitoring_state,
            self.parent_id,
            self.parent_name,
            self.product_id,
            self.product_name,
            self.status,
            self.path,
            self.serial_number
        );

        if let Some(property) = &self.property {
            asset_record = format!("{}\nproperty: {}", asset_record, property);
        };

        write!(f, "{}", asset_record)
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateAssetNameRecord {
    pub asset_id: String,
    pub new_name: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetLocationDTO {
    pub parent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rack_position: Option<RackPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rack_side: Option<RackSide>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rack_u_location: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAssetLocationRecord {
    pub asset_id: String,
    pub new_location_id: String,
    pub rack_position: Option<RackPosition>,
    pub rack_side: Option<RackSide>,
    pub rack_u_location: Option<usize>,
}
