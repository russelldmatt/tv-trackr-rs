#!/bin/bash

set -euf -o pipefail

url=$1
curl "$url" | pup '.button-picker-option-active text{}' | grep -i season
