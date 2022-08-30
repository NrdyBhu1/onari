# with import <nixpkgs> {}; rec {
# 	mygame = stdenv.mkDerivation {
# 		name = "mygame";
# 		nativeBuildInputs = with pkgs; [
# 			llvm
# 			llvmPackages.libclang
# 			rustup
# 		];
# 		LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib";
# 		# BINDGEN_EXTRA_CLANG_ARGS = "-isystem ${llvmPackages.libclang.lib}/lib/clang/${lib.getVersion clang}/include -isystem /nix/store/v6bxmy5i2w6bznh8avfcf41grzncwv1j-glibc-2.34-210-dev/include/";
# 		BINDGEN_EXTRA_CLANG_ARGS = "-isystem ${llvmPackages.libclang.lib}/lib/clang/${lib.getVersion clang}/include -isystem /nix/store/a0s1vq35j3g6aa757v7yg0a001l01ng4-glibc-2.34-210-dev/include/";
# 		# a0s1vq35j3g6aa757v7yg0a001l01ng4-glibc-2.34-210-dev
# 	};
# }

{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
        llvm
        glib
        llvmPackages.libclang
        rustup
    ];

    LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib";
    BINDGEN_EXTRA_CLANG_ARGS = with pkgs; "-isystem ${llvmPackages.libclang.lib}/lib/clang/${lib.getVersion clang}/include -isystem /nix/store/a0s1vq35j3g6aa757v7yg0a001l01ng4-glibc-2.34-210-dev/include/";
}