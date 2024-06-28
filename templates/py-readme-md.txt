## {{ name }}

### Setup

```bash
# create a new virutual environment (if not already created)
$ python -m venv .venv

# activate the environment
$ source .venv/bin/activate

# install project dependencies
$ pip install .
```


### Usage

```bash
# start the project
$ invoke start

# run tests
$ invoke test

# format and lint code
$ invoke fmt 
$ invoke check

# compile to stand-alone distributable executable
$ invoke build

# perform temporary files cleanup
$ invoke clean
```
