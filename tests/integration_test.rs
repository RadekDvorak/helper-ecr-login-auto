use helper_ecr_login_auto::find_aws_profile;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use helper_ecr_login_auto::myenv::MockEnv;
use tempfile::tempdir;

#[test]
fn test_find_aws_profile() {
    let home_dir = tempdir().unwrap();
    let aws_path = home_dir.path().join(".aws");
    let config_path = &aws_path.join("config");

    let config = r###"
        [default]
        #region = eu-west-1
        output = json
        mfa_serial = arn:aws:iam::123456789012:mfa/radek.dvorak
        vegas_yubikey_serial = 5555555
        vegas_yubikey_label = aws/iam/123456789012:radek.dvorak

        [profile foo-sandbox]
        region = eu-west-1
        credential_process = /home/radek/bin/vegas-credentials assume --profile=foo-sandbox
        vegas_role_arn = arn:aws:iam::000000000000:role/MyRole
        vegas_source_profile = default
        role_session_name = radek.dvorak
        mfa_serial = arn:aws:iam::123456789012:mfa/radek.dvorak
        duration_seconds = 28800


        [profile foo-prod]
        region = eu-west-1
        credential_process = /home/radek/bin/vegas-credentials assume --profile=foo-prod
        vegas_role_arn = arn:aws:iam::888888888888:role/MyRole
        vegas_source_profile = default
        role_session_name = radek.dvorak
        mfa_serial = arn:aws:iam::123456789012:mfa/radek.dvorak
        duration_seconds = 28800
    "###;

    fs::create_dir(&aws_path).unwrap();
    fs::write(config_path, config).unwrap();

    let mocked_env = MockEnv(HashMap::new());
    let err = Vec::new();
    let profile = find_aws_profile(
        "888888888888.dkr.ecr.eu-west-1.amazonaws.com",
        err,
        Some(PathBuf::from(&home_dir.path())),
        mocked_env,
    );

    assert!(profile.is_ok());
    let ok_profile = profile.unwrap();
    assert!(ok_profile.is_some());
    assert_eq!(&ok_profile.unwrap(), "foo-prod");

    home_dir.close().unwrap();
}
