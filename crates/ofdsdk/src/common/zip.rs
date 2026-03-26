#[cfg(feature = "parts")]
use super::SdkError;

pub fn resolve_zip_file_path(path: &str) -> String {
  let mut stack = Vec::new();

  for component in path.split('/') {
    match component {
      "" | "." => {}
      ".." => {
        stack.pop();
      }
      _ => {
        stack.push(component);
      }
    }
  }
  stack.join("/")
}

#[cfg(feature = "parts")]
pub fn zip_parent_dirs(path: &str) -> Vec<String> {
  let mut dirs = Vec::new();
  let mut current = String::new();

  for component in path.split('/').filter(|component| !component.is_empty()) {
    if !current.is_empty() {
      current.push('/');
    }
    current.push_str(component);
    current.push('/');
    dirs.push(current.clone());
    current.pop();
  }

  dirs.pop();
  dirs
}

#[inline]
pub fn zip_parent_dir(path: &str) -> String {
  match path.rfind('/') {
    Some(index) => path[..index + 1].to_string(),
    None => String::new(),
  }
}

#[inline]
pub fn resolve_zip_child_path(base_dir: &str, target: &str) -> String {
  if target.starts_with('/') {
    resolve_zip_file_path(target)
  } else {
    resolve_zip_file_path(&format!("{base_dir}{target}"))
  }
}

#[cfg(feature = "parts")]
pub fn read_zip_data<R: std::io::Read + std::io::Seek>(
  archive: &mut zip::ZipArchive<R>,
  path: &str,
) -> Result<Vec<u8>, SdkError> {
  use std::io::Read;

  let normalized_path = resolve_zip_file_path(path);
  let mut zip_entry = archive.by_name(&normalized_path)?;
  let mut data = Vec::with_capacity(zip_entry.size() as usize);
  zip_entry.read_to_end(&mut data)?;
  Ok(data)
}

#[cfg(feature = "parts")]
pub fn load_zip_parts<R, T, F>(
  _current_dir: &str,
  child_paths: Vec<String>,
  archive: &mut zip::ZipArchive<R>,
  mut load: F,
) -> Result<Vec<T>, SdkError>
where
  R: std::io::Read + std::io::Seek,
  F: FnMut(&str, &mut zip::ZipArchive<R>) -> Result<T, SdkError>,
{
  let mut parts = Vec::with_capacity(child_paths.len());

  for child_path in child_paths {
    parts.push(load(&child_path, archive)?);
  }

  Ok(parts)
}

#[cfg(feature = "parts")]
pub fn load_zip_parts_with_context<R, T, C, F>(
  _current_dir: &str,
  child_paths: Vec<String>,
  child_contexts: Vec<C>,
  child_api_name: &str,
  archive: &mut zip::ZipArchive<R>,
  mut load: F,
) -> Result<Vec<T>, SdkError>
where
  R: std::io::Read + std::io::Seek,
  F: FnMut(&str, C, &mut zip::ZipArchive<R>) -> Result<T, SdkError>,
{
  if child_paths.len() != child_contexts.len() {
    return Err(SdkError::CommonError(format!(
      "mismatched child contexts for {}",
      child_api_name
    )));
  }

  let mut parts = Vec::with_capacity(child_paths.len());

  for (child_path, child_context) in child_paths.into_iter().zip(child_contexts.into_iter()) {
    parts.push(load(&child_path, child_context, archive)?);
  }

  Ok(parts)
}

#[cfg(feature = "parts")]
pub fn load_optional_zip_part<R, T, F>(
  _current_dir: &str,
  child_paths: Vec<String>,
  archive: &mut zip::ZipArchive<R>,
  mut load: F,
) -> Result<Option<Box<T>>, SdkError>
where
  R: std::io::Read + std::io::Seek,
  F: FnMut(&str, &mut zip::ZipArchive<R>) -> Result<T, SdkError>,
{
  match child_paths.len() {
    0 => Ok(None),
    1 => Ok(Some(Box::new(load(&child_paths[0], archive)?))),
    _ => Err(SdkError::CommonError(
      "multiple child paths for optional part".to_string(),
    )),
  }
}

