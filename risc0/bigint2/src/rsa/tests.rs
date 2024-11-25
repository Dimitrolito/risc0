// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(feature = "num-bigint-dig")]
extern crate num_bigint_dig as num_bigint;

use num_bigint::BigUint;
use risc0_bigint2_methods::RSA_ELF;
use risc0_zkvm::{
    get_prover_server, ExecutorEnv, ExecutorImpl, ExitCode, ProverOpts, Session, VerifierContext,
};
use test_log::test;

fn execute_modpow_session(base: &BigUint, modulus: &BigUint) -> anyhow::Result<Session> {
    let env = ExecutorEnv::builder().write(&(base, modulus))?.build()?;
    ExecutorImpl::from_elf(env, RSA_ELF)?.run()
}

#[test]
fn modpow_65537() {
    const BASE: &[u8] = b"00024f2f68423c422bb8b7b6ea3f3382a379e635f9501bd9e1f09922407b5c4326fb959eadd237ed80972767deeb416749b137fc5c80876126f154f2c129a04c05970e08396dbd5d1285b4fb3a63b80855101bc3bbc9d90dd0500efd79989267ab36d4e2a361761c6ea1e172c741ec59b0aee1f1956b3ba947ed0b0cc2d45b47c041ee2a47bcc53345bd4714831b5d3125ad9a940f8efcc30ae51fc37ee6e3e4e2bd5f43ff762d7dc23017cd67a56ed00f30f09661839a10c2a487ebc1314c5bf77f0f305d040649741f357b6bb478bd72864845c3b62691c6f80199c923d2efd2e9f75751e26dc26495d3ff848d53c0424b5451d644a9903fb8c93bab50227e59c4e05029c1c50d434bd368dc0880910548804093bdd59da572d5513e57c078354f73aa35ba72d39e22e351449849d158a0ec8bc0069cd09f3dbcb7f02c089a054c35dbf7469089a43c679c3eb7f972c6cd569dcc8bf2540b1188083b77c8517a635ea037ec1b46c0f1cec0770c8d25959f80ae0068656c6c6f20776f726c64";
    const MODULUS: &[u8] = b"a79633c36a2e9a91cac9505c300edb9a0aa8105f115856024fe27dac4910ab2fad99fe3700ddf532a3c86c5171d28177d55b3112fe73f46917a58006ac747ae396557fe84e62ba0cefd55858258e2aecd756a855ca7bd81be968fc3b908a310508828275065ab91b6f8f7964ef5684ee7fd5188c097a99323dfbe1302ab8723d97f9d1a30b89b9fb7504e8fe1e85f858ae2f07818ff8c89f7f9dbd051e52f5dae0eb33ab85d826906817f6712003c866f878874c503ec4ee02457a3c7012c6c77f66bfe23518d2eddbbe9fc3dbb8ad944d9a38e333c7caa86138f5121b89563067c4bac9c32a26cd7c21830315f8e1c5b3783104f24648f7bf8556500b04d8bf11bc7e777aa5e0653c5af9b085c2c08a16f7d2b4bc323d1b32a5cc8f4ab39f374d30efa01f8009aeb60358e5f22345fea8a123e46f394bcc669d183348b6eb29fa5f972ebe90aa64684973f4e67c0ea7d82c5a05c4a837a0a63c86b201c54cbcb1dbabf26a5bbd533b0ec91892716b7deb54c061d2e8696391f210e16b4714dd";
    const EXPECTED: &[u8] = b"6bc245902f8b3c05089a85c2e17e8d00da0b61ac57990bd1fca990ebcb5cf7c1bfbf9adcf72c95ffba1034b62da58361290779dbf92f1e789621dd4cf8926552b5887aff88eeae89eb3dbb7e0bc7d6dd3f93db38ea3f79c0e1ca2fa866a40cad41055e7451c1d180639ecdf69e4e431efa658424c3cb242fd1acf75ce84665a857b50e5e8e5049b6159d926ed193b14f58ef8fd9aab33f308f9a1753daeb4bca467c057822a01a3e3901e2e064a5d8f336ea1ad665055f608f5e0e864ddf9c0c3723a505d649e3be66b10ee525c442e156ec61b288e7913c015804c89d9f983504a0dcf88d5399af4b53f30761484cdd49ef4eff3b6942d10bdbeedbca7b91f1cd20d031d51bf9b34342ad1809d0c4752cfb12ca7a77748c791ede7c06ef89c1017c2a9f81dd270cd504b57c292712e982efc4acd967a260b7c5cf3432a15b05f1da140a7de981338a4aa06b3858a936ef669567f4b9652fc43e806d5575b8bcf2d2a2aef6c33ffa6eeea67b8369d78d6fd952678189bd51d838011111bfa856";

    let base = BigUint::parse_bytes(BASE, 16).unwrap();
    let modulus = BigUint::parse_bytes(MODULUS, 16).unwrap();
    let expected = BigUint::parse_bytes(EXPECTED, 16).unwrap();

    let session = execute_modpow_session(&base, &modulus).unwrap();
    assert_eq!(session.exit_code, ExitCode::Halted(0));
    let result: BigUint = session.journal.as_ref().unwrap().decode().unwrap();
    assert_eq!(result, expected);

    let prover = get_prover_server(&ProverOpts::fast()).unwrap();
    prover
        .prove_session(&VerifierContext::default(), &session)
        .unwrap();
}

