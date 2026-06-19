#!/usr/bin/env bash

mkdir -p .local
cd .local || ( echo ".local directory does not exists" && exit )

OS_TYPE=$(uname -s)

for i in {0..7}; do
  if [ "$OS_TYPE" = "Darwin" ]; then
      TIME_STAMP=$(date -v-$((i+1))d +"%Y-%m-%dT%H:%M:%S")
  else
      TIME_STAMP=$(date -d "$((i+1)) days ago" +"%Y-%m-%dT%H:%M:%S")
  fi

  REPO_DIR="${PWD}/repo_0$i"
  if [[ -d "${REPO_DIR}" ]]; then
    echo "${REPO_DIR} already exists. Deleting...."
    rm -rf "${REPO_DIR}"
  fi

  mkdir -p "$REPO_DIR"
  git -C "$REPO_DIR" init -b master

  if [[ "$i" == 0 ]]; then
    echo "${REPO_DIR}/file-1" > "${REPO_DIR}/file"
    git -C "$REPO_DIR" add .
    GIT_AUTHOR_DATE=${TIME_STAMP} GIT_COMMITTER_DATE=${TIME_STAMP} git -C "$REPO_DIR" commit -m "first commit"

    if [[ -d "${PWD}/repo_bare_0${i}" ]]; then
      echo "${PWD}/repo_bare_0${i} already exists. Deleting...."
      rm -rf "${PWD}/repo_bare_0${i}"
    fi
    git clone --bare "${PWD}/repo_0${i}" "${PWD}/repo_bare_0${i}"
  fi

  if [[ "$i" == [1-2] ]]; then
    echo "${REPO_DIR}/file-1" > "${REPO_DIR}/file"
    git -C "$REPO_DIR" add .
    GIT_AUTHOR_DATE=${TIME_STAMP} GIT_COMMITTER_DATE=${TIME_STAMP} git -C "$REPO_DIR" commit -m "first commit"
  fi

  if [[ "$i" == [3-5] ]]; then
    echo "${REPO_DIR}/file-1" > "${REPO_DIR}/file"
    git -C "$REPO_DIR" add .
    GIT_AUTHOR_DATE=${TIME_STAMP} GIT_COMMITTER_DATE=${TIME_STAMP} git -C "$REPO_DIR" commit -m "first commit"
    git -C "$REPO_DIR" checkout -b "feature/feature-${i}"
  fi

  if [[ "$i" == 6 ]]; then
    echo "${REPO_DIR}/file-1" > "${REPO_DIR}/file"
    git -C "$REPO_DIR" add .
    GIT_AUTHOR_DATE=${TIME_STAMP} GIT_COMMITTER_DATE=${TIME_STAMP} git -C "$REPO_DIR" commit -m "first commit"
    echo "${REPO_DIR}/file-2" > "${REPO_DIR}/file-2"
    git -C "$REPO_DIR" add .
    GIT_AUTHOR_DATE=${TIME_STAMP} GIT_COMMITTER_DATE=${TIME_STAMP} git -C "$REPO_DIR" commit -m "second commit"
    git -C "$REPO_DIR" checkout "$(git -C "$REPO_DIR" rev-list --max-parents=0 HEAD)"

    if [[ -d "${PWD}/repo_bare_0${i}" ]]; then
      echo "${PWD}/repo_bare_0${i} already exists. Deleting...."
      rm -rf "${PWD}/repo_bare_0${i}"
    fi
    git clone --bare "${PWD}/repo_0${i}" "${PWD}/repo_bare_0${i}"
  fi
  if [[ "$i" == 6 ]]; then
    if [[ -d "${PWD}/repo_bare_0${i}" ]]; then
      echo "${PWD}/repo_bare_0${i} already exists. Deleting...."
      rm -rf "${PWD}/repo_bare_0${i}"
    fi
    git clone --bare "${PWD}/repo_0${i}" "${PWD}/repo_bare_0${i}"
  fi
done
