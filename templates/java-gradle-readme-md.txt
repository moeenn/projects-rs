# {{ name }}

## Usage

```bash
# quickly run the project
# use -q flag for quiet
# use -t flag for live reload build 
$ gradle run

# run tests
$ gradle test

# check and compile project classes
$ gradle build

# generate output jar (in ./build/libs/)
$ gradle jar

# execute jar file 
$ java -jar ./build/libs/{{ name }}-0.0.1.jar

# list gradle background daemon details
$ gradle --status

# kill gradle daemons
$ gradle --stop
```