#[cfg(feature = "parts")]
pub fn load_optional_zip_part_with_context<R, T, C, F>(
  _current_dir: &str,
  child_paths: Vec<String>,
  child_contexts: Vec<C>,
  child_api_name: &str,
  archive: &mut zip::ZipArchive<R>,
  mut load: F,
) -> Result<Option<Box<T>>, SdkError>
where
  R: std::io::Read + std::io::Seek,
  F: FnMut(&str, C, &mut zip::ZipArchive<R>) -> Result<T, SdkError>,
{
  if child_paths.len() != child_contexts.len() {
    return Err(SdkError::CommonError(format!(
      "mismatched child contexts for {}",
      child_api_name
    )));
  }

  match child_paths.len() {
    0 => Ok(None),
    1 => {
      let child_path = child_paths.into_iter().next().unwrap();
      let child_context = child_contexts.into_iter().next().unwrap();
      Ok(Some(Box::new(load(&child_path, child_context, archive)?)))
    }
    _ => Err(SdkError::CommonError(
      "multiple child paths for optional part".to_string(),
    )),
  }
}

#[cfg(feature = "parts")]
pub fn load_required_zip_part<R, T, F>(
  current_dir: &str,
  child_paths: Vec<String>,
  child_api_name: &str,
  archive: &mut zip::ZipArchive<R>,
  load: F,
) -> Result<Box<T>, SdkError>
where
  R: std::io::Read + std::io::Seek,
  F: FnMut(&str, &mut zip::ZipArchive<R>) -> Result<T, SdkError>,
{
  load_optional_zip_part(current_dir, child_paths, archive, load)?
    .ok_or_else(|| SdkError::CommonError(child_api_name.to_string()))
}

#[cfg(feature = "parts")]
pub fn load_required_zip_part_with_context<R, T, C, F>(
  current_dir: &str,
  child_paths: Vec<String>,
  child_contexts: Vec<C>,
  child_api_name: &str,
  archive: &mut zip::ZipArchive<R>,
  load: F,
) -> Result<Box<T>, SdkError>
where
  R: std::io::Read + std::io::Seek,
  F: FnMut(&str, C, &mut zip::ZipArchive<R>) -> Result<T, SdkError>,
{
  load_optional_zip_part_with_context(
    current_dir,
    child_paths,
    child_contexts,
    child_api_name,
    archive,
    load,
  )?
  .ok_or_else(|| SdkError::CommonError(child_api_name.to_string()))
}

#[cfg(feature = "parts")]
pub fn save_zip_data<W: std::io::Write + std::io::Seek>(
  inner_path: &str,
  data: &[u8],
  zip: &mut zip::ZipWriter<W>,
  entry_set: &mut std::collections::HashSet<String>,
) -> Result<(), SdkError> {
  use std::io::Write;

  let options = zip::write::SimpleFileOptions::default()
    .compression_method(zip::CompressionMethod::Deflated)
    .unix_permissions(0o755);

  if inner_path.is_empty() {
    return Err(SdkError::CommonError("empty inner_path".to_string()));
  }

  for dir_path in zip_parent_dirs(inner_path) {
    if !entry_set.contains(&dir_path) {
      zip.add_directory(&dir_path, options)?;
      entry_set.insert(dir_path);
    }
  }

  if !entry_set.contains(inner_path) {
    zip.start_file(inner_path, options)?;
    zip.write_all(data)?;
    entry_set.insert(inner_path.to_string());
  }

  Ok(())
}

#[cfg(feature = "parts")]
pub fn save_zip_parts<W, T, F>(
  children: &[T],
  zip: &mut zip::ZipWriter<W>,
  entry_set: &mut std::collections::HashSet<String>,
  mut save: F,
) -> Result<(), SdkError>
where
  W: std::io::Write + std::io::Seek,
  F: FnMut(
    &T,
    &mut zip::ZipWriter<W>,
    &mut std::collections::HashSet<String>,
  ) -> Result<(), SdkError>,
{
  for child in children {
    save(child, zip, entry_set)?;
  }

  Ok(())
}

#[cfg(feature = "parts")]
pub fn save_optional_zip_part<W, T, F>(
  child: &Option<Box<T>>,
  zip: &mut zip::ZipWriter<W>,
  entry_set: &mut std::collections::HashSet<String>,
  mut save: F,
) -> Result<(), SdkError>
where
  W: std::io::Write + std::io::Seek,
  F: FnMut(
    &T,
    &mut zip::ZipWriter<W>,
    &mut std::collections::HashSet<String>,
  ) -> Result<(), SdkError>,
{
  if let Some(child) = child {
    save(child, zip, entry_set)?;
  }

  Ok(())
}

#[cfg(feature = "parts")]
pub fn save_required_zip_part<W, T, F>(
  child: &T,
  zip: &mut zip::ZipWriter<W>,
  entry_set: &mut std::collections::HashSet<String>,
  mut save: F,
) -> Result<(), SdkError>
where
  W: std::io::Write + std::io::Seek,
  F: FnMut(
    &T,
    &mut zip::ZipWriter<W>,
    &mut std::collections::HashSet<String>,
  ) -> Result<(), SdkError>,
{
  save(child, zip, entry_set)
}
