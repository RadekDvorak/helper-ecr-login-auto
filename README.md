# helper-ecr-login-auto

Helper-ecr-login-auto is a wrapper
around [amazon-ecr-credential-helper](https://github.com/awslabs/amazon-ecr-credential-helper).
It automatically detects the appropriate AWS profile to use.

## Installation

Place the released binary `docker-credential-ecr-login-auto` somewhere in your PATH.

## Usage

Substitute `ecr-login` with `ecr-login-auto` in your `$HOME/.docker/config.json`.
Omit any `docker-credential-` prefix as it is prepended by docker itself at the execution time.

```json
{
  "credHelpers": {
    "AWS_ACCOUNT_ID.dkr.ecr.REGION.amazonaws.com": "ecr-login-auto"
  }
}
```

Helper-ecr-login-auto reads your `$HOME/.aws/config` and tries to find the appropriate profile when a private ECR
image is accessed by docker. It searches `credential_process`, `vegas_role_arn` and `role_arn` ini keys for a role
in the same account as the image. If there is a match the profile is exported to the `AWS_PROFILE` environment
variable and for the `ecr-login` credential helper to use.

If there already is the `AWS_PROFILE` environment variable this helper does NOT honor it. Use `ELA_FORCED_PROFILE`
environment variable to force a specific profile.

This helper should not be called directly by users. It is invoked by docker-compatible apps when needed.

### Customizations

This app prefixes its environment variables with ELA_ (ECR Login Auto).

This application can be controlled using environment variables

- `ELA_UPSTREAM_AUTH_APP` is used to configure the credentials helper. Defaults to `docker-credential-ecr-login`. The
  discovered profile is passed in `AWS_PROFILE` environment variable to the helper.
- `ELA_FORCED_PROFILE` is used to specify an aws profile name.
- `ELA_ARN_CONFIG_KEY` is an ini key name corresponding to the value containing an assumable role in an AWS config file.
  The heuristic expects to find a `arn:aws:iam::NUMERIC_ACCOUNT_ID:role/SOME_ROLE` pattern in the value.
- `ELA_LOG_LEVEL` is used to set the logging level. Possible values are `error`, `warn`, `info`, `debug`, `trace` and
  `off`. Defaults to `info`.
- `ELA_LOG_STYLE` is used to set the logging style. Possible values are `always`, `never`, and `auto`.
  Defaults to `auto`.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.
