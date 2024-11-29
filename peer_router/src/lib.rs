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

pub struct LinkHandle(usize);

enum LinkState {
    Authenticating(VerifyingKey, SigningKey),
    Sharing(),
    Up(Aes128Gcm),
    Dead,
}

pub struct Link<'a> {
    state: &'a mut LinkState,
    out_packet: &'a mut DataPacket<'a>,
    in_packet: &'a mut DataPacket<'a>,
}

impl<'a> Link<'a> {
    pub fn push_data(&mut self, in_buf: &[u8]) {
        todo!()
    }
    pub fn pull_data(&mut self, out_buf: &mut [u8]) {
        match self.state {
            LinkState::Authenticating(verify, sign) => {
                let public = sign.verifying_key();
                let res = sign.sign(public.as_bytes());
                *self.state = LinkState::Dead;
            }
            LinkState::Sharing() => todo!(),
            LinkState::Up(_) => todo!(),
            LinkState::Dead => todo!(),
        }
        todo!()
    }
}

struct Route {
    to: u64,
    via: u8,
    hops: u8,
}

impl Route {
    fn new(dst: u64, via: u8, hops: u8) -> Self {
        Self { to: dst, via, hops }
    }
}

pub struct Router<'a, const MAX_PACKET: usize> {
    cert: SigningKey,
    ca: VerifyingKey,
    id: u64,
    route_table: [Option<Route>; 128],
    link_state: [Option<LinkState>; 32],
    packet_buffers: &mut [[u8; MAX_PACKET]],
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

impl<'a> Router<'a> {
    pub fn new(
        id: u64,
        key: &[u8; 64],
        sig: &[u8; 64],
        ca: &[u8; 32],
        buffers: &mut [u8],
    ) -> Result<Self, RouterError> {
        let cert = SigningKey::from_keypair_bytes(key)?;
        let ca = VerifyingKey::from_bytes(ca)?;
        let sig = Signature::from_bytes(sig);
        let (_, packet_buffers, _) = unsafe { buffers.align_to_mut::<[u8; 1000]>() };
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
        for i in 0..self.inbound_packets.len() {
            let Some(inbound) = &mut self.inbound_packets[i] else {
                continue;
            };
            if inbound.read == true {
                continue;
            }

            if inbound.dst == self.id {
                todo!("handle arrived packets");
            }
            let Some(outbound) = &mut self.outbound_packets[0] else {
                continue;
            };

            let routes = &mut self.route_table;
            let dst = inbound.dst;
            let src = inbound.src;
            routes
                .iter_mut()
                .filter_map(|i| i.as_mut())
                .filter(|i| i.to == dst);
        }
    }

    pub fn create_link(&mut self, buffer: &'a mut [u8]) -> Result<LinkHandle, CreateLinkError> {
        let (index, _) = self
            .inbound_packets
            .iter_mut()
            .enumerate()
            .find(|(_, i)| i.is_none())
            .ok_or(CreateLinkError)?;
        let (inbound_buffer, outbound_buffer) = buffer.split_at_mut(buffer.len() / 2);
        self.inbound_packets[index] = Some(DataPacket::new(inbound_buffer));
        self.outbound_packets[index] = Some(DataPacket::new(outbound_buffer));
        Ok(LinkHandle(index))
    }

    pub fn bind_app(&mut self, app_id: u64) {
        todo!()
    }

    pub fn get_link(&'a mut self, &LinkHandle(handle): &LinkHandle) -> Option<Link> {
        let in_packet = self.inbound_packets[handle].as_mut()?;
        let out_packet = self.outbound_packets[handle].as_mut()?;
        let state = self.link_state[handle].as_mut()?;
        Some(Link {
            in_packet,
            out_packet,
            state,
        })
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
