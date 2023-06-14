#	{{{3
#   vim: set tabstop=4 modeline modelines=10:
#   vim: set foldlevel=2 foldcolumn=2 foldmethod=marker:
#	{{{2

#	macOS Rust setup:
#	(Using brew?)
#	Installing rustup (official instructions): 
#	LINK: https://www.rust-lang.org/tools/install
#			curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
#	(Where we trust the curl flags are sufficent to pipe into sh safely)
#	Rust installs to '~/.cargo/bin', which we must add to PATH

#	Message:
#	{{{
#	info: downloading installer
#	
#	Welcome to Rust!
#	
#	This will download and install the official compiler for the Rust
#	programming language, and its package manager, Cargo.
#	
#	Rustup metadata and toolchains will be installed into the Rustup
#	home directory, located at:
#	
#	  /Users/mldavis/.rustup
#	
#	This can be modified with the RUSTUP_HOME environment variable.
#	
#	The Cargo home directory is located at:
#	
#	  /Users/mldavis/.cargo
#	
#	This can be modified with the CARGO_HOME environment variable.
#	
#	The cargo, rustc, rustup and other commands will be added to
#	Cargo's bin directory, located at:
#	
#	  /Users/mldavis/.cargo/bin
#	
#	This path will then be added to your PATH environment variable by
#	modifying the profile files located at:
#	
#	  /Users/mldavis/.profile
#	  /Users/mldavis/.zshenv
#	
#	You can uninstall at any time with rustup self uninstall and
#	these changes will be reverted.
#	
#	Current installation options:
#	
#	
#	   default host triple: aarch64-apple-darwin
#	     default toolchain: stable (default)
#	               profile: default
#	  modify PATH variable: yes
#	
#	1) Proceed with installation (default)
#	2) Customize installation
#	3) Cancel installation
#	}}}

#	Chose '1' default install

#	Rust provides '~/.cargo/env' to add Rust to PATH:
#	{{{
#	# rustup shell setup
#	# affix colons on either side of $PATH to simplify matching
#	case ":${PATH}:" in
#	    *:"$HOME/.cargo/bin":*)
#	        ;;
#	    *)
#	        # Prepending path in case a system-installed rustc needs to be overridden
#	        export PATH="$HOME/.cargo/bin:$PATH"
#	        ;;
#	esac
#	}}}

#	Update rust:
#			rustup update

#	Remove rust:
#			rustup self uninstall

#	Verify commands are available:
#			cargo --version
#			rustc --version
#			rustdoc --version

#	cargo: 		compilation/package manager
#	rustc: 		compiler
#	rustdoc: 	documentation tool

#	Create a hello-world package
#			cargo new --bin helloworld
#	This includes setting up a git repo. Prevent this with '--vcs none'
#	From any directory in the package, build and run with
#			cargo run
#	To remove <generated> files
#			cargo clean

