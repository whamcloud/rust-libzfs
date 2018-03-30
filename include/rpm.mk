include include/rpm-common.mk
include include/copr.mk

VERSION_RELEASE := $(shell repoquery -q --qf "%{version}-%{release}" $(NAME) | sed -e 's/.el7//g')
SRPM            := $(NAME)-$(VERSION_RELEASE).el7.src.rpm

$(SRPM):
	yumdownloader --source $(NAME)

unpack: $(SRPM)
	#if [ -d old ]; then                          \
	#    echo "directory old already exists."     \
	#         "please clean it up and try again"; \
	#    exit 1;                                  \
	#fi
	#mkdir old
	#mv $$(ls | egrep -v -e ^old$$ -e ^Makefile$$) old
	rpm2cpio < $(SRPM) | cpio -iu

download: $(SRPM)

$(RPM_SOURCES):
	if ! spectool $(RPM_DIST_VERSION_ARG)                  \
		   --define "epel 1"                           \
		   -g $(RPM_SPEC); then                        \
	    echo "Failed to fetch $@.";                        \
	    exit 1;                                            \
	fi

install_build_deps:
	echo "Nothing to do"

.PHONY: unpack download install_build_deps
