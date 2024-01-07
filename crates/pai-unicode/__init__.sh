#!/bin/bash

set -ex

CURRENT_DIR=$(dirname $0)

mkdir -p ${CURRENT_DIR}/"__download__"

BASE_UNICODE_UDC_URL="https://www.unicode.org/Public/UCD/latest/ucd"

fn_download_unicode_file()
{
    file=$1
    curl -o ${CURRENT_DIR}/__download__/${file} ${BASE_UNICODE_UDC_URL}/${file}
}

fn_download_unicode_file ReadMe.txt
fn_download_unicode_file DerivedCoreProperties.txt
