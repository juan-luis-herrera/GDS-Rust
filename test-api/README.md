# GDS rust implementation

This is the Rust implementation of GDS.

## Module structure

- `src/main.rs`: GDS logic.
- `Cargo.toml`: Crate definition.
- `README.md`: This file.
- `run.sh`: Script used by the Dockerfile to run the crate.

## Code structure

- `main()`: Main function of the program. Will load environment variables and follow GDS' logic. Check the pseudocode on the paper for further details.
- `get_slo(api_config, slo_id)`: Wrapper function over `ServiceAPIClient` to retrieve the value of the SLO safely. It will return the value of the SLO `slo_id` as an `Option<i64>`, using `api_config` as an underlying client configuration.
- `get_param(api_config, param_id)`: Wrapper function over `ServiceAPIClient` to retrieve the value of the parameter safely. It will return the value of the parameter `param_id` as an `Option<i32>`, using `api_config` as an underlying client configuration.
- `get_param(api_config, param_id, new_value)`: Wrapper function over `ServiceAPIClient` to modify the value of the parameter safely. It will set the value of `param_id` to `new_value` using `api_config` as an underlying client configuration, and return the API's response as a `Result`.

## Environment variables

| Variable | Expected type | Description | Default |
| --- | --- | --- | --- |
| `SLO_ID` | String | ID of the SLO to monitor | ServiceSLO |
| `CONF_PARAM` | String | ID of the configuration parameter to monitor | ServiceParam |
| `SLEEP_INTERVAL` | Unsigned integer | Time (in seconds) to wait per step | 10 |
| `SLO_MIN` | Decimal | Minimum value for the SLO to be considered as fulfilled | 22 |
| `SLO_MAX` | Decimal | Maximum value for the SLO to be considered as fulfilled | 30 |
| `PARAM_MIN` | Integer | Minimum valid value for the parameter | 0 |
| `PARAM_MAX` | Integer | Maximum valid value for the parameter | 16 |
| `PARAM_CORRELATION` | -1, 1 | Correlation between SLO and parameter | 1 |
| `PARAM_CHANGE` | Integer | Maximum rate of change to the parameter | 1 |
| `API_HOST` | String | URL to the ServiceAPI API | `http://localhost:8080/v0` |
| `NUM_STEPS` | Integer | Maximum number of steps. If set to 0 or less, an undetermined number of steps will be taken | 0 |
| `NTFY_REPORT` | String | If set, messages will be sent to this `ntfy.sh` URL when GDS is started and finished | |