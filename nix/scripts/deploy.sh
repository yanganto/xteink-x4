#!/usr/bin/env bash
# Deploy and monitor the device(default)

cd $(git rev-parse --show-toplevel 2>/dev/null)

# Display help if --help or -h is passed
if [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --only     Deploy only, without monitoring"
    echo "  --help, -h  Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0              # Deploy and monitor (default)"
    echo "  $0 --only       # Deploy only"
    exit 0
fi

# Check if --only option is passed
if [ "$1" = "--only" ]; then
    # Deploy only
    echo "Deploying only..."
    espflash flash --chip esp32c3 target/riscv32imc-unknown-none-elf/release/xteink-x4
else
    # Deploy and monitor
    echo "Deploying and monitoring..."
    espflash flash --chip esp32c3 --monitor target/riscv32imc-unknown-none-earlease/xteink-x4
fi
