// Copyright (C) 2017-2018 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

extern crate sgx_tunittest;

extern crate ring;
extern crate untrusted;

use ring::aead;
use sgx_types::*;
use std::string::String;
use std::vec::Vec;
use std::io::{self, Write};
use std::{slice, panic};
use sgx_tunittest::*;

mod aead_tests;
use aead_tests::*;

mod agreement_tests;
use agreement_tests::*;

mod digest_tests;
use digest_tests::*;

mod ecdsa_tests;
use ecdsa_tests::*;

mod ed25519_tests;
use ed25519_tests::*;

mod hkdf_tests;
use hkdf_tests::*;

mod hmac_tests;
use hmac_tests::*;

mod pbkdf2_tests;
use pbkdf2_tests::*;

mod rsa_tests;
use rsa_tests::*;

mod signature_tests;
use signature_tests::*;

#[no_mangle]
pub extern "C" fn say_something(some_string: *const u8, some_len: usize) -> sgx_status_t {

    let str_slice = unsafe { slice::from_raw_parts(some_string, some_len) };
    let _ = io::stdout().write(str_slice);

    println!("aes: {:?}", &aead::AES_256_GCM);
    // A sample &'static string
    let rust_raw_string = "This is a in-Enclave ";
    // An array
    let word:[u8;4] = [82, 117, 115, 116];
    // An vector
    let word_vec:Vec<u8> = vec![32, 115, 116, 114, 105, 110, 103, 33];

    // Construct a string from &'static string
    let mut hello_string = String::from(rust_raw_string);

    // Iterate on word array
    for c in word.iter() {
        hello_string.push(*c as char);
    }

    // Rust style convertion
    hello_string += String::from_utf8(word_vec).expect("Invalid UTF-8")
                                               .as_str();

    // Ocall to normal world for output
    println!("{}", &hello_string);

    rsgx_unit_tests!(
        // aead_tests.rs
        aead_aes_gcm_128,
        aead_aes_gcm_256,
        aead_chacha20_poly1305,
        // agreement_test.rs
        agreement_agree_ephemeral,
        test_agreement_ecdh_x25519_rfc_iterated,
        // digest_test.rs
        digest_misc,
        test_fmt_algorithm,
        digest_test_fmt,
        digest_shavs::SHA1::short_msg_known_answer_test,
        digest_shavs::SHA1::long_msg_known_answer_test,
        digest_shavs::SHA1::monte_carlo_test,
        digest_shavs::SHA256::short_msg_known_answer_test,
        digest_shavs::SHA256::long_msg_known_answer_test,
        digest_shavs::SHA256::monte_carlo_test,
        digest_shavs::SHA384::short_msg_known_answer_test,
        digest_shavs::SHA384::long_msg_known_answer_test,
        digest_shavs::SHA384::monte_carlo_test,
        digest_shavs::SHA512::short_msg_known_answer_test,
        digest_shavs::SHA512::long_msg_known_answer_test,
        digest_tests::digest_shavs::SHA512::monte_carlo_test,
        ecdsa_from_pkcs8_test,
        ecdsa_generate_pkcs8_test,
        signature_ecdsa_verify_asn1_test,
        signature_ecdsa_verify_fixed_test,
        test_signature_ed25519,
        test_ed25519_from_seed_and_public_key_misuse,
        test_ed25519_from_pkcs8_unchecked,
        test_ed25519_from_pkcs8,
        hkdf_tests,
        hmac_tests,
        pbkdf2_tests,
        pbkdf2_one_iteration,
        rsa_from_pkcs8_test,
        test_signature_rsa_pkcs1_sign,
        test_signature_rsa_pss_sign,
        test_rsa_key_pair_traits,
        test_signature_rsa_pkcs1_verify,
        test_signature_rsa_pss_verify,
        test_signature_rsa_primitive_verification,
        signature_impl_test
    );

    println!("Testing should_panic cases...");
    println!("Testing pkdf2_zero_iterations");
    should_panic!(pbkdf2_zero_iterations());
    println!("should_panic test finished!");

    println!("All tests finished!");
    sgx_status_t::SGX_SUCCESS
}
