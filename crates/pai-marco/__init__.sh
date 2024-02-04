#!/bin/bash

set -ex

CURRENT_DIR=$(dirname $0)

mkdir -p ${CURRENT_DIR}/__cache__

cargo rustc --package pai-marco --test run -- -Z unpretty=expanded > ${CURRENT_DIR}/__cache__/code.rs