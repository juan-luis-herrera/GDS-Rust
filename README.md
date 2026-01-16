# Greedy Decision System - GDS

## What is this repository?

This is an implementation for the GDS system, as proposed in the Transactions on Services Computing paper, using Rust. It includes both the Rust client for the service API, and the GDS implementation itself.

Documentation and packaging files are also included.

## What do I need to run this?

At the very least, you will need a container runtime. `docker` is recommended, as it was used for testing and development, but `podman` may work too. `docker-compose` is recommended to run the container specification. You can also set up the containers manually or through other tools to follow the specification.

You will need a Service API instance to connect to, as well as a parameter that you want to modify and a SLO you want to use for checking.

## Repository structure

- `ServiceAPIClient`: Client for the Service API. [It has its own documentation](ServiceAPIClient/README.md)
- `test-api`: Implementation of GDS. [It has its own documentation](test-api/README.md)
- `batch-openapi-generator.yaml`: Configuration for OpenAPI generator.
- `docker-compose.yaml`: Docker Compose specification for the scenario.
- `Dockerfile`: File to build a Docker image for this repository
- `README.md`: Current file

## Setup details

### Tools and data used

- Tool used to generate the basis of the Service API: `openapi-generator` 7.18.0 ([Arch Linux package](https://archlinux.org/packages/extra/any/openapi-generator/))
- Tool used to manage the container deployment: `docker-compose` 5.0.1 ([Arch Linux package](https://archlinux.org/packages/extra/x86_64/docker-compose/))

### Instructions

1. Edit `docker-compose.yaml` to fit your infrastructure. You should point it to your Service API instance (`API_HOST`), set the SLO to check (`SLO_ID`) and the parameter to adjust (`CONF_PARAM`), as well as how long you wish to sleep for (`SLEEP_INTERVAL`). To simplify reconfiguration, the ranges for the SLO (`SLO_MIN`, `SLO_MAX`) and parameter (`PARAM_MIN`, `PARAM_MAX`) should be declared here too. The correlation (`PARAM_CORRELATION`) and change rate (`PARAM_CHANGE`) should be set too. A maximum number of steps (`NUM_STEPS`) can be declared too, leave it unset or give it a value lesser or equal to 0 to make the agent execute indefinitely. The results will be output to a file at `/results/Results.csv` within the container, so you can mount a volume accordingly to preserve them.
2. Start up the agent: `docker-compose up -d`.
3. GDS will be managing your configuration.