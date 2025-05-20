#![no_std]

extern crate core;

use aes_gcm::Aes128Gcm;

use ed25519_dalek::{
    ed25519::signature::SignerMut, Signature, SignatureError, SigningKey, VerifyingKey,
};

use x25519_dalek::PublicKey;

enum LinkFrame<'a> {
    Packet(DataPacket<'a>),
    Authentication(AuthenticationPacket<'a>),
}

pub struct DataPacket<'a> {
    read: bool,
    src: u64,
    dst: u64,
    hops: u8,
    data: &'a mut [u8],
}

impl<'a> DataPacket<'a> {
    fn new(buffer: &'a mut [u8]) -> Self {
        Self {
            read: false,
            src: 0,
            dst: 0,
            hops: 0,
            data: buffer,
        }
    }
}

pub struct AuthenticationPacket<'a> {
    pub_key: &'a VerifyingKey,
    sig: &'a Signature,
    dh_key: &'a PublicKey,
    dh_sig: &'a Signature,
}

pub struct InterfaceHandle(usize);

enum InterfaceState {
    Authenticating(VerifyingKey, SigningKey),
    Sharing(),
    Up(Aes128Gcm),
    Dead,
}

pub struct Interface<'a, MAX_PACKET> {
    router: &'a mut Router<MAX_PACKET>,
    state: &'a mut InterfaceState,
    out_packet: &'a mut DataPacket<'a>,
    in_packet: &'a mut DataPacket<'a>,
}

impl<'a> Interface<'a> {
    pub fn push_data(&mut self, in_buf: &[u8]) {
        todo!()
    }
    pub fn pull_data(&mut self, out_buf: &mut [u8]) {
        match self.state {
            InterfaceState::Authenticating(verify, sign) => {
                let public = sign.verifying_key();
                let res = sign.sign(public.as_bytes());
                *self.state = InterfaceState::Dead;
            }
            InterfaceState::Sharing() => todo!(),
            InterfaceState::Up(_) => todo!(),
            InterfaceState::Dead => todo!(),
        }
        todo!()
    }
}

struct Route {
    to: u64,
    via: u16,
    hops: u8,
}

impl Route {
    fn new(to: u64, via: u8, hops: u8) -> Self {
        Self { to, via, hops }
    }
}

pub struct Router<'a, const MAX_PACKET: usize> {
    cert: SigningKey,
    ca: VerifyingKey,
    id: u64,
    route_table: [Option<Route>; 128],
    link_state: [Option<InterfaceState>; 32],
    packet_buffers: &'a mut [[u8; MAX_PACKET]],
}

pub enum RouterError {
    SignatureError(SignatureError),
}

impl From<SignatureError> for RouterError {
    fn from(value: SignatureError) -> Self {
        RouterError::SignatureError(value)
    }
}

struct CreateLinkError;

impl<'a, const MAX_PACKET: usize> Router<'a, MAX_PACKET> {
    pub fn new(
        id: u64,
        key: &[u8; 64],
        sig: &[u8; 64],
        ca: &[u8; 32],
        buffers: &'a mut [u8],
    ) -> Result<Self, RouterError> {
        let cert = SigningKey::from_keypair_bytes(key)?;
        let ca = VerifyingKey::from_bytes(ca)?;
        let sig = Signature::from_bytes(sig);
        let (_, packet_buffers, _) = unsafe { buffers.align_to_mut::<[u8; MAX_PACKET]>() };
        Result::Ok(Self {
            id,
            cert,
            ca,
            packet_buffers,
            link_state: [const { Option::None }; 32],
            route_table: [const { Option::None }; 128],
        })
    }

    pub fn process(&mut self) {
        todo!()
    }

    pub fn create_interface(
        &mut self,
        buffer: &'a mut [u8],
    ) -> Result<InterfaceHandle, CreateLinkError> {
        todo!()
    }

    pub fn destroy_interface(link: InterfaceHandle) {
        todo!()
    }
    pub fn get_link(&'a mut self, &InterfaceHandle(handle): &InterfaceHandle) -> Option<Interface> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use aes_gcm::{
        aead::{consts::U12, AeadInPlace},
        Aes256Gcm, Key, KeyInit, Nonce,
    };

    use crate::Router;

    //#[test]
    //fn foo() {
    //    let mut buffer = [0u8; 1024];
    //    let key = [0u8; 64];
    //    let Ok(mut router) = Router::new(1337, &key, &key[..32].try_into().unwrap()) else {
    //        return;
    //    };
    //    let Ok(link_token) = router.create_link(&mut buffer) else {
    //        return;
    //    };
    //    let Some(link) = router.get_link(&link_token) else {
    //        return;
    //    };
    //    let key: &Key<Aes256Gcm> = &[0; 32].into();
    //    let nonce: Nonce<U12> = [0u8; 12].into();
    //    let mut cipher = Aes256Gcm::new(key);
    //    let mut inplace = [0u8; 100];
    //    let tag = cipher
    //        .encrypt_in_place_detached(&nonce, &[0], &mut inplace)
    //        .unwrap();
    //    let foo = cipher
    //        .decrypt_in_place_detached(&nonce, &[0], &mut inplace, &tag)
    //        .unwrap();
    //}
}
