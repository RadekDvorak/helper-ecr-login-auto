# helper-ecr-login-auto

Helper-ecr-login-auto is a wrapper around [amazon-ecr-credential-helper](https://github.com/awslabs/amazon-ecr-credential-helper). 
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
image is accessed by docker. It searches `credential_process` and `role_arn` ini keys for a role in the same
account as the image. If there is a match the profile is exported to the `AWS_PROFILE` environment variable
and for the `ecr-login` credential helper to use.

If there already is the `AWS_PROFILE` environment variable this helper does not interfere.

### Customizations
This application can be controlled using environment variables
- `ECR_LOGIN_APP` is used to configure the credentials helper. Defaults to `docker-credential-ecr-login`.
- `ENV_NAME` is used to specify an aws profile name. It defaults to `AWS_PROFILE`, you may choose to use e.g. `AWS_DEFAULT_PROFILE` instead.
- `ARN_CONFIG_KEY` is not used if aws profile is known explicitly (see `ENV_NAME`). If there is no selected profile, this helper
looks for an assumable role in records with this ini key. The heuristic expects to find a `arn:aws:iam::NUMERIC_ACCOUNT_ID:role/SOME_ROLE`
pattern in the value. 

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.
