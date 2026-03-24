use super::*;

fn with_occurs_state(options: TraversalOptions, occurs_state: OccursState) -> TraversalOptions {
  TraversalOptions {
    type_name_mode: options.type_name_mode,
    occurs_state,
  }
}

fn sequence_options(options: TraversalOptions, seq: &Sequence) -> TraversalOptions {
  with_occurs_state(
    options,
    next_occurs_state(
      options.occurs_state,
      seq.min_occurs.as_deref(),
      seq.max_occurs.as_deref(),
    ),
  )
}

fn choice_options(options: TraversalOptions, choice: &Choice) -> TraversalOptions {
  with_occurs_state(
    options,
    next_occurs_state(
      options.occurs_state,
      choice.min_occurs.as_deref(),
      choice.max_occurs.as_deref(),
    ),
  )
}

fn choice_branch_options(options: TraversalOptions) -> TraversalOptions {
  with_occurs_state(options, choice_branch_occurs_state(options.occurs_state))
}

pub(super) fn gen_sequence_contents(
  skd_data_context: &SdkDataContext,
  xsd_schema: &XsdSchema,
  sdk_data_struct: &mut Struct,
  sdk_data_schema: &mut SdkDataSchema,
  seq: &Sequence,
  options: TraversalOptions,
  deferred_inline_elements: &mut Vec<DeferredInlineElement>,
) -> anyhow::Result<()> {
  let options = sequence_options(options, seq);
  let occurs_state = options.occurs_state;

  for seq_content in &seq.contents {
    match seq_content {
      SequenceContentChoice::Element(seq_cc_e) => {
        let e_name = resolve_particle_element_name(seq_cc_e)?;
        let e_type = resolve_element_type(
          skd_data_context,
          xsd_schema,
          sdk_data_schema,
          &sdk_data_struct.name,
          seq_cc_e,
          options.type_name_mode,
          deferred_inline_elements,
        )?;
        let element_occurs_state = element_occurs_state(occurs_state, seq_cc_e);

        push_sequence_element(
          sdk_data_schema,
          sdk_data_struct,
          e_name,
          e_type,
          !element_occurs_state.is_vec && element_occurs_state.is_option,
          element_occurs_state.is_vec,
        )?;
      }
      SequenceContentChoice::Choice(seq_cc_c) => {
        gen_choice_contents(
          skd_data_context,
          xsd_schema,
          sdk_data_struct,
          sdk_data_schema,
          seq_cc_c,
          options,
          deferred_inline_elements,
        )?;
      }
      SequenceContentChoice::Sequence(nested_seq) => {
        gen_sequence_contents(
          skd_data_context,
          xsd_schema,
          sdk_data_struct,
          sdk_data_schema,
          nested_seq,
          options,
          deferred_inline_elements,
        )?;
      }
    }
  }

  Ok(())
}

pub(super) fn gen_choice_contents(
  skd_data_context: &SdkDataContext,
  xsd_schema: &XsdSchema,
  sdk_data_struct: &mut Struct,
  sdk_data_schema: &mut SdkDataSchema,
  choice: &Choice,
  options: TraversalOptions,
  deferred_inline_elements: &mut Vec<DeferredInlineElement>,
) -> anyhow::Result<()> {
  let options = choice_options(options, choice);

  for choice_content in &choice.contents {
    match choice_content {
      ChoiceContentChoice::Element(choice_e) => {
        let e_name = resolve_particle_element_name(choice_e)?;
        let e_type = resolve_element_type(
          skd_data_context,
          xsd_schema,
          sdk_data_schema,
          &sdk_data_struct.name,
          choice_e,
          options.type_name_mode,
          deferred_inline_elements,
        )?;

        push_choice_element(sdk_data_schema, sdk_data_struct, e_name, e_type)?;
      }
      ChoiceContentChoice::Choice(nested_choice) => {
        gen_choice_contents(
          skd_data_context,
          xsd_schema,
          sdk_data_struct,
          sdk_data_schema,
          nested_choice,
          choice_branch_options(options),
          deferred_inline_elements,
        )?;
      }
      ChoiceContentChoice::Sequence(nested_seq) => {
        gen_sequence_contents(
          skd_data_context,
          xsd_schema,
          sdk_data_struct,
          sdk_data_schema,
          nested_seq,
          choice_branch_options(options),
          deferred_inline_elements,
        )?;
      }
    }
  }

  Ok(())
}

