# ring-rust-sgx tests

This project is used for testing ring with Baidu's rust-sgx-sdk. Currently the ring here is outdated and I'll get it upgraded later.

## What's included

* A [Dockerfile](docker/Dockerfile)
* Ported testcases under folder [enclave/src](enclave/src)
* My ported [ring](ring)

Now I merged all ring's test cases to one enclave. To finish this, I removed all `#[test]` and make all test cases `pub` functions. Then I use `rsgx_unit_tests!` to test everything.

Currently the only panic test is `pbkdf2_zero_iterations` and I use `should_panic!` to test it.

Currently `xargo` is not supported.

## Use without SGX hardware support (simulation mode)

Build the docker image from Dockerfile or just pull the pre-built one from `baiduxlab/ring-sgx-test`:

```
$ docker pull baiduxlab/ring-sgx-test
```

Then start a container, then build and test:

```
$ docker run -ti --rm -v /path/to/ring-sgx-test:/root/ring-sgx-test baiduxlab/ring-sgx-test bash
# cd /root/ring-sgx-test
# SGX_MODE=SW make
# cd bin
# ./app
```

## Use with SGX hardware support (hardware mode)

Launch a container with SGX hardware:

```
$ docker run -ti --rm --device /dev/isgx -v /path/to/ring-sgx-test:/root/ring-sgx-test baiduxlab/ring-sgx-test bash
```

Launch `aesmd` before testing:

```
~ # /opt/intel/libsgx-enclave-common/aesm/aesm_service
aesm_service[487]: The server sock is 0x5599b2ce5960
aesm_service[487]: [ADMIN]White List update requested
aesm_service[487]: [ADMIN]Platform Services initializing
aesm_service[487]: [ADMIN]Platform Services initialization failed due to DAL error
aesm_service[487]: [ADMIN]White list update request successful for Version: 48
```

Build in hardware mode and run:

```
# cd /root/ring-sgx-test
# make
# cd bin
# ./app
```

