//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default)]
pub struct Document {
  pub inner_path: String,
  pub doc_body: crate::schemas::ofd::DocBody,
  pub root_element: crate::schemas::document::Document,
  pub pages: Vec<crate::parts::page::Page>,
  pub template_pages: Vec<crate::parts::template_page::TemplatePage>,
  pub annotations: Option<Box<crate::parts::annotations::Annotations>>,
  pub attachments: Option<Box<crate::parts::attachments::Attachments>>,
  pub custom_tags: Option<Box<crate::parts::custom_tags::CustomTags>>,
  pub extensions: Option<Box<crate::parts::extensions::Extensions>>,
  pub public_res: Vec<crate::parts::public_res::PublicRes>,
  pub document_res: Vec<crate::parts::document_res::DocumentRes>,
  pub signatures: Option<Box<crate::parts::signatures::Signatures>>,
  pub versions: Vec<crate::parts::version::Version>,
}
impl Document {
  pub(crate) fn new_from_archive<R: std::io::Read + std::io::Seek>(
    path: &str,
    doc_body: crate::schemas::ofd::DocBody,
    archive: &mut zip::ZipArchive<R>,
  ) -> Result<Self, crate::common::SdkError> {
    let root_element = crate::schemas::document::Document::from_reader(std::io::BufReader::new(
      std::io::Cursor::new(crate::common::read_zip_data(archive, path)?),
    ))?;
    let current_dir = crate::common::zip_parent_dir(path);
    let mut pages_paths: Vec<String> = vec![];
    let value_0 = &root_element.pages;
    for value_1 in &value_0.page {
      pages_paths.push(value_1.base_loc.clone());
    }
    let pages =
      crate::common::load_zip_parts(&current_dir, pages_paths, archive, |child_path, archive| {
        let resolved_path = crate::common::resolve_zip_child_path(&current_dir, child_path);
        crate::parts::page::Page::new_from_archive(&resolved_path, archive)
      })?;
    let mut template_pages_paths: Vec<String> = vec![];
    let value_0 = &root_element.common_data;
    for value_1 in &value_0.template_page {
      template_pages_paths.push(value_1.base_loc.clone());
    }
    let template_pages = crate::common::load_zip_parts(
      &current_dir,
      template_pages_paths,
      archive,
      |child_path, archive| {
        let resolved_path = crate::common::resolve_zip_child_path(&current_dir, child_path);
        crate::parts::template_page::TemplatePage::new_from_archive(&resolved_path, archive)
      },
    )?;
    let mut annotations_paths: Vec<String> = vec![];
    if let Some(value) = &root_element.annotations {
      annotations_paths.push(value.clone());
    }
    let annotations = crate::common::load_optional_zip_part(
      &current_dir,
      annotations_paths,
      archive,
      |child_path, archive| {
        let resolved_path = crate::common::resolve_zip_child_path(&current_dir, child_path);
        crate::parts::annotations::Annotations::new_from_archive(&resolved_path, archive)
      },
    )?;
    let mut attachments_paths: Vec<String> = vec![];
    if let Some(value) = &root_element.attachments {
      attachments_paths.push(value.clone());
    }
    let attachments = crate::common::load_optional_zip_part(
      &current_dir,
      attachments_paths,
      archive,
      |child_path, archive| {
        let resolved_path = crate::common::resolve_zip_child_path(&current_dir, child_path);
        crate::parts::attachments::Attachments::new_from_archive(&resolved_path, archive)
      },
    )?;
    let mut custom_tags_paths: Vec<String> = vec![];
    if let Some(value) = &root_element.custom_tags {
      custom_tags_paths.push(value.clone());
    }
    let custom_tags = crate::common::load_optional_zip_part(
      &current_dir,
      custom_tags_paths,
      archive,
      |child_path, archive| {
        let resolved_path = crate::common::resolve_zip_child_path(&current_dir, child_path);
        crate::parts::custom_tags::CustomTags::new_from_archive(&resolved_path, archive)
      },
    )?;
    let mut extensions_paths: Vec<String> = vec![];
    if let Some(value) = &root_element.extensions {
      extensions_paths.push(value.clone());
    }
    let extensions = crate::common::load_optional_zip_part(
      &current_dir,
      extensions_paths,
      archive,
      |child_path, archive| {
        let resolved_path = crate::common::resolve_zip_child_path(&current_dir, child_path);
        crate::parts::extensions::Extensions::new_from_archive(&resolved_path, archive)
      },
    )?;
    let mut public_res_paths: Vec<String> = vec![];
    let value_0 = &root_element.common_data;
    for value in &value_0.public_res {
      public_res_paths.push(value.clone());
    }
    let public_res = crate::common::load_zip_parts(
      &current_dir,
      public_res_paths,
      archive,
      |child_path, archive| {
        let resolved_path = crate::common::resolve_zip_child_path(&current_dir, child_path);
        crate::parts::public_res::PublicRes::new_from_archive(&resolved_path, archive)
      },
    )?;
    let mut document_res_paths: Vec<String> = vec![];
    let value_0 = &root_element.common_data;
    for value in &value_0.document_res {
      document_res_paths.push(value.clone());
    }
    let document_res = crate::common::load_zip_parts(
      &current_dir,
      document_res_paths,
      archive,
      |child_path, archive| {
        let resolved_path = crate::common::resolve_zip_child_path(&current_dir, child_path);
        crate::parts::document_res::DocumentRes::new_from_archive(&resolved_path, archive)
      },
    )?;
    let mut signatures_paths: Vec<String> = vec![];
    if let Some(value) = &doc_body.signatures {
      signatures_paths.push(value.clone());
    }
    let signatures = crate::common::load_optional_zip_part(
      &current_dir,
      signatures_paths,
      archive,
      |child_path, archive| {
        let resolved_path = crate::common::resolve_zip_file_path(child_path);
        crate::parts::signatures::Signatures::new_from_archive(&resolved_path, archive)
      },
    )?;
    let mut versions_paths: Vec<String> = vec![];
    if let Some(value_0) = &doc_body.versions {
      for value_1 in &value_0.version {
        versions_paths.push(value_1.base_loc.clone());
      }
    }
    let versions = crate::common::load_zip_parts(
      &current_dir,
      versions_paths,
      archive,
      |child_path, archive| {
        let resolved_path = crate::common::resolve_zip_file_path(child_path);
        crate::parts::version::Version::new_from_archive(&resolved_path, archive)
      },
    )?;
    Ok(Self {
      inner_path: path.to_string(),
      doc_body,
      root_element,
      pages,
      template_pages,
      annotations,
      attachments,
      custom_tags,
      extensions,
      public_res,
      document_res,
      signatures,
      versions,
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
    crate::common::save_zip_parts(&self.pages, zip, entry_set, |child, zip, entry_set| {
      child.save_zip(zip, entry_set)
    })?;
    crate::common::save_zip_parts(
      &self.template_pages,
      zip,
      entry_set,
      |child, zip, entry_set| child.save_zip(zip, entry_set),
    )?;
    crate::common::save_optional_zip_part(
      &self.annotations,
      zip,
      entry_set,
      |child, zip, entry_set| child.save_zip(zip, entry_set),
    )?;
    crate::common::save_optional_zip_part(
      &self.attachments,
      zip,
      entry_set,
      |child, zip, entry_set| child.save_zip(zip, entry_set),
    )?;
    crate::common::save_optional_zip_part(
      &self.custom_tags,
      zip,
      entry_set,
      |child, zip, entry_set| child.save_zip(zip, entry_set),
    )?;
    crate::common::save_optional_zip_part(
      &self.extensions,
      zip,
      entry_set,
      |child, zip, entry_set| child.save_zip(zip, entry_set),
    )?;
    crate::common::save_zip_parts(&self.public_res, zip, entry_set, |child, zip, entry_set| {
      child.save_zip(zip, entry_set)
    })?;
    crate::common::save_zip_parts(
      &self.document_res,
      zip,
      entry_set,
      |child, zip, entry_set| child.save_zip(zip, entry_set),
    )?;
    crate::common::save_optional_zip_part(
      &self.signatures,
      zip,
      entry_set,
      |child, zip, entry_set| child.save_zip(zip, entry_set),
    )?;
    crate::common::save_zip_parts(&self.versions, zip, entry_set, |child, zip, entry_set| {
      child.save_zip(zip, entry_set)
    })?;
    Ok(())
  }
}
