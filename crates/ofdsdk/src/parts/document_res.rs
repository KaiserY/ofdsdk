//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default)]
pub struct DocumentRes {
  pub inner_path: String,
  pub root_element: crate::schemas::res::Res,
  pub document_res_font_files: Vec<crate::parts::document_res_font_file::DocumentResFontFile>,
  pub document_res_media_files: Vec<crate::parts::document_res_media_file::DocumentResMediaFile>,
}
impl DocumentRes {
  pub(crate) fn new_from_archive<R: std::io::Read + std::io::Seek>(
    path: &str,
    archive: &mut zip::ZipArchive<R>,
  ) -> Result<Self, crate::common::SdkError> {
    let root_element = crate::schemas::res::Res::from_reader(std::io::BufReader::new(
      std::io::Cursor::new(crate::common::read_zip_data(archive, path)?),
    ))?;
    let current_dir = crate::common::zip_parent_dir(path);
    let mut document_res_font_files_paths: Vec<String> = vec![];
    for choice in &root_element.xml_children {
      if let crate::schemas::res::ResContentChoice::Fonts(value) = choice {
        let value_0 = &**value;
        for value_1 in &value_0.font {
          if let Some(value) = &value_1.font_file {
            document_res_font_files_paths.push(value.clone());
          }
        }
      }
    }
    let document_res_font_files = crate::common::load_zip_parts(
      &current_dir,
      document_res_font_files_paths,
      archive,
      |child_path, archive| {
        let resolved_path = {
          let base_path = root_element.base_loc.clone();
          let base_dir = crate::common::resolve_zip_child_path(&current_dir, &base_path);
          crate::common::resolve_zip_child_path(&format!("{base_dir}/"), child_path)
        };
        crate::parts::document_res_font_file::DocumentResFontFile::new_from_archive(
          &resolved_path,
          archive,
        )
      },
    )?;
    let mut document_res_media_files_paths: Vec<String> = vec![];
    for choice in &root_element.xml_children {
      if let crate::schemas::res::ResContentChoice::MultiMedias(value) = choice {
        let value_0 = &**value;
        for value_1 in &value_0.multi_media {
          document_res_media_files_paths.push(value_1.media_file.clone());
        }
      }
    }
    let document_res_media_files = crate::common::load_zip_parts(
      &current_dir,
      document_res_media_files_paths,
      archive,
      |child_path, archive| {
        let resolved_path = {
          let base_path = root_element.base_loc.clone();
          let base_dir = crate::common::resolve_zip_child_path(&current_dir, &base_path);
          crate::common::resolve_zip_child_path(&format!("{base_dir}/"), child_path)
        };
        crate::parts::document_res_media_file::DocumentResMediaFile::new_from_archive(
          &resolved_path,
          archive,
        )
      },
    )?;
    Ok(Self {
      inner_path: path.to_string(),
      root_element,
      document_res_font_files,
      document_res_media_files,
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
      &self.document_res_font_files,
      zip,
      entry_set,
      |child, zip, entry_set| child.save_zip(zip, entry_set),
    )?;
    crate::common::save_zip_parts(
      &self.document_res_media_files,
      zip,
      entry_set,
      |child, zip, entry_set| child.save_zip(zip, entry_set),
    )?;
    Ok(())
  }
}
