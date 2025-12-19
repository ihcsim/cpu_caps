#!/bin/bash

source ./obsolete.sh

out_path="./testdata"
node_name="isim-dev"
usable_models=($(yq -oy ${out_path}/virsh_domcapabilities.xml | yq '.domainCapabilities.cpu.mode.[].model.[] | select(.+@usable=="yes").+content'))
supported_models=""
declare -A global_supported_models
for usable_model in "${usable_models[@]}"; do
  obsolete="false"
  for obsolete_cpu in "${!obsolete_cpu_models[@]}"; do
    if [ "${usable_model}" = "${obsolete_cpu}" ]; then
      obsolete="true"
    fi
  done

  if [ "${obsolete}" = "false" ]; then
    if [ -z "${supported_models}" ]; then
      supported_models="${usable_model}"
    else
      supported_models="${supported_models} ${usable_model}"
    fi

    if [ -z "${global_supported_models[${usable_model}]}" ]; then
      global_supported_models[${usable_model}]="${node_name}"
    else
      global_supported_models[${usable_model}]="${global_supported_models[${usable_model}]} ${node_name}"
    fi
  fi
done
formatted=$(echo "${supported_models}" | sed -z 's/ /\n    - /g')
echo "- ${formatted}"

global_entry="global:
  supported_cpu_models:"
for model in "${!global_supported_models[@]}"; do
  global_entry="${global_entry}\n  - ${model}: ${global_supported_models[${model}]}"
done
echo -e "${global_entry}"
