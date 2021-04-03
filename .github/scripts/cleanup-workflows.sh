#!/usr/bin/env bash -e

OWNER="vikin91"
REPO="yaml2env"

# RUN MANUALLY
# gh auth login

WORKFLOW_IDS=($(gh api -X GET /repos/$OWNER/$REPO/actions/workflows | jq '.workflows[] | .id' ))

for id in ${WORKFLOW_IDS[@]}; do
    RUNS=($(gh api -X GET /repos/$OWNER/$REPO/actions/workflows/${id}/runs | jq '.workflow_runs[] | .id'))
    for run in ${RUNS[@]}; do
        echo "Processing ID/RUN: $id/${run}"
        gh api --silent -X DELETE "/repos/$OWNER/$REPO/actions/runs/${run}"
    done
done
