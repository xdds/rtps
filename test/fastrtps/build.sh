#!/bin/bash
set -ex

#path_to_executable=$(which brew)
#if [ -x "$path_to_executable" ] ; then
#    boost_installed=$(brew ls boost)
#    if [[ $? == 1 ]]; then
#        $(brew update && brew install boost)
#        exit 1
#    fi
#else
#    echo "Am I in $TRAVIS?"
#    # noop, use apt packages in travis
#fi

FAST_RTPS_TAG=v1.3.1
FAST_CDR_TAG=v1.0.6
if [[ ! -d Fast-RTPS ]]; then
    git clone https://github.com/eProsima/Fast-RTPS.git
fi

echo "Pushing to Fast-RTPS"

pushd Fast-RTPS
    if [[ ! $(git branch | grep "* $FAST_RTPS_TAG") ]]; then
        git checkout tags/$FAST_RTPS_TAG -b $FAST_RTPS_TAG
    fi

    cmake . -DEPROSIMA_BUILD=ON -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=../install
    make
    make install
popd

## This would work if the build system was Fast-RTPS didn't rely on git submodules
#if [[ ! -d Fast-CDR ]]; then
#    curl --silent --fail -L -O https://github.com/eProsima/Fast-CDR/archive/$FAST_CDR_TAG.tar.gz
#    tar xzf $FAST_CDR_TAG.tar.gz
#    rm $FAST_CDR_TAG.tar.gz
#    mv Fast-CDR-${FAST_CDR_TAG/v/} Fast-CDR
#fi
#if [[ ! -d Fast-RTPS ]]; then
#    curl --silent --fail -L -O https://github.com/eProsima/Fast-RTPS/archive/$FAST_RTPS_TAG.tar.gz
#    tar xzf $FAST_RTPS_TAG.tar.gz
#    rm $FAST_RTPS_TAG.tar.gz
#    mv Fast-RTPS-${FAST_RTPS_TAG/v/} Fast-RTPS
#fi


