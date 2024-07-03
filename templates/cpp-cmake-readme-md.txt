## {{ name }}

### Setup

```bash
# generate makefile
$ cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=1 .

# compile program
$ make

# execute program
$ ./bin/sandbox

# cleanup build files
$ make clean