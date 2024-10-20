#!/bin/bash

# Check if the user provided an argument (path to the assembly file)
if [ $# -eq 0 ]; then
    echo "Usage: ./rv32-binary {path_to_assembly_file}"
    exit 1
fi

# Get the full file path from the first argument
FILE_PATH=$1

# Extract the directory and the file name without the extension
FILE_DIR=$(dirname "$FILE_PATH")
FILE_NAME=$(basename "$FILE_PATH" .s)

# Assemble the assembly file into an object file in the same directory
riscv64-unknown-elf-as -march=rv32i -o "${FILE_DIR}/${FILE_NAME}.o" "$FILE_PATH"

# Link the object file to create an executable in the same directory
riscv64-unknown-elf-ld -m elf32lriscv -o "${FILE_DIR}/${FILE_NAME}" "${FILE_DIR}/${FILE_NAME}.o"

# Generate the binary file in the same directory as the .s file
riscv64-unknown-elf-objcopy -O binary "${FILE_DIR}/${FILE_NAME}" "${FILE_DIR}/${FILE_NAME}.bin"

# Display success message and binary output path
echo "Binary generated: ${FILE_DIR}/${FILE_NAME}.bin"
