use super::*;

#[test]
fn mock_simple_init() {
    // (Pass) Should look the same as a fresh 'vet init'.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, audits, imports) = files_inited(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-init", output);
}

#[test]
fn mock_simple_no_exemptions() {
    // (Fail) Should look the same as a fresh 'vet init' but with all 'exemptions' entries deleted.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, audits, imports) = files_no_exemptions(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-no-unaudited", output);
}

#[test]
fn mock_simple_full_audited() {
    // (Pass) All entries have direct full audits.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, audits, imports) = files_full_audited(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-full-audited", output);
}

#[test]
fn builtin_simple_init() {
    // (Pass) Should look the same as a fresh 'vet init'.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_inited(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-init", output);
}

#[test]
fn builtin_simple_no_exemptions() {
    // (Fail) Should look the same as a fresh 'vet init' but with all 'exemptions' entries deleted.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_no_exemptions(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-no-unaudited", output);
}

#[test]
fn builtin_simple_full_audited() {
    // (Pass) All entries have direct full audits.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_full_audited(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-full-audited", output);
}

#[test]
fn mock_simple_missing_transitive() {
    // (Fail) Missing an audit for a transitive dep

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = files_full_audited(&metadata);

    audits
        .audits
        .get_mut("transitive-third-party1")
        .unwrap()
        .clear();

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-missing-transitive", output);
}

#[test]
fn mock_simple_missing_direct_internal() {
    // (Fail) Missing an audit for a direct dep that has children

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = files_full_audited(&metadata);

    audits.audits.get_mut("third-party1").unwrap().clear();

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-missing-direct-internal", output);
}

#[test]
fn mock_simple_missing_direct_leaf() {
    // (Fail) Missing an entry for direct dep that has no children

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = files_full_audited(&metadata);

    audits.audits.get_mut("third-party2").unwrap().clear();

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-missing-direct-leaf", output);
}

#[test]
fn mock_simple_missing_leaves() {
    // (Fail) Missing all leaf audits (but not the internal)

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = files_full_audited(&metadata);

    audits.audits.get_mut("third-party2").unwrap().clear();
    audits
        .audits
        .get_mut("transitive-third-party1")
        .unwrap()
        .clear();

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-missing-leaves", output);
}

#[test]
fn mock_simple_weaker_transitive_req() {
    // (Pass) A third-party dep with weaker requirements on a child dep

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = files_full_audited(&metadata);

    let trans_audits = &mut audits.audits.get_mut("transitive-third-party1").unwrap();
    trans_audits.clear();
    trans_audits.push(full_audit(ver(DEFAULT_VER), "weak-reviewed"));

    let direct_audits = &mut audits.audits.get_mut("third-party1").unwrap();
    direct_audits.clear();
    direct_audits.push(full_audit_dep(
        ver(DEFAULT_VER),
        "reviewed",
        [("transitive-third-party1", ["weak-reviewed"])],
    ));

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-weaker-transitive-req", output);
}

#[test]
fn mock_simple_weaker_transitive_req_using_implies() {
    // (Pass) A third-party dep with weaker requirements on a child dep
    // but the child dep actually has *super* reqs, to check that implies works

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = files_full_audited(&metadata);

    let trans_audits = &mut audits.audits.get_mut("transitive-third-party1").unwrap();
    trans_audits.clear();
    trans_audits.push(full_audit(ver(DEFAULT_VER), "strong-reviewed"));

    let direct_audits = &mut audits.audits.get_mut("third-party1").unwrap();
    direct_audits.clear();
    direct_audits.push(full_audit_dep(
        ver(DEFAULT_VER),
        "reviewed",
        [("transitive-third-party1", ["weak-reviewed"])],
    ));

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-weaker-transitive-req-using-implies", output);
}

#[test]
fn mock_simple_lower_version_review() {
    // (Fail) A dep that has a review but for a lower version.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = files_full_audited(&metadata);

    let direct_audits = &mut audits.audits.get_mut("third-party1").unwrap();
    direct_audits.clear();
    direct_audits.push(full_audit(ver(DEFAULT_VER - 1), DEFAULT_CRIT));

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-lower-version-review", output);
}

#[test]
fn mock_simple_higher_version_review() {
    // (Fail) A dep that has a review but for a higher version.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = files_full_audited(&metadata);

    let direct_audits = &mut audits.audits.get_mut("third-party1").unwrap();
    direct_audits.clear();
    direct_audits.push(full_audit(ver(DEFAULT_VER + 1), DEFAULT_CRIT));

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-higher-version-review", output);
}

#[test]
fn mock_simple_higher_and_lower_version_review() {
    // (Fail) A dep that has a review but for both a higher and lower version.
    // Once I mock out fake diffs it should prefer the lower one because the
    // system will make application size grow quadratically.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = files_full_audited(&metadata);

    let direct_audits = &mut audits.audits.get_mut("third-party1").unwrap();
    direct_audits.clear();
    direct_audits.push(full_audit(ver(DEFAULT_VER - 1), DEFAULT_CRIT));
    direct_audits.push(full_audit(ver(DEFAULT_VER + 1), DEFAULT_CRIT));

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-higher-and-lower-version-review", output);
}

#[test]
fn mock_simple_reviewed_too_weakly() {
    // (Fail) A dep has a review but the criteria is too weak

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = files_full_audited(&metadata);

    let trans_audits = &mut audits.audits.get_mut("transitive-third-party1").unwrap();
    trans_audits.clear();
    trans_audits.push(full_audit(ver(DEFAULT_VER), "weak-reviewed"));

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-reviewed-too-weakly", output);
}

