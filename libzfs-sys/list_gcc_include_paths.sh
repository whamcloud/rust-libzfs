#!/bin/sh

gcc -Wp,-v -x c - -fsyntax-only < /dev/null 2>&1 | grep '^ ' | sed -e 's/^ //'
