#!/usr/bin/env bash
set -euo pipefail

project=$(pwd -P)
temp_parent=$(cd "${TMPDIR:-/tmp}" && pwd -P)
test_dir=$(mktemp -d "$temp_parent/grover-stage-test.XXXXXXXX")
trap '[[ "$test_dir" == "$temp_parent"/grover-stage-test.* ]] && rm -rf -- "$test_dir"' EXIT
export GITHUB_STEP_SUMMARY="$test_dir/summary"

git init -q --bare "$test_dir/origin.git"
git init -q -b dev "$test_dir/source"
cd "$test_dir/source"
git config user.name 'Stage test'
git config user.email 'stage@example.invalid'
git remote add origin "$test_dir/origin.git"
cp "$project/docker-compose.stage.yml" .
git add docker-compose.stage.yml
git commit -qm 'First source commit'
git push -q origin dev

# Each attempt gets a clean checkout, as it would on a GitHub runner.
promote() (
    checkout=$(mktemp -d "$test_dir/checkout.XXXXXXXX")
    git worktree add -q --detach "$checkout" "$1"
    cd "$checkout"
    bash "$project/scripts/promote-stage.sh" "$2"
)

first_source=$(git rev-parse HEAD)
image=ghcr.io/mcnamara84/grover
first_image="$image@sha256:$(printf '%064d' 1)"
second_image="$image@sha256:$(printf '%064d' 2)"
deploy_ref=refs/remotes/origin/deploy/stage

promote "$first_source" "$first_image"
first_deploy=$(git rev-parse "$deploy_ref")
test "$(git rev-parse "$first_deploy^")" = "$first_source"
test "$(git show "$first_deploy:docker-compose.stage.yml" | docker compose -p grover-test -f - config --images)" = "$first_image"
promote "$first_source" "$first_image"
test "$(git rev-parse "$deploy_ref")" = "$first_deploy"

git commit -q --allow-empty -m 'Second source commit'
git push -q origin dev
second_source=$(git rev-parse HEAD)
promote "$second_source" "$second_image"
second_deploy=$(git rev-parse "$deploy_ref")
test "$(git rev-parse "$second_deploy^1")" = "$first_deploy"
test "$(git rev-parse "$second_deploy^2")" = "$second_source"

promote "$first_source" "$first_image"
test "$(git rev-parse "$deploy_ref")" = "$second_deploy"
if promote "$second_source" "$image:latest"; then
    echo 'Promotion accepted a mutable tag.' >&2
    exit 1
fi
test "$(git rev-parse "$deploy_ref")" = "$second_deploy"
echo 'Stage promotion checks passed.'
