#!/bin/sh -xe

# allow caller to override MAPPED_DIR, but default if they don't
MAPPED_DIR="${MAPPED_DIR:-/build}"

# pass the Travis environment into (a file in the) docker environment
env > travis_env

# Run tests in Container
docker run --privileged -d -ti -e "container=docker"  -v /sys/fs/cgroup:/sys/fs/cgroup -v "$(pwd)":"$MAPPED_DIR":rw centos:centos7 /usr/sbin/init
DOCKER_CONTAINER_ID=$(docker ps | grep centos | awk '{print $1}')
docker logs "$DOCKER_CONTAINER_ID"
docker exec -ti "$DOCKER_CONTAINER_ID" /bin/bash -xec "cd $MAPPED_DIR; $1"
docker ps -a
docker stop "$DOCKER_CONTAINER_ID"
docker rm -v "$DOCKER_CONTAINER_ID"
