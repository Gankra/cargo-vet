use super::*;

fn get_unaudited(store: &Store) -> String {
    toml_edit::ser::to_string_pretty(&store.config.unaudited).unwrap()
}

#[test]
fn builtin_simple_unaudited_not_a_real_dep_regenerate() {
    // (Pass) there's an unaudited entry for a package that isn't in our tree at all.
    // Should strip the result and produce an empty unaudited file.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (mut config, audits, imports) = builtin_files_full_audited(&metadata);

    config.unaudited.insert(
        "fake-dep".to_string(),
        vec![unaudited(ver(DEFAULT_VER), SAFE_TO_DEPLOY)],
    );

    let mut store = Store::mock(config, audits, imports);
    let cfg = mock_cfg(&metadata);
    crate::minimize_unaudited(&cfg, &mut store, None).unwrap();

    let unaudited = get_unaudited(&store);
    insta::assert_snapshot!("builtin-simple-not-a-real-dep-regenerate", unaudited);
}

#[test]
fn builtin_simple_deps_unaudited_overbroad_regenerate() {
    // (Pass) the unaudited entry is needed but it's overbroad
    // Should downgrade from safe-to-deploy to safe-to-run

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple_deps();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert("dev".to_string(), vec![]);

    config.unaudited.insert(
        "dev".to_string(),
        vec![unaudited(ver(DEFAULT_VER), SAFE_TO_DEPLOY)],
    );

    let mut store = Store::mock(config, audits, imports);
    let cfg = mock_cfg(&metadata);
    crate::minimize_unaudited(&cfg, &mut store, None).unwrap();

    let unaudited = get_unaudited(&store);
    insta::assert_snapshot!("builtin-simple-unaudited-overbroad-regenerate", unaudited);
}

#[test]
fn builtin_complex_unaudited_twins_regenerate() {
    // (Pass) two versions of a crate exist and both are unaudited and they're needed
    // Should be a no-op and both entries should remain

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::complex();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert("third-core".to_string(), vec![]);

    config.unaudited.insert(
        "third-core".to_string(),
        vec![
            unaudited(ver(DEFAULT_VER), SAFE_TO_DEPLOY),
            unaudited(ver(5), SAFE_TO_DEPLOY),
        ],
    );

    let mut store = Store::mock(config, audits, imports);
    let cfg = mock_cfg(&metadata);
    crate::minimize_unaudited(&cfg, &mut store, None).unwrap();

    let unaudited = get_unaudited(&store);
    insta::assert_snapshot!("builtin-simple-unaudited-twins-regenerate", unaudited);
}

#[test]
fn builtin_complex_unaudited_partial_twins_regenerate() {
    // (Pass) two versions of a crate exist and one is unaudited and one is audited
    // Should be a no-op and both entries should remain

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::complex();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert(
        "third-core".to_string(),
        vec![full_audit(ver(5), SAFE_TO_DEPLOY)],
    );

    config.unaudited.insert(
        "third-core".to_string(),
        vec![unaudited(ver(DEFAULT_VER), SAFE_TO_DEPLOY)],
    );

    let mut store = Store::mock(config, audits, imports);
    let cfg = mock_cfg(&metadata);
    crate::minimize_unaudited(&cfg, &mut store, None).unwrap();

    let unaudited = get_unaudited(&store);
    insta::assert_snapshot!(
        "builtin-simple-unaudited-partial-twins-regenerate",
        unaudited
    );
}

#[test]
fn builtin_simple_unaudited_in_delta_regenerate() {
    // (Pass) An audited entry overlaps a delta and isn't needed
    // Should emit an empty unaudited file

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

    config.unaudited.insert(
        "third-party1".to_string(),
        vec![unaudited(ver(5), SAFE_TO_DEPLOY)],
    );

    let mut store = Store::mock(config, audits, imports);
    let cfg = mock_cfg(&metadata);
    crate::minimize_unaudited(&cfg, &mut store, None).unwrap();

    let unaudited = get_unaudited(&store);
    insta::assert_snapshot!("builtin-simple-unaudited-in-delta-regenerate", unaudited);
}

#[test]
fn builtin_simple_unaudited_in_full_regenerate() {
    // (Pass) An audited entry overlaps a full audit and isn't needed
    // Should emit an empty unaudited file

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

    config.unaudited.insert(
        "third-party1".to_string(),
        vec![unaudited(ver(3), SAFE_TO_DEPLOY)],
    );

    let mut store = Store::mock(config, audits, imports);
    let cfg = mock_cfg(&metadata);
    crate::minimize_unaudited(&cfg, &mut store, None).unwrap();

    let unaudited = get_unaudited(&store);
    insta::assert_snapshot!("builtin-simple-unaudited-in-full-regenerate", unaudited);
}

#[test]
fn builtin_simple_deps_unaudited_adds_uneeded_criteria_regenerate() {
    // (Pass) An audited entry overlaps a full audit which is the cur version and isn't needed
    // Should produce an empty unaudited

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
        .unaudited
        .insert("dev".to_string(), vec![unaudited(ver(5), SAFE_TO_DEPLOY)]);

    let mut store = Store::mock(config, audits, imports);
    let cfg = mock_cfg(&metadata);
    crate::minimize_unaudited(&cfg, &mut store, None).unwrap();

    let unaudited = get_unaudited(&store);
    insta::assert_snapshot!(
        "builtin-simple-deps-unaudited-adds-uneeded-criteria-regenerate",
        unaudited
    );
}

