#!/bin/bash
# Extract repository or folder name
REPO_FLDR_NAME=$(basename "$(git rev-parse --show-toplevel 2>/dev/null || pwd)")
# Export to .env file for docker-compose to use

cp example.env .env
echo "REPO_FLDR_NAME=$REPO_FLDR_NAME" >> .env

# # Add or update REPO_FLDR_NAME in the parent .env file
# ENV_FILE=".env"
# if [ ! -f "$ENV_FILE" ]; then
#     touch "$ENV_FILE"
# fi
# if grep -q "^REPO_FLDR_NAME=" "$ENV_FILE"; then
#     sed -i "s/^REPO_FLDR_NAME=.*/REPO_FLDR_NAME=$REPO_FLDR_NAME/" "$ENV_FILE"
# else
#     echo "REPO_FLDR_NAME=$REPO_FLDR_NAME" >> "$ENV_FILE"
# fi
