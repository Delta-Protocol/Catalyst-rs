use catalyst_common::constants;
use catalyst_common::std_signature::*;
pub use catalyst_protocol_sdk_rust::prelude::*;
pub use catalyst_protocol_sdk_rust::Cryptography::ErrorCode;

#[cfg(test)]
mod integration_tests {
    use super::*;
    use hex::FromHex;

    #[test]
    fn can_create_signature() {
        let mut sig = [0u8; constants::SIGNATURE_LENGTH];
        let mut public_key = [0u8; constants::PUBLIC_KEY_LENGTH];
        let private_key = [0u8; constants::PRIVATE_KEY_LENGTH];
        let message = b"message";
        let context = b"context";
        let result = sign(&mut sig, &mut public_key, &private_key, message, context);
        assert_eq!(result, ErrorCode::NO_ERROR.value());
    }

    #[test]
    fn can_sign_message_and_verify_signature() {
        let mut sig = [0u8; constants::SIGNATURE_LENGTH];
        let mut public_key = [0u8; constants::PUBLIC_KEY_LENGTH];
        let private_key = [0u8; constants::PRIVATE_KEY_LENGTH];

        let message = b"message";
        let context = b"Context 1 2 3";
        sign(&mut sig, &mut public_key, &private_key, message, context);

        assert_eq!(
            verify(&sig, &public_key, message, context,),
            ErrorCode::NO_ERROR.value()
        );
    }

    #[test]
    fn verification_fails_for_incorrect_context() {
        let mut sig = [0u8; constants::SIGNATURE_LENGTH];
        let mut public_key = [0u8; constants::PUBLIC_KEY_LENGTH];
        let private_key = [0u8; constants::PRIVATE_KEY_LENGTH];

        let message = b"message";
        let context1 = b"Context 1 2 3";
        let context2 = b"Context 1 2 3 4";
        sign(&mut sig, &mut public_key, &private_key, message, context1);
        let verified = verify(&sig, &public_key, message, context2);
        assert_eq!(verified, ErrorCode::SIGNATURE_VERIFICATION_FAILURE.value())
    }

    #[test]
    fn signing_fails_for_context_length_exceeding_maximum() {
        let mut sig = [0u8; constants::SIGNATURE_LENGTH];
        let mut public_key = [0u8; constants::PUBLIC_KEY_LENGTH];
        let private_key = [0u8; constants::PRIVATE_KEY_LENGTH];

        let message = b"message";
        let context = b"2hPB7lVGQHENtQLcfOoTnEjBHO5jg0zgyQYyrKHOxywDrDDfmyg0z9M9Q0hRRfPUV4fWvRGR9l48a3cXmFqBPneErN5GwzD28E3cLhDRNAdaNEpelPRDzN4w2dGaNWc4Jrc7TlVEbC5JQdfMgmtPkakmF3mPCU1YUFQArFUbQFQdFLHL2PByvyzdHaStkSgZbCz0zb9jCBO0vwx4J6YXvXFoc9urYREcR7uiFEVcrf6L2C2uUVOtWQUHRQyIRtmx";
        let result = sign(&mut sig, &mut public_key, &private_key, message, context);

        assert_eq!(result, ErrorCode::INVALID_CONTEXT_LENGTH.value())
    }

    #[test]
    fn verification_fails_for_invalid_public_key() {
        let mut sig = [0u8; constants::SIGNATURE_LENGTH];
        let mut public_key = [0u8; constants::PUBLIC_KEY_LENGTH];
        let private_key = [0u8; constants::PRIVATE_KEY_LENGTH];

        let message = b"message";
        let context = b"Context 1 2 3";
        sign(&mut sig, &mut public_key, &private_key, message, context);
        public_key[constants::PUBLIC_KEY_LENGTH - 1] =
            public_key[constants::PUBLIC_KEY_LENGTH - 1].wrapping_add(1u8);
        let verified = verify(&sig, &public_key, message, context);
        assert_eq!(verified, ErrorCode::INVALID_PUBLIC_KEY.value())
    }

    #[test]
    fn can_validate_using_known_test_vector() {
        let sig = <[u8; constants::SIGNATURE_LENGTH]>::from_hex("98a70222f0b8121aa9d30f813d683f809e462b469c7ff87639499bb94e6dae4131f85042463c2a355a2003d062adf5aaa10b8c61e636062aaad11c2a26083406").unwrap();
        let public_key = <[u8; constants::PRIVATE_KEY_LENGTH]>::from_hex(
            "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf",
        )
        .unwrap();
        let mut message = [0u8; 3];
        hex::decode_to_slice("616263", &mut message as &mut [u8]).unwrap();
        let context = b"";
        assert_eq!(
            verify(&sig, &public_key, &message, context,),
            ErrorCode::NO_ERROR.value()
        );
    }

    #[test]
    fn can_sign_message_and_verify_signature_with_empty_context() {
        let mut sig = [0u8; constants::SIGNATURE_LENGTH];
        let mut public_key = [0u8; constants::PUBLIC_KEY_LENGTH];
        let private_key = [0u8; constants::PRIVATE_KEY_LENGTH];

        let message = b"message";
        let context = b"";
        assert_eq!(
            sign(&mut sig, &mut public_key, &private_key, message, context,),
            ErrorCode::NO_ERROR.value()
        );

        assert_eq!(
            verify(&sig, &public_key, message, context,),
            ErrorCode::NO_ERROR.value()
        );
    }
}
