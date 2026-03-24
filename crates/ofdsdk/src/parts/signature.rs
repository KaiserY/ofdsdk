//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default)]
pub struct Signature {
  pub inner_path: String,
  pub root_element: crate::schemas::signature::Signature,
  pub signature_value: Box<crate::parts::signature_value::SignatureValue>,
  pub seal_file: Option<Box<crate::parts::seal_file::SealFile>>,
}
impl Signature {
  pub(crate) fn new_from_archive<R: std::io::Read + std::io::Seek>(
    path: &str,
    archive: &mut zip::ZipArchive<R>,
  ) -> Result<Self, crate::common::SdkError> {
    let root_element = crate::schemas::signature::Signature::from_reader(std::io::BufReader::new(
      std::io::Cursor::new(crate::common::read_zip_data(archive, path)?),
    ))?;
    let current_dir = crate::common::zip_parent_dir(path);
    let signature_value_single_paths = vec![root_element.signed_value.clone()];
    let signature_value = crate::common::load_required_zip_part(
      &current_dir,
      signature_value_single_paths,
      "SignatureValue",
      archive,
      |child_path, archive| {
        let resolved_path = crate::common::resolve_zip_child_path(&current_dir, child_path);
        crate::parts::signature_value::SignatureValue::new_from_archive(&resolved_path, archive)
      },
    )?;
    let mut seal_file_paths: Vec<String> = vec![];
    let value_0 = &root_element.signed_info;
    if let Some(value_1) = &value_0.seal {
      seal_file_paths.push(value_1.base_loc.clone());
    }
    let seal_file = crate::common::load_optional_zip_part(
      &current_dir,
      seal_file_paths,
      archive,
      |child_path, archive| {
        let resolved_path = crate::common::resolve_zip_child_path(&current_dir, child_path);
        crate::parts::seal_file::SealFile::new_from_archive(&resolved_path, archive)
      },
    )?;
    Ok(Self {
      inner_path: path.to_string(),
      root_element,
      signature_value,
      seal_file,
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
    crate::common::save_required_zip_part(
      &self.signature_value,
      zip,
      entry_set,
      |child, zip, entry_set| child.save_zip(zip, entry_set),
    )?;
    crate::common::save_optional_zip_part(
      &self.seal_file,
      zip,
      entry_set,
      |child, zip, entry_set| child.save_zip(zip, entry_set),
    )?;
    Ok(())
  }
}