#[test]
fn builtin_dev_detection_unaudited_adds_uneeded_criteria_indirect_regenerate() {
    // (Pass) An audited entry overlaps a full audit which is the cur version and isn't needed
    // Should result in an empty unaudited file

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::dev_detection();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = builtin_files_minimal_audited(&metadata);

    audits.audits.insert(
        "simple-dev-indirect".to_string(),
        vec![
            full_audit(ver(5), SAFE_TO_RUN),
            delta_audit(ver(5), ver(DEFAULT_VER), SAFE_TO_RUN),
            delta_audit(ver(5), ver(DEFAULT_VER), SAFE_TO_DEPLOY),
        ],
    );

    config.unaudited.insert(
        "simple-dev-indirect".to_string(),
        vec![unaudited(ver(5), SAFE_TO_DEPLOY)],
    );

    let mut store = Store::mock(config, audits, imports);
    let cfg = mock_cfg(&metadata);
    crate::minimize_unaudited(&cfg, &mut store, None).unwrap();

    let unaudited = get_unaudited(&store);
    insta::assert_snapshot!(
        "builtin-dev-detection-unaudited-adds-uneeded-criteria-indirect-regenerate",
        unaudited
    );
}

#[test]
fn builtin_simple_unaudited_extra_regenerate() {
    // (Pass) there's an extra unused unaudited entry, but the other is needed.
    // Should result in only the v10 unaudited entry remaining.

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert("third-party1".to_string(), vec![]);

    config.unaudited.insert(
        "third-party1".to_string(),
        vec![
            unaudited(ver(5), SAFE_TO_DEPLOY),
            unaudited(ver(DEFAULT_VER), SAFE_TO_DEPLOY),
        ],
    );

    let mut store = Store::mock(config, audits, imports);
    let cfg = mock_cfg(&metadata);
    crate::minimize_unaudited(&cfg, &mut store, None).unwrap();

    let unaudited = get_unaudited(&store);
    insta::assert_snapshot!("builtin-simple-unaudited-extra-regenerate", unaudited);
}

#[test]
fn builtin_simple_unaudited_in_direct_full_regenerate() {
    // (Pass) An audited entry overlaps a full audit which is the cur version and isn't needed
    // Should produce an empty unaudited

    let _enter = TEST_RUNTIME.enter();
    let mock = MockMetadata::simple();

    let metadata = mock.metadata();
    let (mut config, mut audits, imports) = builtin_files_full_audited(&metadata);

    audits.audits.insert(
        "third-party1".to_string(),
        vec![full_audit(ver(DEFAULT_VER), SAFE_TO_DEPLOY)],
    );

    config.unaudited.insert(
        "third-party1".to_string(),
        vec![unaudited(ver(DEFAULT_VER), SAFE_TO_DEPLOY)],
    );

    let mut store = Store::mock(config, audits, imports);
    let cfg = mock_cfg(&metadata);
    crate::minimize_unaudited(&cfg, &mut store, None).unwrap();

    let unaudited = get_unaudited(&store);
    insta::assert_snapshot!(
        "builtin-simple-unaudited-in-direct-full-regenerate",
        unaudited
    );
}

#[test]
fn builtin_simple_unaudited_nested_weaker_req_regenerate() {
    // (Pass) A dep that has weaker requirements on its dep
    // BUSTED: doesn't emit dependency-criteria for third-party1's 'unaudited'

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

    config.unaudited.insert(
        "third-party1".to_string(),
        vec![unaudited_dep(
            ver(3),
            SAFE_TO_DEPLOY,
            [("transitive-third-party1", [SAFE_TO_RUN])],
        )],
    );

    config.unaudited.insert(
        "transitive-third-party1".to_string(),
        vec![unaudited(ver(4), SAFE_TO_RUN)],
    );

    let mut store = Store::mock(config, audits, imports);
    let cfg = mock_cfg(&metadata);
    crate::minimize_unaudited(&cfg, &mut store, None).unwrap();

    let unaudited = get_unaudited(&store);
    insta::assert_snapshot!(
        "builtin-simple-unaudited-nested-weaker-req-regenerate",
        unaudited
    );
}

#[test]
fn builtin_simple_unaudited_nested_stronger_req_regenerate() {
    // (Pass) A dep that has stronger requirements on its dep
    // BUSTED: should emit safe-to-deploy for transitive-third-party1

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

    config.unaudited.insert(
        "third-party1".to_string(),
        vec![unaudited(ver(3), SAFE_TO_RUN)],
    );

    config.unaudited.insert(
        "transitive-third-party1".to_string(),
        vec![unaudited(ver(4), SAFE_TO_DEPLOY)],
    );

    let mut store = Store::mock(config, audits, imports);
    let cfg = mock_cfg(&metadata);
    crate::minimize_unaudited(&cfg, &mut store, None).unwrap();

    let unaudited = get_unaudited(&store);
    insta::assert_snapshot!(
        "builtin-simple-unaudited-nested-stronger-req-regenerate",
        unaudited
    );
}
