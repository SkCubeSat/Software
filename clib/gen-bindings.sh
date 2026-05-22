#!/bin/bash
bindgen_cmd="bindgen --use-core wrapper.h -- '-I./libcsp/include' '-I./cfg' '-I./libcsp/src' > bindings.rs"
echo "Running: $bindgen_cmd"
eval $bindgen_cmd
