# Taken from the lambda class cairo_native repo
CAIRO_2_VERSION=2.3.1

setup:
	curl -L -o "cairo.tar" "https://github.com/starkware-libs/cairo/releases/download/v$(CAIRO_2_VERSION)/release-aarch64-apple-darwin.tar"
	rm -rf corelib \
	&& tar -xzvf cairo.tar \
	&& mv cairo/corelib . \
	&& rm -rf cairo \
	&& rm cairo.tar

llvm:
	-brew install llvm@17 --quiet
	@echo "You can execute the env-macos.sh script to setup the needed env variables."