pub(super) fn gen_extension_content(
  skd_data_context: &SdkDataContext,
  xsd_schema: &XsdSchema,
  sdk_data_struct: &mut Struct,
  sdk_data_schema: &mut SdkDataSchema,
  ext_content: &ExtensionContentChoice,
  type_name_mode: TypeNameMode,
  deferred_inline_elements: &mut Vec<DeferredInlineElement>,
) -> anyhow::Result<()> {
  match ext_content {
    ExtensionContentChoice::Attribute(attr) => gen_attribute(
      skd_data_context,
      xsd_schema,
      attr,
      sdk_data_struct,
      sdk_data_schema,
    ),
    ExtensionContentChoice::Sequence(seq) => gen_sequence_contents(
      skd_data_context,
      xsd_schema,
      sdk_data_struct,
      sdk_data_schema,
      seq,
      TraversalOptions {
        type_name_mode: TypeNameMode::CamelCaseInlineName,
        occurs_state: OccursState::default(),
      },
      deferred_inline_elements,
    ),
    ExtensionContentChoice::Choice(choice) => gen_choice_contents(
      skd_data_context,
      xsd_schema,
      sdk_data_struct,
      sdk_data_schema,
      choice,
      TraversalOptions {
        type_name_mode,
        occurs_state: OccursState::default(),
      },
      deferred_inline_elements,
    ),
  }
}

pub(super) fn gen_simple_content_extension(
  skd_data_context: &SdkDataContext,
  xsd_schema: &XsdSchema,
  sdk_data_struct: &mut Struct,
  sdk_data_schema: &mut SdkDataSchema,
  ext: &Extension,
) -> anyhow::Result<()> {
  insert_sequence_child(
    sdk_data_schema,
    sdk_data_struct,
    Child {
      name: "xml_value".to_string(),
      xml_name: None,
      resolved_xml_name: String::new(),
      ident: "xml_value".to_string(),
      r#type: resolve_xsd_type(skd_data_context, xsd_schema, &ext.base),
      resolved_type: String::new(),
      type_kind: CodegenTypeKind::String,
      is_struct: false,
      source_module_name: Some(sdk_data_struct.module_name.clone()),
      is_option: false,
      is_vec: false,
    },
  )?;

  for ext_content in &ext.contents {
    if let ExtensionContentChoice::Attribute(attr) = ext_content {
      gen_attribute(
        skd_data_context,
        xsd_schema,
        attr,
        sdk_data_struct,
        sdk_data_schema,
      )?;
    }
  }

  Ok(())
}

pub(super) fn gen_complex_type_particle(
  skd_data_context: &SdkDataContext,
  xsd_schema: &XsdSchema,
  sdk_data_struct: &mut Struct,
  sdk_data_schema: &mut SdkDataSchema,
  ct_content: &ComplexTypeContentChoice,
  type_name_mode: TypeNameMode,
  deferred_inline_elements: &mut Vec<DeferredInlineElement>,
) -> anyhow::Result<bool> {
  match ct_content {
    ComplexTypeContentChoice::Attribute(attr) => {
      gen_attribute(
        skd_data_context,
        xsd_schema,
        attr,
        sdk_data_struct,
        sdk_data_schema,
      )?;
      Ok(true)
    }
    ComplexTypeContentChoice::Sequence(seq) => {
      gen_sequence_contents(
        skd_data_context,
        xsd_schema,
        sdk_data_struct,
        sdk_data_schema,
        seq,
        TraversalOptions {
          type_name_mode,
          occurs_state: OccursState::default(),
        },
        deferred_inline_elements,
      )?;
      Ok(true)
    }
    ComplexTypeContentChoice::Choice(choice) => {
      gen_choice_contents(
        skd_data_context,
        xsd_schema,
        sdk_data_struct,
        sdk_data_schema,
        choice,
        TraversalOptions {
          type_name_mode,
          occurs_state: OccursState::default(),
        },
        deferred_inline_elements,
      )?;
      Ok(true)
    }
    _ => Ok(false),
  }
}