#[test]
fn modpow_65537_small_base() {
    const MODULUS: &[u8] = b"a79633c36a2e9a91cac9505c300edb9a0aa8105f115856024fe27dac4910ab2fad99fe3700ddf532a3c86c5171d28177d55b3112fe73f46917a58006ac747ae396557fe84e62ba0cefd55858258e2aecd756a855ca7bd81be968fc3b908a310508828275065ab91b6f8f7964ef5684ee7fd5188c097a99323dfbe1302ab8723d97f9d1a30b89b9fb7504e8fe1e85f858ae2f07818ff8c89f7f9dbd051e52f5dae0eb33ab85d826906817f6712003c866f878874c503ec4ee02457a3c7012c6c77f66bfe23518d2eddbbe9fc3dbb8ad944d9a38e333c7caa86138f5121b89563067c4bac9c32a26cd7c21830315f8e1c5b3783104f24648f7bf8556500b04d8bf11bc7e777aa5e0653c5af9b085c2c08a16f7d2b4bc323d1b32a5cc8f4ab39f374d30efa01f8009aeb60358e5f22345fea8a123e46f394bcc669d183348b6eb29fa5f972ebe90aa64684973f4e67c0ea7d82c5a05c4a837a0a63c86b201c54cbcb1dbabf26a5bbd533b0ec91892716b7deb54c061d2e8696391f210e16b4714dd";

    let base = BigUint::from(1u32);
    let modulus = BigUint::parse_bytes(MODULUS, 16).unwrap();
    let expected = BigUint::from(1u32);

    let session = execute_modpow_session(&base, &modulus).unwrap();
    assert_eq!(session.exit_code, ExitCode::Halted(0));
    let result: BigUint = session.journal.as_ref().unwrap().decode().unwrap();
    assert_eq!(result, expected);

    let prover = get_prover_server(&ProverOpts::fast()).unwrap();
    prover
        .prove_session(&VerifierContext::default(), &session)
        .unwrap();
}

#[test]
fn modpow_65537_small_mod() {
    const BASE: &[u8] = b"00024f2f68423c422bb8b7b6ea3f3382a379e635f9501bd9e1f09922407b5c4326fb959eadd237ed80972767deeb416749b137fc5c80876126f154f2c129a04c05970e08396dbd5d1285b4fb3a63b80855101bc3bbc9d90dd0500efd79989267ab36d4e2a361761c6ea1e172c741ec59b0aee1f1956b3ba947ed0b0cc2d45b47c041ee2a47bcc53345bd4714831b5d3125ad9a940f8efcc30ae51fc37ee6e3e4e2bd5f43ff762d7dc23017cd67a56ed00f30f09661839a10c2a487ebc1314c5bf77f0f305d040649741f357b6bb478bd72864845c3b62691c6f80199c923d2efd2e9f75751e26dc26495d3ff848d53c0424b5451d644a9903fb8c93bab50227e59c4e05029c1c50d434bd368dc0880910548804093bdd59da572d5513e57c078354f73aa35ba72d39e22e351449849d158a0ec8bc0069cd09f3dbcb7f02c089a054c35dbf7469089a43c679c3eb7f972c6cd569dcc8bf2540b1188083b77c8517a635ea037ec1b46c0f1cec0770c8d25959f80ae0068656c6c6f20776f726c64";

    let base = BigUint::parse_bytes(BASE, 16).unwrap();
    let modulus = BigUint::from(2147483647u32);
    let expected = BigUint::from(1205382585u32);

    let session = execute_modpow_session(&base, &modulus).unwrap();
    assert_eq!(session.exit_code, ExitCode::Halted(0));
    let result: BigUint = session.journal.as_ref().unwrap().decode().unwrap();
    assert_eq!(result, expected);

    let prover = get_prover_server(&ProverOpts::fast()).unwrap();
    prover
        .prove_session(&VerifierContext::default(), &session)
        .unwrap();
}

