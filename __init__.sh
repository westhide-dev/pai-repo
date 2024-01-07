#!/bin/bash

set -ex

CURRENT_DIR=$(dirname $0)

fn_init_crates()
{
    for file in $(find crates -name "__init__.sh")
    do
        bash $file
    done
}

fn_init_crates