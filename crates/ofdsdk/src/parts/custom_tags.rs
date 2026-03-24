//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default)]
pub struct CustomTags {
  pub inner_path: String,
  pub root_element: crate::schemas::custom_tags::CustomTags,
  pub custom_tag_files: Vec<crate::parts::custom_tag_file::CustomTagFile>,
  pub custom_tag_schemas: Vec<crate::parts::custom_tag_schema::CustomTagSchema>,
}
impl CustomTags {
  pub(crate) fn new_from_archive<R: std::io::Read + std::io::Seek>(
    path: &str,
    archive: &mut zip::ZipArchive<R>,
  ) -> Result<Self, crate::common::SdkError> {
    let root_element =
      crate::schemas::custom_tags::CustomTags::from_reader(std::io::BufReader::new(
        std::io::Cursor::new(crate::common::read_zip_data(archive, path)?),
      ))?;
    let current_dir = crate::common::zip_parent_dir(path);
    let mut custom_tag_files_paths: Vec<String> = vec![];
    for value_0 in &root_element.custom_tag {
      custom_tag_files_paths.push(value_0.file_loc.clone());
    }
    let custom_tag_files = crate::common::load_zip_parts(
      &current_dir,
      custom_tag_files_paths,
      archive,
      |child_path, archive| {
        let resolved_path = crate::common::resolve_zip_child_path(&current_dir, child_path);
        crate::parts::custom_tag_file::CustomTagFile::new_from_archive(&resolved_path, archive)
      },
    )?;
    let mut custom_tag_schemas_paths: Vec<String> = vec![];
    for value_0 in &root_element.custom_tag {
      if let Some(value) = &value_0.schema_loc {
        custom_tag_schemas_paths.push(value.clone());
      }
    }
    let custom_tag_schemas = crate::common::load_zip_parts(
      &current_dir,
      custom_tag_schemas_paths,
      archive,
      |child_path, archive| {
        let resolved_path = crate::common::resolve_zip_child_path(&current_dir, child_path);
        crate::parts::custom_tag_schema::CustomTagSchema::new_from_archive(&resolved_path, archive)
      },
    )?;
    Ok(Self {
      inner_path: path.to_string(),
      root_element,
      custom_tag_files,
      custom_tag_schemas,
    })
  }
  pub(crate) fn save_zip<W: std::io::Write + std::io::Seek>(
    &self,
    zip: &mut zip::ZipWriter<W>,
    entry_set: &mut std::collections::HashSet<String>,
  ) -> Result<(), crate::common::SdkError> {
    crate::common::save_zip_data(
      &self.inner_path,
      self.root_element.to_xml()?.as_bytes(),
      zip,
      entry_set,
    )?;
    crate::common::save_zip_parts(
      &self.custom_tag_files,
      zip,
      entry_set,
      |child, zip, entry_set| child.save_zip(zip, entry_set),
    )?;
    crate::common::save_zip_parts(
      &self.custom_tag_schemas,
      zip,
      entry_set,
      |child, zip, entry_set| child.save_zip(zip, entry_set),
    )?;
    Ok(())
  }
}
