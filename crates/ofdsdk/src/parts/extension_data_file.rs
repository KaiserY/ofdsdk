//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default)]
pub struct ExtensionDataFile {
  pub inner_path: String,
  pub part_content: Vec<u8>,
}
impl ExtensionDataFile {
  pub(crate) fn new_from_archive<R: std::io::Read + std::io::Seek>(
    path: &str,
    archive: &mut zip::ZipArchive<R>,
  ) -> Result<Self, crate::common::SdkError> {
    let part_content = crate::common::read_zip_data(archive, path)?;
    Ok(Self {
      inner_path: path.to_string(),
      part_content,
    })
  }
  pub(crate) fn save_zip<W: std::io::Write + std::io::Seek>(
    &self,
    zip: &mut zip::ZipWriter<W>,
    entry_set: &mut std::collections::HashSet<String>,
  ) -> Result<(), crate::common::SdkError> {
    crate::common::save_zip_data(&self.inner_path, &self.part_content, zip, entry_set)?;
    Ok(())
  }
}