#[test]
fn modpow_65537_large_base_panics() {
    const BASE: &[u8] = b"88888888e9e6b995feb7a6082d5a50f97744c063816ca28ac93d16d7edd9019c9dc943cfe285aca7f174a2b60d8f461d0bcf26274530df09f2bf9497ebe04628a1263a900cddb2f54c8e9502490e79facc585a614626ad5cc7007df8bbdaefa37e891258ff556cad2b4bdbff6c4bdac0deec4b8bd390830a2285be2e708e38b41ad5499ae8bcc8a9c357fe91c18cdaf831b3b6baefda7181c5a89cfc0c7b10d1a2cd8e5f01b2b1a6e52774e049f1db8b7348cd85122128c165ea0d00c571b0619c40f9ddad422f38df6540b9a2d196b978fd2908b4719c88338370060b9d40026e966dbd01e746473a80af66ad4dc859839c6ce84ee6faf006b37bca5cc5e629eff9cd0c8ed38ce21ac137f78dfd4a18881d635791a87a8a649d65c81772f1a0a9a538b1c6a7f5b62e38dc99988fdf02ff6458e1fc3ba264e761eeeacc1d600e7a13aedf5b0df74e7e466e5a18c21024f297483c5e41bdc2cb237b95353711c469106e67c65c5df821b54f71c2190ed1aa2d0c532347293b31a03cc84ddac62c0b6e7234dca9f8bfe920364d7a6b64ff19de7acc25a733404a317ed43962b4ec6a08c1e3867477e494adc2711cba603d4b238c135679bdaa6e8e2a0b737f827e90712cc041b42f5760c88460471c459a16ae7e30beecdea25695ed06520339d40f94517043bada42dc27ae75154cd09239b75def8f80c512bc0866994c1487ed2cc7c0d7";

    let base = BigUint::parse_bytes(BASE, 16).unwrap();
    let modulus = BigUint::from(2u32);

    let err = execute_modpow_session(&base, &modulus).err().unwrap();
    assert!(err.to_string().contains("Input too large"));
}

#[test]
fn modpow_65537_large_mod_panics() {
    const MODULUS: &[u8] = b"88888888e9e6b995feb7a6082d5a50f97744c063816ca28ac93d16d7edd9019c9dc943cfe285aca7f174a2b60d8f461d0bcf26274530df09f2bf9497ebe04628a1263a900cddb2f54c8e9502490e79facc585a614626ad5cc7007df8bbdaefa37e891258ff556cad2b4bdbff6c4bdac0deec4b8bd390830a2285be2e708e38b41ad5499ae8bcc8a9c357fe91c18cdaf831b3b6baefda7181c5a89cfc0c7b10d1a2cd8e5f01b2b1a6e52774e049f1db8b7348cd85122128c165ea0d00c571b0619c40f9ddad422f38df6540b9a2d196b978fd2908b4719c88338370060b9d40026e966dbd01e746473a80af66ad4dc859839c6ce84ee6faf006b37bca5cc5e629eff9cd0c8ed38ce21ac137f78dfd4a18881d635791a87a8a649d65c81772f1a0a9a538b1c6a7f5b62e38dc99988fdf02ff6458e1fc3ba264e761eeeacc1d600e7a13aedf5b0df74e7e466e5a18c21024f297483c5e41bdc2cb237b95353711c469106e67c65c5df821b54f71c2190ed1aa2d0c532347293b31a03cc84ddac62c0b6e7234dca9f8bfe920364d7a6b64ff19de7acc25a733404a317ed43962b4ec6a08c1e3867477e494adc2711cba603d4b238c135679bdaa6e8e2a0b737f827e90712cc041b42f5760c88460471c459a16ae7e30beecdea25695ed06520339d40f94517043bada42dc27ae75154cd09239b75def8f80c512bc0866994c1487ed2cc7c0d7";

    let base = BigUint::from(1u32);
    let modulus = BigUint::parse_bytes(MODULUS, 16).unwrap();

    let err = execute_modpow_session(&base, &modulus).err().unwrap();
    assert!(err.to_string().contains("Input too large"));
}
