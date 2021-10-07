use helper_ecr_login_auto;
use helper_ecr_login_auto::find_aws_profile;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_find_aws_profile() {
    let home_dir = tempdir().unwrap();
    let aws_path = home_dir.path().join(".aws");
    let config_path = &aws_path.join("config");

    let config = r###"
        [default]
        region = eu-central-1

        [profile testing]
        credential_process = aws-credential-process --mfa-oath-slot "aws/iam/123456:user" --mfa-serial-number arn:aws:iam::456789:mfa/user --assume-role-arn arn:aws:iam::123456789012:role/NewRole
        region = eu-central-1
    "###;

    fs::create_dir(&aws_path).unwrap();
    fs::write(config_path, config).unwrap();

    let err = Vec::new();
    let profile = find_aws_profile(
        "123456789012.dkr.ecr.eu-central-1.amazonaws.com",
        err,
        Some(PathBuf::from(&home_dir.path())),
    );

    assert!(profile.is_ok());
    let ok_profile = profile.unwrap();
    assert!(ok_profile.is_some());
    assert_eq!(&ok_profile.unwrap(), "testing");

    home_dir.close().unwrap();
}
