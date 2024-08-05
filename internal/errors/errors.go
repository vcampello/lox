package errors

// Based on sysexits.h
type ExitCode = int

const (
	// The command was used incorrectly, e.g., with the
	// wrong number of arguments, a bad flag, a bad syntax
	// in a parameter, or whatever.
	EX_USAGE ExitCode = 64

	// The input data was incorrect in some way.  This
	// should only be used for user's data and not system
	// files.
	EX_DATAERR = 65

	// An error occurred while doing I/O on some file.
	EX_IOERR = 74
)
