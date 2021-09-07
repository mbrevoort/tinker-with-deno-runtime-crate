package main

/*
#cgo LDFLAGS: -L./lib -ltheno
#include "./lib/libtheno.h"
*/
import "C"
import "os"

func main() {
	path := os.Args[1]
	C.run(C.CString(path))
}
