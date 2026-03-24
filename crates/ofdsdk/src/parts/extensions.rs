//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default)]
pub struct Extensions {
  pub inner_path: String,
  pub root_element: crate::schemas::extensions::Extensions,
  pub extension_data_files: Vec<crate::parts::extension_data_file::ExtensionDataFile>,
}
impl Extensions {
  pub(crate) fn new_from_archive<R: std::io::Read + std::io::Seek>(
    path: &str,
    archive: &mut zip::ZipArchive<R>,
  ) -> Result<Self, crate::common::SdkError> {
    let root_element =
      crate::schemas::extensions::Extensions::from_reader(std::io::BufReader::new(
        std::io::Cursor::new(crate::common::read_zip_data(archive, path)?),
      ))?;
    let current_dir = crate::common::zip_parent_dir(path);
    let mut extension_data_files_paths: Vec<String> = vec![];
    for value_0 in &root_element.extension {
      for choice in &value_0.xml_children {
        if let crate::schemas::extensions::CtExtensionContentChoice::ExtendData(value) = choice {
          extension_data_files_paths.push((**value).clone());
        }
      }
    }
    let extension_data_files = crate::common::load_zip_parts(
      &current_dir,
      extension_data_files_paths,
      archive,
      |child_path, archive| {
        let resolved_path = crate::common::resolve_zip_child_path(&current_dir, child_path);
        crate::parts::extension_data_file::ExtensionDataFile::new_from_archive(
          &resolved_path,
          archive,
        )
      },
    )?;
    Ok(Self {
      inner_path: path.to_string(),
      root_element,
      extension_data_files,
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
      &self.extension_data_files,
      zip,
      entry_set,
      |child, zip, entry_set| child.save_zip(zip, entry_set),
    )?;
    Ok(())
  }
}
