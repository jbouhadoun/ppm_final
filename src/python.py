from ctypes import cdll

lib = cdll.LoadLibrary("../target/debug/libsrc.so")

print(lib.dummy())
