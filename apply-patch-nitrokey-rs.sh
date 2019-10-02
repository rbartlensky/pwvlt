#!/bin/sh

git clone https://git.ireas.org/nitrokey-rs/ && cd nitrokey-rs/ && \
    git apply ../nitrokey-rs.patch
