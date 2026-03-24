//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default)]
pub struct OfdPackage {
  pub inner_path: String,
  pub root_element: crate::schemas::ofd::Ofd,
  pub documents: Vec<crate::parts::document::Document>,
}
impl OfdPackage {
  pub(crate) fn new_from_archive<R: std::io::Read + std::io::Seek>(
    path: &str,
    archive: &mut zip::ZipArchive<R>,
  ) -> Result<Self, crate::common::SdkError> {
    let root_element = crate::schemas::ofd::Ofd::from_reader(std::io::BufReader::new(
      std::io::Cursor::new(crate::common::read_zip_data(archive, path)?),
    ))?;
    let current_dir = crate::common::zip_parent_dir(path);
    let mut documents_paths: Vec<String> = vec![];
    for value_0 in &root_element.doc_body {
      documents_paths.push(value_0.doc_root.clone());
    }
    let mut documents_contexts: Vec<crate::schemas::ofd::DocBody> = vec![];
    for value in &root_element.doc_body {
      documents_contexts.push(value.clone());
    }
    let documents = crate::common::load_zip_parts_with_context(
      &current_dir,
      documents_paths,
      documents_contexts,
      "Documents",
      archive,
      |child_path, child_context, archive| {
        let resolved_path = crate::common::resolve_zip_child_path(&current_dir, child_path);
        crate::parts::document::Document::new_from_archive(&resolved_path, child_context, archive)
      },
    )?;
    Ok(Self {
      inner_path: path.to_string(),
      root_element,
      documents,
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
    crate::common::save_zip_parts(&self.documents, zip, entry_set, |child, zip, entry_set| {
      child.save_zip(zip, entry_set)
    })?;
    Ok(())
  }
}
impl OfdPackage {
  pub fn new<R: std::io::Read + std::io::Seek>(reader: R) -> Result<Self, crate::common::SdkError> {
    let mut archive = zip::ZipArchive::new(reader)?;
    Self::new_from_archive("OFD.xml", &mut archive)
  }
  pub fn new_from_file<P: AsRef<std::path::Path>>(
    path: P,
  ) -> Result<Self, crate::common::SdkError> {
    Self::new(std::io::BufReader::new(std::fs::File::open(path)?))
  }
  pub fn save<W: std::io::Write + std::io::Seek>(
    &self,
    writer: W,
  ) -> Result<(), crate::common::SdkError> {
    let mut entry_set = std::collections::HashSet::new();
    let mut zip = zip::ZipWriter::new(writer);
    self.save_zip(&mut zip, &mut entry_set)?;
    zip.finish()?;
    Ok(())
  }
  pub fn save_to_file<P: AsRef<std::path::Path>>(
    &self,
    path: P,
  ) -> Result<(), crate::common::SdkError> {
    self.save(std::fs::File::create(path)?)
  }
}