#[test]
fn mock_simple_delta_to_exemptions() {
    // (Pass) A dep has a delta to an exemptions entry

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = files_full_audited(&metadata);

    let direct_audits = &mut audits.audits.get_mut("third-party1").unwrap();
    direct_audits.clear();
    direct_audits.push(delta_audit(
        ver(DEFAULT_VER - 5),
        ver(DEFAULT_VER),
        DEFAULT_CRIT,
    ));

    let direct_exemptions = &mut config.exemptions;
    direct_exemptions.insert(
        "third-party1".to_string(),
        vec![exemptions(ver(DEFAULT_VER - 5), DEFAULT_CRIT)],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-delta-to-unaudited", output);
}

#[test]
fn mock_simple_delta_to_exemptions_overshoot() {
    // (Fail) A dep has a delta but it overshoots the exemptions entry.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = files_full_audited(&metadata);

    let direct_audits = &mut audits.audits.get_mut("third-party1").unwrap();
    direct_audits.clear();
    direct_audits.push(delta_audit(
        ver(DEFAULT_VER - 6),
        ver(DEFAULT_VER),
        DEFAULT_CRIT,
    ));

    let direct_exemptions = &mut config.exemptions;
    direct_exemptions.insert(
        "third-party1".to_string(),
        vec![exemptions(ver(DEFAULT_VER - 5), DEFAULT_CRIT)],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-delta-to-unaudited-overshoot", output);
}

#[test]
fn mock_simple_delta_to_exemptions_undershoot() {
    // (Fail) A dep has a delta but it undershoots the exemptions entry.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = files_full_audited(&metadata);

    let direct_audits = &mut audits.audits.get_mut("third-party1").unwrap();
    direct_audits.clear();
    direct_audits.push(delta_audit(
        ver(DEFAULT_VER - 3),
        ver(DEFAULT_VER),
        DEFAULT_CRIT,
    ));

    let direct_exemptions = &mut config.exemptions;
    direct_exemptions.insert(
        "third-party1".to_string(),
        vec![exemptions(ver(DEFAULT_VER - 5), DEFAULT_CRIT)],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-delta-to-unaudited-undershoot", output);
}

#[test]
fn mock_simple_delta_to_full_audit() {
    // (Pass) A dep has a delta to a fully audited entry

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = files_full_audited(&metadata);

    let direct_audits = &mut audits.audits.get_mut("third-party1").unwrap();
    direct_audits.clear();
    direct_audits.push(delta_audit(
        ver(DEFAULT_VER - 5),
        ver(DEFAULT_VER),
        DEFAULT_CRIT,
    ));
    direct_audits.push(full_audit(ver(DEFAULT_VER - 5), DEFAULT_CRIT));

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-delta-to-full-audit", output);
}

#[test]
fn mock_simple_delta_to_full_audit_overshoot() {
    // (Fail) A dep has a delta to a fully audited entry but overshoots

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = files_full_audited(&metadata);

    let direct_audits = &mut audits.audits.get_mut("third-party1").unwrap();
    direct_audits.clear();
    direct_audits.push(delta_audit(
        ver(DEFAULT_VER - 6),
        ver(DEFAULT_VER),
        DEFAULT_CRIT,
    ));
    direct_audits.push(full_audit(ver(DEFAULT_VER - 5), DEFAULT_CRIT));

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-delta-to-full-audit-overshoot", output);
}

#[test]
fn mock_simple_delta_to_full_audit_undershoot() {
    // (Fail) A dep has a delta to a fully audited entry but undershoots

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = files_full_audited(&metadata);

    let direct_audits = &mut audits.audits.get_mut("third-party1").unwrap();
    direct_audits.clear();
    direct_audits.push(delta_audit(
        ver(DEFAULT_VER - 3),
        ver(DEFAULT_VER),
        DEFAULT_CRIT,
    ));
    direct_audits.push(full_audit(ver(DEFAULT_VER - 5), DEFAULT_CRIT));

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-delta-to-full-audit-undershoot", output);
}

#[test]
fn mock_simple_reverse_delta_to_full_audit() {
    // (Pass) A dep has a *reverse* delta to a fully audited entry

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = files_full_audited(&metadata);

    let direct_audits = &mut audits.audits.get_mut("third-party1").unwrap();
    direct_audits.clear();
    direct_audits.push(delta_audit(
        ver(DEFAULT_VER + 5),
        ver(DEFAULT_VER),
        DEFAULT_CRIT,
    ));
    direct_audits.push(full_audit(ver(DEFAULT_VER + 5), DEFAULT_CRIT));

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-reverse-delta-to-full-audit", output);
}

#[test]
fn mock_simple_reverse_delta_to_exemptions() {
    // (Pass) A dep has a *reverse* delta to an exemptions entry

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = files_full_audited(&metadata);

    let direct_audits = &mut audits.audits.get_mut("third-party1").unwrap();
    direct_audits.clear();
    direct_audits.push(delta_audit(
        ver(DEFAULT_VER + 5),
        ver(DEFAULT_VER),
        DEFAULT_CRIT,
    ));

    let direct_exemptions = &mut config.exemptions;
    direct_exemptions.insert(
        "third-party1".to_string(),
        vec![exemptions(ver(DEFAULT_VER + 5), DEFAULT_CRIT)],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-reverse-delta-to-unaudited", output);
}

#[test]
fn mock_simple_wrongly_reversed_delta_to_exemptions() {
    // (Fail) A dep has a *reverse* delta to an exemptions entry but they needed a normal one

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = files_full_audited(&metadata);

    let direct_audits = &mut audits.audits.get_mut("third-party1").unwrap();
    direct_audits.clear();
    direct_audits.push(delta_audit(
        ver(DEFAULT_VER),
        ver(DEFAULT_VER - 5),
        DEFAULT_CRIT,
    ));

    let direct_exemptions = &mut config.exemptions;
    direct_exemptions.insert(
        "third-party1".to_string(),
        vec![exemptions(ver(DEFAULT_VER - 5), DEFAULT_CRIT)],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-wrongly-reversed-delta-to-unaudited", output);
}

#[test]
fn mock_simple_wrongly_reversed_delta_to_full_audit() {
    // (Fail) A dep has a *reverse* delta to a fully audited entry but they needed a normal one

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = files_full_audited(&metadata);

    let direct_audits = &mut audits.audits.get_mut("third-party1").unwrap();
    direct_audits.clear();
    direct_audits.push(delta_audit(
        ver(DEFAULT_VER),
        ver(DEFAULT_VER - 5),
        DEFAULT_CRIT,
    ));
    direct_audits.push(full_audit(ver(DEFAULT_VER - 5), DEFAULT_CRIT));

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-wrongly-reversed-delta-to-full-audit", output);
}

#[test]
fn mock_simple_needed_reversed_delta_to_exemptions() {
    // (Fail) A dep has a delta to an exemptions entry but they needed a reversed one

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = files_full_audited(&metadata);

    let direct_audits = &mut audits.audits.get_mut("third-party1").unwrap();
    direct_audits.clear();
    direct_audits.push(delta_audit(
        ver(DEFAULT_VER),
        ver(DEFAULT_VER + 5),
        DEFAULT_CRIT,
    ));

    let direct_exemptions = &mut config.exemptions;
    direct_exemptions.insert(
        "third-party1".to_string(),
        vec![exemptions(ver(DEFAULT_VER + 5), DEFAULT_CRIT)],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-needed-reversed-delta-to-unaudited", output);
}

#[test]
fn mock_simple_delta_to_exemptions_too_weak() {
    // (Fail) A dep has a delta to an exemptions entry but it's too weak

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = files_full_audited(&metadata);

    let direct_audits = &mut audits.audits.get_mut("third-party1").unwrap();
    direct_audits.clear();
    direct_audits.push(delta_audit(
        ver(DEFAULT_VER - 5),
        ver(DEFAULT_VER),
        "weak-reviewed",
    ));

    let direct_exemptions = &mut config.exemptions;
    direct_exemptions.insert(
        "third-party1".to_string(),
        vec![exemptions(ver(DEFAULT_VER - 5), DEFAULT_CRIT)],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-delta-to-unaudited-too-weak", output);
}

#[test]
fn mock_simple_delta_to_full_audit_too_weak() {
    // (Fail) A dep has a delta to a fully audited entry but it's too weak

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = files_full_audited(&metadata);

    let direct_audits = &mut audits.audits.get_mut("third-party1").unwrap();
    direct_audits.clear();
    direct_audits.push(delta_audit(
        ver(DEFAULT_VER - 5),
        ver(DEFAULT_VER),
        "weak-reviewed",
    ));
    direct_audits.push(full_audit(ver(DEFAULT_VER - 5), DEFAULT_CRIT));

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-delta-to-full-audit-too-weak", output);
}

#[test]
fn mock_simple_delta_to_too_weak_full_audit() {
    // (Fail) A dep has a delta to a fully audited entry but it's too weak

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = files_full_audited(&metadata);

    let direct_audits = &mut audits.audits.get_mut("third-party1").unwrap();
    direct_audits.clear();
    direct_audits.push(delta_audit(
        ver(DEFAULT_VER - 5),
        ver(DEFAULT_VER),
        DEFAULT_CRIT,
    ));
    direct_audits.push(full_audit(ver(DEFAULT_VER - 5), "weak-reviewed"));

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-simple-delta-to-too-weak-full-audit", output);
}

#[test]
fn mock_complex_inited() {
    // (Fail) Should look the same as a fresh 'vet init' but with all 'exemptions' entries deleted.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::complex();
    let metadata = mock.metadata();
    let (config, audits, imports) = files_inited(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-complex-inited", output);
}

#[test]
fn mock_complex_no_exemptions() {
    // (Fail) Should look the same as a fresh 'vet init' but with all 'exemptions' entries deleted.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::complex();
    let metadata = mock.metadata();
    let (config, audits, imports) = files_no_exemptions(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-complex-no-unaudited", output);
}

#[test]
fn mock_complex_full_audited() {
    // (Pass) All entries have direct full audits.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::complex();
    let metadata = mock.metadata();
    let (config, audits, imports) = files_full_audited(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-complex-full-audited", output);
}

#[test]
fn builtin_complex_inited() {
    // (Fail) Should look the same as a fresh 'vet init' but with all 'exemptions' entries deleted.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::complex();
    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_inited(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-complex-inited", output);
}

#[test]
fn builtin_complex_no_exemptions() {
    // (Fail) Should look the same as a fresh 'vet init' but with all 'exemptions' entries deleted.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::complex();
    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_no_exemptions(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-complex-no-unaudited", output);
}

#[test]
fn builtin_complex_full_audited() {
    // (Pass) All entries have direct full audits.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::complex();
    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_full_audited(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-complex-full-audited", output);
}

#[test]
fn builtin_complex_minimal_audited() {
    // (Pass) All entries have direct full audits.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::complex();
    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_minimal_audited(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-complex-minimal-audited", output);
}

#[test]
fn mock_complex_missing_core5() {
    // (Fail) Missing an audit for the v5 version of third-core

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::complex();
    let metadata = mock.metadata();
    let (config, mut audits, imports) = files_full_audited(&metadata);

    audits.audits.insert(
        "third-core".to_string(),
        vec![full_audit(ver(DEFAULT_VER), "reviewed")],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-complex-missing-core5", output);
}

#[test]
fn mock_complex_missing_core10() {
    // (Fail) Missing an audit for the v10 version of third-core

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::complex();
    let metadata = mock.metadata();
    let (config, mut audits, imports) = files_full_audited(&metadata);

    audits.audits.insert(
        "third-core".to_string(),
        vec![full_audit(ver(5), "reviewed")],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-complex-missing-core10", output);
}

#[test]
fn mock_complex_core10_too_weak() {
    // (Fail) Criteria for core10 is too weak

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::complex();
    let metadata = mock.metadata();
    let (config, mut audits, imports) = files_full_audited(&metadata);

    audits.audits.insert(
        "third-core".to_string(),
        vec![
            full_audit(ver(DEFAULT_VER), "weak-reviewed"),
            full_audit(ver(5), "reviewed"),
        ],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-complex-core10-too-weak", output);
}

#[test]
fn mock_complex_core10_partially_too_weak() {
    // (Fail) Criteria for core10 is too weak for thirdA but not thirdA and thirdAB (full)

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::complex();
    let metadata = mock.metadata();
    let (config, mut audits, imports) = files_full_audited(&metadata);

    audits.audits.insert(
        "third-core".to_string(),
        vec![
            full_audit(ver(DEFAULT_VER), "weak-reviewed"),
            full_audit(ver(5), "reviewed"),
        ],
    );

    let audit_with_weaker_req = full_audit_dep(
        ver(DEFAULT_VER),
        "reviewed",
        [("third-core", ["weak-reviewed"])],
    );
    audits
        .audits
        .insert("thirdA".to_string(), vec![audit_with_weaker_req.clone()]);
    audits
        .audits
        .insert("thirdAB".to_string(), vec![audit_with_weaker_req]);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("mock-complex-core10-partially-too-weak", output);
}

#[test]
fn mock_complex_core10_partially_too_weak_via_weak_delta() {
    // (Fail) Criteria for core10 is too weak for thirdA but not thirdA and thirdAB (weak delta)

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::complex();
    let metadata = mock.metadata();
    let (config, mut audits, imports) = files_full_audited(&metadata);

    audits.audits.insert(
        "third-core".to_string(),
        vec![
            delta_audit(ver(5), ver(DEFAULT_VER), "weak-reviewed"),
            full_audit(ver(5), "reviewed"),
        ],
    );

    let audit_with_weaker_req = full_audit_dep(
        ver(DEFAULT_VER),
        "reviewed",
        [("third-core", ["weak-reviewed"])],
    );
    audits
        .audits
        .insert("thirdA".to_string(), vec![audit_with_weaker_req.clone()]);
    audits
        .audits
        .insert("thirdAB".to_string(), vec![audit_with_weaker_req]);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!(
        "mock-complex-core10-partially-too-weak-via-weak-delta",
        output
    );
}

#[test]
fn mock_complex_core10_partially_too_weak_via_strong_delta() {
    // (Fail) Criteria for core10 is too weak for thirdA but not thirdA and thirdAB
    // because there's a strong delta from 5->10 but 0->5 is still weak!

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::complex();
    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = files_full_audited(&metadata);

    audits.audits.insert(
        "third-core".to_string(),
        vec![
            delta_audit(ver(5), ver(DEFAULT_VER), "reviewed"),
            full_audit(ver(5), "weak-reviewed"),
        ],
    );

    let audit_with_weaker_req = full_audit_dep(
        ver(DEFAULT_VER),
        "reviewed",
        [("third-core", ["weak-reviewed"])],
    );
    audits
        .audits
        .insert("thirdA".to_string(), vec![audit_with_weaker_req.clone()]);
    audits
        .audits
        .insert("thirdAB".to_string(), vec![audit_with_weaker_req]);

    config.policy.insert(
        "firstA".to_string(),
        dep_policy([("third-core", ["weak-reviewed"])]),
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!(
        "mock-complex-core10-partially-too-weak-via-strong-delta",
        output
    );
}

#[test]
fn mock_simple_policy_root_too_strong() {
    // (Fail) Root policy is too strong

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();
    let metadata = mock.metadata();
    let (mut config, audits, imports) = files_full_audited(&metadata);

    config
        .policy
        .insert("root-package".to_string(), self_policy(["strong-reviewed"]));

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("simple-policy-root-too-strong", output);
}

#[test]
fn mock_simple_policy_root_weaker() {
    // (Pass) Root policy weaker than necessary

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();
    let metadata = mock.metadata();
    let (mut config, audits, imports) = files_full_audited(&metadata);

    config
        .policy
        .insert("root-package".to_string(), self_policy(["weak-reviewed"]));

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("simple-policy-root-weaker", output);
}

#[test]
fn mock_simple_policy_first_too_strong() {
    // (Fail) First-party policy is too strong

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();
    let metadata = mock.metadata();
    let (mut config, audits, imports) = files_full_audited(&metadata);

    config
        .policy
        .insert("first-party".to_string(), self_policy(["strong-reviewed"]));

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("simple-policy-first-too-strong", output);
}

#[test]
fn mock_simple_policy_first_weaker() {
    // (Pass) First-party policy weaker than necessary

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();
    let metadata = mock.metadata();
    let (mut config, audits, imports) = files_full_audited(&metadata);

    config
        .policy
        .insert("first-party".to_string(), self_policy(["weak-reviewed"]));

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("simple-policy-first-weaker", output);
}

#[test]
fn mock_simple_policy_root_dep_weaker() {
    // (Pass) root->first-party policy weaker than necessary

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();
    let metadata = mock.metadata();
    let (mut config, audits, imports) = files_full_audited(&metadata);

    config.policy.insert(
        "root-package".to_string(),
        dep_policy([("first-party", ["weak-reviewed"])]),
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("simple-policy-root-dep-weaker", output);
}

#[test]
fn mock_simple_policy_root_dep_too_strong() {
    // (Pass) root->first-party policy stronger than necessary

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();
    let metadata = mock.metadata();
    let (mut config, audits, imports) = files_full_audited(&metadata);

    config.policy.insert(
        "root-package".to_string(),
        dep_policy([("first-party", ["strong-reviewed"])]),
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("simple-policy-root-dep-too-strong", output);
}

#[test]
fn mock_simple_policy_first_dep_weaker() {
    // (Pass) first-party->third-party policy weaker than necessary

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();
    let metadata = mock.metadata();
    let (mut config, audits, imports) = files_full_audited(&metadata);

    config.policy.insert(
        "first-party".to_string(),
        dep_policy([("third-party1", ["weak-reviewed"])]),
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("simple-policy-first-dep-weaker", output);
}

#[test]
fn mock_simple_policy_first_dep_too_strong() {
    // (Pass) first-party->third-party policy too strong

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();
    let metadata = mock.metadata();
    let (mut config, audits, imports) = files_full_audited(&metadata);

    config.policy.insert(
        "first-party".to_string(),
        dep_policy([("third-party1", ["strong-reviewed"])]),
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("simple-policy-first-dep-too-strong", output);
}

#[test]
fn mock_simple_policy_first_dep_stronger() {
    // (Pass) first-party->third-party policy stronger but satisfied

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();
    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = files_full_audited(&metadata);

    config.policy.insert(
        "first-party".to_string(),
        dep_policy([("third-party2", ["strong-reviewed"])]),
    );

    audits.audits.insert(
        "third-party2".to_string(),
        vec![full_audit(ver(DEFAULT_VER), "strong-reviewed")],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("simple-policy-first-dep-stronger", output);
}

#[test]
fn mock_simple_policy_first_dep_weaker_needed() {
    // (Pass) first-party->third-party policy weaker out of necessity

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();
    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = files_full_audited(&metadata);

    config.policy.insert(
        "first-party".to_string(),
        dep_policy([("third-party1", ["weak-reviewed"])]),
    );

    audits.audits.insert(
        "third-party1".to_string(),
        vec![full_audit(ver(DEFAULT_VER), "weak-reviewed")],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("simple-policy-first-dep-weaker-needed", output);
}

#[test]
fn mock_simple_policy_first_dep_extra() {
    // (Pass) first-party->third-party policy has extra satisfied criteria

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();
    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = files_full_audited(&metadata);

    config.policy.insert(
        "first-party".to_string(),
        dep_policy([("third-party2", ["reviewed", "fuzzed"])]),
    );

    audits.audits.insert(
        "third-party2".to_string(),
        vec![
            full_audit(ver(DEFAULT_VER), "reviewed"),
            full_audit(ver(DEFAULT_VER), "fuzzed"),
        ],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("simple-policy-first-dep-extra", output);
}

#[test]
fn mock_simple_policy_first_dep_extra_missing() {
    // (Fail) first-party->third-party policy has extra unsatisfied criteria

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();
    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = files_full_audited(&metadata);

    config.policy.insert(
        "first-party".to_string(),
        dep_policy([("third-party2", ["reviewed", "fuzzed"])]),
    );

    audits.audits.insert(
        "third-party2".to_string(),
        vec![full_audit(ver(DEFAULT_VER), "reviewed")],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("simple-policy-first-dep-extra-missing", output);
}

#[test]
fn mock_simple_policy_first_extra_partially_missing() {
    // (Fail) first-party policy has extra unsatisfied criteria

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();
    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = files_full_audited(&metadata);

    config.policy.insert(
        "first-party".to_string(),
        self_policy(["reviewed", "fuzzed"]),
    );

    audits.audits.insert(
        "third-party2".to_string(),
        vec![
            full_audit(ver(DEFAULT_VER), "reviewed"),
            full_audit(ver(DEFAULT_VER), "fuzzed"),
        ],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("simple-policy-first-extra-partially-missing", output);
}

#[test]
fn mock_simple_first_policy_redundant() {
    // (Pass) first-party policy has redundant implied things

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();
    let metadata = mock.metadata();
    let (mut config, audits, imports) = files_full_audited(&metadata);

    config.policy.insert(
        "first-party".to_string(),
        self_policy(["reviewed", "weak-reviewed"]),
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("simple-policy-first-policy-redundant", output);
}

#[test]
fn builtin_simple_deps_inited() {
    // (Pass) Should look the same as a fresh 'vet init'.
    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple_deps();

    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_inited(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-deps-init", output);
}

#[test]
fn builtin_simple_deps_no_exemptions() {
    // (Fail) Should look the same as a fresh 'vet init' but with all 'exemptions' entries deleted.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple_deps();

    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_no_exemptions(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-deps-no-unaudited", output);
}

#[test]
fn builtin_simple_deps_full_audited() {
    // (Pass) All entries have direct full audits.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple_deps();

    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_full_audited(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-deps-full-audited", output);
}

#[test]
fn builtin_simple_deps_minimal_audited() {
    // (Pass) All entries have direct full audits.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple_deps();

    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_minimal_audited(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-deps-minimal-audited", output);
}

#[test]
fn builtin_no_deps() {
    // (Pass) No actual deps
    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::new(vec![MockPackage {
        name: "root-package",
        is_workspace: true,
        is_first_party: true,
        deps: vec![],
        ..Default::default()
    }]);

    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_full_audited(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-no-deps", output);
}

#[test]
fn builtin_only_first_deps() {
    // (Pass) No actual deps
    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::new(vec![
        MockPackage {
            name: "root-package",
            is_workspace: true,
            is_first_party: true,
            deps: vec![dep("first-party")],
            ..Default::default()
        },
        MockPackage {
            name: "first-party",
            is_first_party: true,
            deps: vec![],
            ..Default::default()
        },
    ]);

    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_full_audited(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-only-first-deps", output);
}

#[test]
fn builtin_cycle_inited() {
    // (Pass) Should look the same as a fresh 'vet init'.
    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::cycle();

    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_inited(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-cycle-inited", output);
}

#[test]
fn builtin_cycle_exemptions() {
    // (Fail) Should look the same as a fresh 'vet init' but with all 'exemptions' entries deleted.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::cycle();

    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_no_exemptions(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-cycle-unaudited", output);
}

#[test]
fn builtin_cycle_full_audited() {
    // (Pass) All entries have direct full audits.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::cycle();

    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_full_audited(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-cycle-full-audited", output);
}

#[test]
fn builtin_cycle_minimal_audited() {
    // (Pass) All entries have direct full audits.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::cycle();

    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_minimal_audited(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-cycle-minimal-audited", output);
}

#[test]
fn builtin_dev_detection() {
    // (Pass) Check that we properly identify things that are or aren't only dev-deps,
    // even when they're indirect or used in both contexts.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::dev_detection();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = builtin_files_no_exemptions(&metadata);
    audits.audits.insert(
        "normal".to_string(),
        vec![full_audit(ver(DEFAULT_VER), SAFE_TO_DEPLOY)],
    );
    audits.audits.insert(
        "both".to_string(),
        vec![full_audit(ver(DEFAULT_VER), SAFE_TO_DEPLOY)],
    );
    audits.audits.insert(
        "simple-dev".to_string(),
        vec![full_audit(ver(DEFAULT_VER), SAFE_TO_RUN)],
    );
    audits.audits.insert(
        "simple-dev-indirect".to_string(),
        vec![full_audit(ver(DEFAULT_VER), SAFE_TO_RUN)],
    );
    audits.audits.insert(
        "dev-cycle-direct".to_string(),
        vec![full_audit(ver(DEFAULT_VER), SAFE_TO_RUN)],
    );
    audits.audits.insert(
        "dev-cycle-indirect".to_string(),
        vec![full_audit(ver(DEFAULT_VER), SAFE_TO_RUN)],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-dev-detection", output);
}

#[test]
fn builtin_dev_detection_empty() {
    // (Fail) same as above but without any audits to confirm expectations

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::dev_detection();

    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_no_exemptions(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-dev-detection-empty", output);
}

#[test]
fn builtin_dev_detection_empty_deeper() {
    // (Fail) same as above but deeper

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::dev_detection();

    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_no_exemptions(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Deep);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-dev-detection-empty-deeper", output);
}

#[test]
fn builtin_simple_exemptions_extra() {
    // (Pass) there's an extra unused exemptions entry, but the other is needed
    // (This test could warn if we try to detect "useless exemptions" eagerly)

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert("third-party1".to_string(), vec![]);

    config.exemptions.insert(
        "third-party1".to_string(),
        vec![
            exemptions(ver(5), SAFE_TO_DEPLOY),
            exemptions(ver(DEFAULT_VER), SAFE_TO_DEPLOY),
        ],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-unaudited-extra", output);
}

#[test]
fn builtin_simple_exemptions_not_a_real_dep() {
    // (Pass) there's an exemptions entry for a package that isn't in our tree at all.
    // (This test could warn if we try to detect "useless exemptions" eagerly)

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (mut config, audits, imports) = builtin_files_full_audited(&metadata);

    config.exemptions.insert(
        "fake-dep".to_string(),
        vec![exemptions(ver(DEFAULT_VER), SAFE_TO_DEPLOY)],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-not-a-real-dep", output);
}

#[test]
fn builtin_simple_deps_exemptions_overbroad() {
    // (Pass) the exemptions entry is needed but it's overbroad
    // (This test could warn if we try to detect "useless exemptions" eagerly)

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple_deps();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert("dev".to_string(), vec![]);

    config.exemptions.insert(
        "dev".to_string(),
        vec![exemptions(ver(DEFAULT_VER), SAFE_TO_DEPLOY)],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-unaudited-overbroad", output);
}

#[test]
fn builtin_complex_exemptions_twins() {
    // (Pass) two versions of a crate exist and both are exemptions and they're needed

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::complex();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert("third-core".to_string(), vec![]);

    config.exemptions.insert(
        "third-core".to_string(),
        vec![
            exemptions(ver(DEFAULT_VER), SAFE_TO_DEPLOY),
            exemptions(ver(5), SAFE_TO_DEPLOY),
        ],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-unaudited-twins", output);
}

#[test]
fn builtin_complex_exemptions_partial_twins() {
    // (Pass) two versions of a crate exist and one is exemptions and one is audited

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::complex();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert(
        "third-core".to_string(),
        vec![full_audit(ver(5), SAFE_TO_DEPLOY)],
    );

    config.exemptions.insert(
        "third-core".to_string(),
        vec![exemptions(ver(DEFAULT_VER), SAFE_TO_DEPLOY)],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-unaudited-partial-twins", output);
}

#[test]
fn builtin_simple_exemptions_in_delta() {
    // (Pass) An audited entry overlaps a delta and isn't needed
    // (This test could warn if we try to detect "useless exemptions" eagerly)

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert(
        "third-party1".to_string(),
        vec![
            full_audit(ver(3), SAFE_TO_DEPLOY),
            delta_audit(ver(3), ver(5), SAFE_TO_DEPLOY),
            delta_audit(ver(5), ver(DEFAULT_VER), SAFE_TO_DEPLOY),
        ],
    );

    config.exemptions.insert(
        "third-party1".to_string(),
        vec![exemptions(ver(5), SAFE_TO_DEPLOY)],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-unaudited-in-delta", output);
}

#[test]
fn builtin_simple_exemptions_in_full() {
    // (Pass) An audited entry overlaps a full audit and isn't needed
    // (This test could warn if we try to detect "useless exemptions" eagerly)

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert(
        "third-party1".to_string(),
        vec![
            full_audit(ver(3), SAFE_TO_DEPLOY),
            delta_audit(ver(3), ver(5), SAFE_TO_DEPLOY),
            delta_audit(ver(5), ver(DEFAULT_VER), SAFE_TO_DEPLOY),
        ],
    );

    config.exemptions.insert(
        "third-party1".to_string(),
        vec![exemptions(ver(3), SAFE_TO_DEPLOY)],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-unaudited-in-full", output);
}

#[test]
fn builtin_simple_exemptions_in_direct_full() {
    // (Pass) An audited entry overlaps a full audit which is the cur version and isn't needed
    // (This test used to warn when we tried to detect "useless exemptions" eagerly)

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert(
        "third-party1".to_string(),
        vec![full_audit(ver(DEFAULT_VER), SAFE_TO_DEPLOY)],
    );

    config.exemptions.insert(
        "third-party1".to_string(),
        vec![exemptions(ver(DEFAULT_VER), SAFE_TO_DEPLOY)],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-unaudited-in-direct-full", output);
}

#[test]
fn builtin_simple_exemptions_nested_weaker_req() {
    // (Pass) A dep that has weaker requirements on its dep
    // including dependency_criteria on an exemptions entry

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert(
        "third-party1".to_string(),
        vec![
            delta_audit_dep(
                ver(3),
                ver(6),
                SAFE_TO_DEPLOY,
                [("transitive-third-party1", [SAFE_TO_RUN])],
            ),
            delta_audit_dep(
                ver(6),
                ver(DEFAULT_VER),
                SAFE_TO_DEPLOY,
                [("transitive-third-party1", [SAFE_TO_RUN])],
            ),
        ],
    );
    audits.audits.insert(
        "transitive-third-party1".to_string(),
        vec![
            delta_audit(ver(4), ver(8), SAFE_TO_RUN),
            delta_audit(ver(8), ver(DEFAULT_VER), SAFE_TO_RUN),
        ],
    );

    config.exemptions.insert(
        "third-party1".to_string(),
        vec![exemptions_dep(
            ver(3),
            SAFE_TO_DEPLOY,
            [("transitive-third-party1", [SAFE_TO_RUN])],
        )],
    );

    config.exemptions.insert(
        "transitive-third-party1".to_string(),
        vec![exemptions(ver(4), SAFE_TO_RUN)],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-unaudited-nested-weaker-req", output);
}

#[test]
fn builtin_simple_exemptions_nested_weaker_req_needs_dep_criteria() {
    // (Fail) A dep that has weaker requirements on its dep
    // but the exemptions entry is missing that so the whole thing fails

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert(
        "third-party1".to_string(),
        vec![
            delta_audit_dep(
                ver(3),
                ver(6),
                SAFE_TO_DEPLOY,
                [("transitive-third-party1", [SAFE_TO_RUN])],
            ),
            delta_audit_dep(
                ver(6),
                ver(DEFAULT_VER),
                SAFE_TO_DEPLOY,
                [("transitive-third-party1", [SAFE_TO_RUN])],
            ),
        ],
    );
    audits.audits.insert(
        "transitive-third-party1".to_string(),
        vec![
            delta_audit(ver(4), ver(8), SAFE_TO_RUN),
            delta_audit(ver(8), ver(DEFAULT_VER), SAFE_TO_RUN),
        ],
    );

    config.exemptions.insert(
        "third-party1".to_string(),
        vec![exemptions(ver(3), SAFE_TO_DEPLOY)],
    );

    config.exemptions.insert(
        "transitive-third-party1".to_string(),
        vec![exemptions(ver(4), SAFE_TO_RUN)],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!(
        "builtin-simple-unaudited-nested-weaker-req-needs-dep-criteria",
        output
    );
}

#[test]
fn builtin_simple_exemptions_nested_stronger_req() {
    // (Pass) A dep that has stronger requirements on its dep

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = builtin_files_full_audited(&metadata);

    config.policy.insert(
        "first-party".to_string(),
        dep_policy([("third-party1", [SAFE_TO_RUN])]),
    );

    audits.audits.insert(
        "third-party1".to_string(),
        vec![
            delta_audit_dep(
                ver(3),
                ver(6),
                SAFE_TO_RUN,
                [("transitive-third-party1", [SAFE_TO_DEPLOY])],
            ),
            delta_audit_dep(
                ver(6),
                ver(DEFAULT_VER),
                SAFE_TO_RUN,
                [("transitive-third-party1", [SAFE_TO_DEPLOY])],
            ),
        ],
    );
    audits.audits.insert(
        "transitive-third-party1".to_string(),
        vec![
            delta_audit(ver(4), ver(8), SAFE_TO_DEPLOY),
            delta_audit(ver(8), ver(DEFAULT_VER), SAFE_TO_DEPLOY),
        ],
    );

    config.exemptions.insert(
        "third-party1".to_string(),
        vec![exemptions(ver(3), SAFE_TO_RUN)],
    );

    config.exemptions.insert(
        "transitive-third-party1".to_string(),
        vec![exemptions(ver(4), SAFE_TO_DEPLOY)],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-unaudited-nested-stronger-req", output);
}

#[test]
fn builtin_simple_deps_exemptions_adds_uneeded_criteria() {
    // (Pass) An audited entry overlaps a full audit which is the cur version and isn't needed
    // (This test could warn if we try to detect "useless exemptions" eagerly)

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple_deps();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert(
        "dev".to_string(),
        vec![
            full_audit(ver(5), SAFE_TO_RUN),
            delta_audit(ver(5), ver(DEFAULT_VER), SAFE_TO_DEPLOY),
        ],
    );

    config
        .exemptions
        .insert("dev".to_string(), vec![exemptions(ver(5), SAFE_TO_DEPLOY)]);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!(
        "builtin-simple-deps-unaudited-adds-uneeded-criteria",
        output
    );
}

#[test]
fn builtin_dev_detection_exemptions_adds_uneeded_criteria_indirect() {
    // (Pass) An audited entry overlaps a full audit which is the cur version and isn't needed
    // (This test could warn if we try to detect "useless exemptions" eagerly)

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::dev_detection();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = builtin_files_minimal_audited(&metadata);

    audits.audits.insert(
        "simple-dev-indirect".to_string(),
        vec![
            full_audit(ver(5), SAFE_TO_RUN),
            delta_audit(ver(5), ver(DEFAULT_VER), SAFE_TO_DEPLOY),
        ],
    );

    config.exemptions.insert(
        "simple-dev-indirect".to_string(),
        vec![exemptions(ver(5), SAFE_TO_DEPLOY)],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!(
        "builtin-dev-detection-unaudited-adds-uneeded-criteria-indirect",
        output
    );
}

#[test]
fn builtin_dev_detection_cursed_full() {
    // (Pass): dev-indirect has safe-to-run and by policy we only need safe-to-run
    // but dev (its parent) is audited for safe-to-deploy which naively requires the child
    // be safe-to-deploy. However criteria "decomposition" makes this ok, and we do succesfully
    // validate for safe-to-run.
    //
    // This test is "cursed" because it caused some crashes in glitched out the blame system.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::dev_detection();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert(
        "simple-dev-indirect".to_string(),
        vec![
            full_audit(ver(5), SAFE_TO_RUN),
            delta_audit(ver(5), ver(DEFAULT_VER), SAFE_TO_RUN),
            delta_audit(ver(5), ver(DEFAULT_VER), SAFE_TO_DEPLOY),
        ],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-dev-detection-cursed-full", output);
}

#[test]
fn builtin_dev_detection_cursed_minimal() {
    // (Pass): the same as the full cursed one, but without the cursed part.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::dev_detection();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = builtin_files_minimal_audited(&metadata);

    audits.audits.insert(
        "simple-dev-indirect".to_string(),
        vec![
            full_audit(ver(5), SAFE_TO_RUN),
            delta_audit(ver(5), ver(DEFAULT_VER), SAFE_TO_RUN),
            delta_audit(ver(5), ver(DEFAULT_VER), SAFE_TO_DEPLOY),
        ],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-dev-detection-cursed-minimal", output);
}

#[test]
fn builtin_simple_delta_cycle() {
    // (Pass) simple delta cycle

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert(
        "third-party1".to_string(),
        vec![
            full_audit(ver(3), SAFE_TO_DEPLOY),
            delta_audit(ver(3), ver(5), SAFE_TO_DEPLOY),
            delta_audit(ver(5), ver(7), SAFE_TO_DEPLOY),
            delta_audit(ver(7), ver(5), SAFE_TO_DEPLOY),
            delta_audit(ver(7), ver(DEFAULT_VER), SAFE_TO_DEPLOY),
        ],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-delta-cycle", output);
}

#[test]
fn builtin_simple_noop_delta() {
    // (Pass) completely pointless noop delta
    // (This test could warn if we try to detect "useless deltas")

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert(
        "third-party1".to_string(),
        vec![
            full_audit(ver(3), SAFE_TO_DEPLOY),
            delta_audit(ver(3), ver(5), SAFE_TO_DEPLOY),
            delta_audit(ver(5), ver(5), SAFE_TO_DEPLOY),
            delta_audit(ver(5), ver(7), SAFE_TO_DEPLOY),
            delta_audit(ver(7), ver(DEFAULT_VER), SAFE_TO_DEPLOY),
        ],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-noop-delta", output);
}

#[test]
fn builtin_simple_delta_double_cycle() {
    // (Pass) double delta cycle

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert(
        "third-party1".to_string(),
        vec![
            full_audit(ver(2), SAFE_TO_DEPLOY),
            delta_audit(ver(2), ver(3), SAFE_TO_DEPLOY),
            delta_audit(ver(3), ver(4), SAFE_TO_DEPLOY),
            delta_audit(ver(4), ver(3), SAFE_TO_DEPLOY),
            delta_audit(ver(4), ver(5), SAFE_TO_DEPLOY),
            delta_audit(ver(5), ver(6), SAFE_TO_DEPLOY),
            delta_audit(ver(6), ver(5), SAFE_TO_DEPLOY),
            delta_audit(ver(6), ver(7), SAFE_TO_DEPLOY),
            delta_audit(ver(7), ver(DEFAULT_VER), SAFE_TO_DEPLOY),
        ],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-delta-double-cycle", output);
}

#[test]
fn builtin_simple_delta_broken_double_cycle() {
    // (Fail) double delta cycle that's broken

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert(
        "third-party1".to_string(),
        vec![
            full_audit(ver(2), SAFE_TO_DEPLOY),
            delta_audit(ver(2), ver(3), SAFE_TO_DEPLOY),
            delta_audit(ver(3), ver(4), SAFE_TO_DEPLOY),
            delta_audit(ver(4), ver(3), SAFE_TO_DEPLOY),
            // broken: delta_audit(ver(4), ver(5), SAFE_TO_DEPLOY),
            delta_audit(ver(5), ver(6), SAFE_TO_DEPLOY),
            delta_audit(ver(6), ver(5), SAFE_TO_DEPLOY),
            delta_audit(ver(6), ver(7), SAFE_TO_DEPLOY),
            delta_audit(ver(7), ver(DEFAULT_VER), SAFE_TO_DEPLOY),
        ],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-delta-broken-double-cycle", output);
}

#[test]
fn builtin_simple_delta_broken_cycle() {
    // (Fail) simple delta cycle that's broken

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert(
        "third-party1".to_string(),
        vec![
            full_audit(ver(3), SAFE_TO_DEPLOY),
            delta_audit(ver(3), ver(5), SAFE_TO_DEPLOY),
            delta_audit(ver(5), ver(7), SAFE_TO_DEPLOY),
            delta_audit(ver(7), ver(5), SAFE_TO_DEPLOY),
            // broken: delta_audit(ver(7), ver(8), SAFE_TO_DEPLOY),
            delta_audit(ver(8), ver(DEFAULT_VER), SAFE_TO_DEPLOY),
        ],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-delta-broken-cycle", output);
}

#[test]
fn builtin_simple_long_cycle() {
    // (Pass) long delta cycle

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert(
        "third-party1".to_string(),
        vec![
            full_audit(ver(2), SAFE_TO_DEPLOY),
            delta_audit(ver(2), ver(7), SAFE_TO_DEPLOY),
            delta_audit(ver(7), ver(6), SAFE_TO_DEPLOY),
            delta_audit(ver(6), ver(8), SAFE_TO_DEPLOY),
            delta_audit(ver(8), ver(7), SAFE_TO_DEPLOY),
            delta_audit(ver(8), ver(DEFAULT_VER), SAFE_TO_DEPLOY),
        ],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-long-cycle", output);
}

#[test]
fn builtin_simple_useless_long_cycle() {
    // (Pass) useless long delta cycle

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert(
        "third-party1".to_string(),
        vec![
            full_audit(ver(2), SAFE_TO_DEPLOY),
            delta_audit(ver(2), ver(7), SAFE_TO_DEPLOY),
            delta_audit(ver(7), ver(6), SAFE_TO_DEPLOY),
            delta_audit(ver(6), ver(8), SAFE_TO_DEPLOY),
            delta_audit(ver(8), ver(7), SAFE_TO_DEPLOY),
            delta_audit(ver(7), ver(DEFAULT_VER), SAFE_TO_DEPLOY),
        ],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-useless-long-cycle", output);
}

#[test]
fn builtin_haunted_init() {
    // (Pass) Should look the same as a fresh 'vet init'.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::haunted_tree();

    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_inited(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);
    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-haunted-init", output);
}

#[test]
fn builtin_haunted_no_exemptions() {
    // (Fail) Should look the same as a fresh 'vet init' but with all 'exemptions' entries deleted.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::haunted_tree();

    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_no_exemptions(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-haunted-no-unaudited", output);
}

#[test]
fn builtin_haunted_no_exemptions_deeper() {
    // (Fail) Should look the same as a fresh 'vet init' but with all 'exemptions' entries deleted.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::haunted_tree();

    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_no_exemptions(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Deep);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-haunted-no-unaudited-deeper", output);
}

#[test]
fn builtin_haunted_full_audited() {
    // (Pass) All entries have direct full audits.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::haunted_tree();

    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_full_audited(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-haunted-full-audited", output);
}

#[test]
fn builtin_haunted_minimal_audited() {
    // (Pass) All entries have direct minimal audits.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::haunted_tree();

    let metadata = mock.metadata();
    let (config, audits, imports) = builtin_files_minimal_audited(&metadata);

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-haunted-minimal-audited", output);
}

#[test]
fn builtin_simple_audit_as_default_root_no_audit() {
    // (Fail) the root is audit-as-crates-io but has no audits

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (mut config, audits, imports) = builtin_files_inited(&metadata);

    config
        .policy
        .insert("root-package".to_string(), audit_as_policy(Some(true)));

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-audit-as-default-root-no-audit", output);
}

#[test]
fn builtin_simple_audit_as_default_root() {
    // (Pass) the root is audit-as-crates-io and only needs to be safe-to-run

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (mut config, audits, imports) = builtin_files_inited(&metadata);

    config
        .policy
        .insert("root-package".to_string(), audit_as_policy(Some(true)));
    config.exemptions.insert(
        "root-package".to_string(),
        vec![exemptions(ver(DEFAULT_VER), SAFE_TO_DEPLOY)],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-audit-as-default-root", output);
}

#[test]
fn builtin_simple_audit_as_default_root_too_weak() {
    // (Fail) the root is audit-as-crates-io but is only safe-to-run

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (mut config, audits, imports) = builtin_files_inited(&metadata);

    config
        .policy
        .insert("root-package".to_string(), audit_as_policy(Some(true)));
    config.exemptions.insert(
        "root-package".to_string(),
        vec![exemptions(ver(DEFAULT_VER), SAFE_TO_RUN)],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-audit-as-default-root-too-weak", output);
}

#[test]
fn builtin_simple_audit_as_weaker_root() {
    // (Pass) the root is audit-as-crates-io and only needs to be safe-to-run

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (mut config, audits, imports) = builtin_files_inited(&metadata);

    config.policy.insert(
        "root-package".to_string(),
        PolicyEntry {
            criteria: Some(vec![SAFE_TO_RUN.to_string().into()]),
            ..audit_as_policy(Some(true))
        },
    );
    config.exemptions.insert(
        "root-package".to_string(),
        vec![exemptions(ver(DEFAULT_VER), SAFE_TO_RUN)],
    );

    let store = Store::mock(config, audits, imports);
    let report = crate::resolver::resolve(&metadata, None, &store, ResolveDepth::Shallow);

    let output = get_report(&metadata, report);
    insta::assert_snapshot!("builtin-simple-audit-as-weaker-root", output);
}
