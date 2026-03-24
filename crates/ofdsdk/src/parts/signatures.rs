//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default)]
pub struct Signatures {
  pub inner_path: String,
  pub root_element: crate::schemas::signatures::Signatures,
  pub signatures: Vec<crate::parts::signature::Signature>,
}
impl Signatures {
  pub(crate) fn new_from_archive<R: std::io::Read + std::io::Seek>(
    path: &str,
    archive: &mut zip::ZipArchive<R>,
  ) -> Result<Self, crate::common::SdkError> {
    let root_element =
      crate::schemas::signatures::Signatures::from_reader(std::io::BufReader::new(
        std::io::Cursor::new(crate::common::read_zip_data(archive, path)?),
      ))?;
    let current_dir = crate::common::zip_parent_dir(path);
    let mut signatures_paths: Vec<String> = vec![];
    for value_0 in &root_element.signature {
      signatures_paths.push(value_0.base_loc.clone());
    }
    let signatures = crate::common::load_zip_parts(
      &current_dir,
      signatures_paths,
      archive,
      |child_path, archive| {
        let resolved_path = crate::common::resolve_zip_child_path(&current_dir, child_path);
        crate::parts::signature::Signature::new_from_archive(&resolved_path, archive)
      },
    )?;
    Ok(Self {
      inner_path: path.to_string(),
      root_element,
      signatures,
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
    crate::common::save_zip_parts(&self.signatures, zip, entry_set, |child, zip, entry_set| {
      child.save_zip(zip, entry_set)
    })?;
    Ok(())
  }
}
