//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default)]
pub struct Annotations {
  pub inner_path: String,
  pub root_element: crate::schemas::annotations::Annotations,
  pub annotations: Vec<crate::parts::annotation::Annotation>,
}
impl Annotations {
  pub(crate) fn new_from_archive<R: std::io::Read + std::io::Seek>(
    path: &str,
    archive: &mut zip::ZipArchive<R>,
  ) -> Result<Self, crate::common::SdkError> {
    let root_element =
      crate::schemas::annotations::Annotations::from_reader(std::io::BufReader::new(
        std::io::Cursor::new(crate::common::read_zip_data(archive, path)?),
      ))?;
    let current_dir = crate::common::zip_parent_dir(path);
    let mut annotations_paths: Vec<String> = vec![];
    for value_0 in &root_element.page {
      annotations_paths.push(value_0.file_loc.clone());
    }
    let annotations = crate::common::load_zip_parts(
      &current_dir,
      annotations_paths,
      archive,
      |child_path, archive| {
        let resolved_path = crate::common::resolve_zip_child_path(&current_dir, child_path);
        crate::parts::annotation::Annotation::new_from_archive(&resolved_path, archive)
      },
    )?;
    Ok(Self {
      inner_path: path.to_string(),
      root_element,
      annotations,
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
      &self.annotations,
      zip,
      entry_set,
      |child, zip, entry_set| child.save_zip(zip, entry_set),
    )?;
    Ok(())
  }
}
