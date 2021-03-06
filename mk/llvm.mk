# Copyright 2012 The Rust Project Developers. See the COPYRIGHT
# file at the top-level directory of this distribution and at
# http://rust-lang.org/COPYRIGHT.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.


ifdef CFG_ENABLE_FAST_MAKE
LLVM_DEPS := $(S)/.gitmodules
else

# This is just a rough approximation of LLVM deps
LLVM_DEPS=$(call rwildcard,$(CFG_LLVM_SRC_DIR),*cpp *hpp)
endif

define DEF_LLVM_RULES

# If CFG_LLVM_ROOT is defined then we don't build LLVM ourselves
ifeq ($(CFG_LLVM_ROOT),)

$$(LLVM_CONFIG_$(1)): $$(LLVM_DEPS)
	@$$(call E, make: llvm)
	$$(Q)$$(MAKE) -C $$(CFG_LLVM_BUILD_DIR_$(1)) $$(CFG_LLVM_BUILD_ENV)
	$$(Q)touch $$(LLVM_CONFIG_$(1))
endif

endef

$(foreach target,$(CFG_TARGET_TRIPLES), \
 $(eval $(call DEF_LLVM_RULES,$(target))))
