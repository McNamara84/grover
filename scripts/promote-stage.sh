#!/usr/bin/env bash
set -euo pipefail

image_ref=${1:?Expected the verified GHCR image with its digest}
compose=docker-compose.stage.yml
deploy_ref=refs/heads/deploy/stage
source_sha=$(git rev-parse HEAD)
digest=${image_ref##*@}
template="${image_ref%@*}:deployment-template"

if [[ ! "$image_ref" =~ @sha256:[0-9a-f]{64}$ ]] || [[ $(docker compose -f "$compose" config --images) != "$template" ]]; then
    echo "The verified image does not match the Stage template: $image_ref" >&2
    exit 1
fi
sed -i "/^[[:space:]]*image:/s|:deployment-template|@$digest|" "$compose"
docker compose -f "$compose" config --quiet
if [[ $(docker compose -f "$compose" config --images) != "$image_ref" ]]; then
    echo "The rendered Compose file must reference exactly $image_ref" >&2
    exit 1
fi

git add -- "$compose"
tree=$(git write-tree)
subject="Deploy Stage from $source_sha"

remote_tip() {
    git ls-remote --heads origin "$1" | cut -f1
}

for attempt in 1 2 3; do
    previous=$(remote_tip "$deploy_ref")
    parents=(-p "$source_sha")
    if [[ -n "$previous" ]]; then
        git fetch --quiet --no-tags origin "$deploy_ref"
        previous=$(git rev-parse FETCH_HEAD)
        if [[ $(git rev-parse "$previous^{tree}") == "$tree" && $(git log -1 --format=%s "$previous") == "$subject" ]]; then
            echo "Stage already references $source_sha at $image_ref." >> "$GITHUB_STEP_SUMMARY"
            exit 0
        fi
        parents=(-p "$previous" -p "$source_sha")
    fi

    commit=$(git -c user.name='github-actions[bot]' -c user.email='41898282+github-actions[bot]@users.noreply.github.com' \
        commit-tree "$tree" "${parents[@]}" -m "$subject" -m "$image_ref")
    dev_head=$(remote_tip refs/heads/dev)
    if [[ "$dev_head" != "$source_sha" ]]; then
        echo "Skipped $source_sha because dev moved on." >> "$GITHUB_STEP_SUMMARY"
        exit 0
    fi
    if git push --force-with-lease="$deploy_ref:$previous" origin "$commit:$deploy_ref"; then
        printf 'Portainer can now pull deployment %s for dev commit %s.\n\nVerified image %s\n' \
            "$commit" "$source_sha" "$image_ref" >> "$GITHUB_STEP_SUMMARY"
        exit 0
    fi
    # Retry only a competing update. Surface permission and transport failures.
    competing_tip=$(remote_tip "$deploy_ref")
    if [[ "$competing_tip" == "$previous" ]]; then
        exit 1
    fi
    echo "Deployment branch changed during attempt $attempt." >&2
done
exit 1
