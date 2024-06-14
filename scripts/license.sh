#!/usr/bin/env bash

#    Copyright 2024 Ibrahim Mbaziira
#
#    Licensed under the Apache License, Version 2.0 (the "License");
#    you may not use this file except in compliance with the License.
#    You may obtain a copy of the License at
#
#        http://www.apache.org/licenses/LICENSE-2.0
#
#    Unless required by applicable law or agreed to in writing, software
#    distributed under the License is distributed on an "AS IS" BASIS,
#    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#    See the License for the specific language governing permissions and
#    limitations under the License.

# files to exclude
exclude_pattern='target|.vscode|node_modules|.idea'

files=()
echo "Crawling source code .🔎."
echo 'Crawling *.rs|cargo.toml files .🔎.'
files+=($(find . -type f \( -name '*.rs' -o -name 'cargo.toml' \) -print | egrep -v ${exclude_pattern}))
echo "Crawling html|js|ts|css files .🔎."
files+=($(find . -type f \( -name '*.html' -o -name '*.css' -o -name '*.ts' -o -name '*.js' -o -name '*.scss' \) -print | egrep -v ${exclude_pattern}))
echo "Crawling shell script files .🔎."
files+=($(find . -type f \( -name '*.sh' -o -name '*.bash' -o -name '*.ksh' -o -name '*.csh' -o -name '*.tcsh' -o -name '*.fsh' \) -print | egrep -v ${exclude_pattern}))
echo "Crawling text files.🔎."
files+=($(find . -type f -name '*.txt' -print | egrep -v ${exclude_pattern}))
echo "Crawling yml|yaml|.ignore files .🔎."
files+=($(find . -type f \( -name '*.yaml' -o -name '*.yml' \) -print | egrep -v ${exclude_pattern}))
files+=($(find . -type f -name '.dockerignore' -print | egrep -v ${exclude_pattern}))
files+=($(find . -type f -name '.gitignore' -print | egrep -v ${exclude_pattern}))
echo "Crawling make files.🔎."
files+=($(find . -type f -name 'Makefile' -print | egrep -v ${exclude_pattern}))

copyright_notice="Copyright 2024 Ibrahim Mbaziira"

no_license_header=0

echo "verifying ${#files[@]} source code files license headers .🔎."
for file in "${files[@]}"; do
  head -4 "${file}" | grep -q "${copyright_notice}"
  exit_code=$?
  if [[ ${exit_code} -ne 0 ]]; then
    echo "${file} is missing a license header ❌"
    no_license_header=1
  fi
done

if [[ ${no_license_header} -eq 1 ]]; then
  echo "‼️ some files are missing a license header."
else
  echo "License verification successful."
fi

exit ${no_license_header}
