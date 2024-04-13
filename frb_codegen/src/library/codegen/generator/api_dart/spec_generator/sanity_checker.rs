use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use anyhow::ensure;
use itertools::Itertools;

pub(crate) fn sanity_check_class_name_duplicates(
    classes: &[ApiDartGeneratedClass],
) -> anyhow::Result<()> {
    let duplicate_class_names = classes
        .iter()
        .map(|c| c.class_name.clone())
        .duplicates()
        .collect_vec();

    // This will stop the whole generator and tell the users, so we do not care about testing it
    // frb-coverage:ignore-start
    ensure!(
        duplicate_class_names.is_empty(),
        "Will generate duplicated class names ({:?}). This is often because the type is auto inferred as both opaque and non-opaque. Try to add `#[frb(opaque)]` or `#[frb(non_opaque)]` to the struct, or change code that uses it.",
        duplicate_class_names,
    );
    // frb-coverage:ignore-end

    Ok(())
}
