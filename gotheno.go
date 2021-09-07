package main

/*
#cgo LDFLAGS: ./lib/libtheno.a -ldl -framework CoreServices 
#include "./lib/libtheno.h"
*/
import "C"
import "os"

func main() {
	path := os.Args[1]
	C.run(C.CString(path))
}
