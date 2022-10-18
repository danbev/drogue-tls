#!/bin/bash

key_name=sscs-tool
private_key=$key_name
github_user=drogue-iot
project_name=embedded-tls
github_url=https://github.com/$github_user/${project_name}.git
workdir=work

## Create a work directory for all artifacts
mkdir ${workdir}
pushd ${workdir} > /dev/null

# Generate a keypair to be used with in-toto commands
# TODO: figure out how this can be by using sigstore so that we can verify
# the identity of the user doing this.
echo "1) Generating keypair named $key_name"
in-toto-keygen $private_key

echo "2) Cloning $github_url"
in-toto-run --step-name clone_project -k $private_key -- git clone $github_url

echo "3) Create new branch named version_update_branch"
in-toto-record start --step-name create_branch --key $private_key --materials $project_name
pushd $project_name > /dev/null
git checkout -b version_update_branch
popd > /dev/null
in-toto-record stop --step-name create_branch --key $private_key --products $project_name

echo "4) Update Cargo.toml version using cargo-bump"
pushd $project_name > /dev/null
cargo install -q cargo-bump > /dev/null
popd > /dev/null
in-toto-record start --step-name update-version --key $private_key --materials ${project_name}/Cargo.toml ${project_name}/Cargo.lock
## Just updating the patch version while testing this out
pushd $project_name > /dev/null
cargo bump patch
cargo build -q
git add Cargo.toml Cargo.lock 2> /dev/null
git ci -S -m "Bumped version"
popd > /dev/null
in-toto-record stop --step-name update-version --key $private_key --products ${project_name}/Cargo.toml ${project_name}/Cargo.lock

echo "5) Run tests"
cargo test -q --manifest-path=${project_name}/Cargo.toml --no-run
in-toto-run --step-name run_tests -k $private_key -- cargo test --manifest-path ${project_name}/Cargo.toml

popd > /dev/null

echo "Generated artifacts in $workdir"
