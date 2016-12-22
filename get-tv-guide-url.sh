#!/bin/bash

set -euf -o pipefail

name=$(echo $1 | tr ' ' '+')
curl -s "http://www.tvguide.com/search/media/?keyword=${name}" | pup -c '.search-result-objects-item json{}'  | jq '.[0].children | .[0].href'

