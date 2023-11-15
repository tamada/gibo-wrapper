GO=go
SHELL=/bin/bash
NAME := gibo-wrapper
VERSION := 1.0.0
DIST := $(NAME)-$(VERSION)

all: test build

test:
	$(GO) test -covermode=count -coverprofile=coverage.out $$(go list ./...)

build: test
	$(GO) build -o $(NAME) main.go


# refer from https://pod.hatenablog.com/entry/2017/06/13/150342
define _createDist
	echo -n "create dist/$(DIST)_$(1)_$(2).tar.gz ...."
	mkdir -p dist/$(1)_$(2)/$(DIST)/bin
	GOOS=$1 GOARCH=$2 go build -o dist/$(1)_$(2)/$(DIST)/bin/$(NAME)$(3) main.go
	cp -r completions dist/$(1)_$(2)/$(DIST)
	cp -r README.md LICENSE dist/$(1)_$(2)/$(DIST)
	tar cfz dist/$(DIST)_$(1)_$(2).tar.gz -C dist/$(1)_$(2) $(DIST)
	echo "done."
endef

dist: build
	@$(call _createDist,darwin,amd64,)
	@$(call _createDist,darwin,arm64,)
	@$(call _createDist,windows,amd64,.exe)
	@$(call _createDist,windows,386,.exe)
	@$(call _createDist,linux,amd64,)
	@$(call _createDist,linux,386,)

clean:
	$(GO) clean
	rm -rf gibo-wrapper
