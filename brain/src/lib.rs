#![feature(pointer_byte_offsets)]
#![feature(const_for)]
use anyhow::{format_err, Result};
use nix::{
    fcntl::{fallocate, FallocateFlags},
    sys::mman::{mmap, MapFlags, ProtFlags},
    unistd::{sysconf, SysconfVar},
};
use rand::{distributions::Alphanumeric, prelude::*, thread_rng};
use std::{
    fs::OpenOptions, mem::size_of, num::NonZeroUsize, os::fd::IntoRawFd, path::PathBuf, slice,
    sync::Mutex,
};
mod neuron;

pub struct SharedMemory {
    fd: Mutex<i32>,
    page_size: usize,
}

impl SharedMemory {
    pub fn new(file: Option<&str>) -> Result<(SharedMemory, PathBuf)> {
        let page_size =
            sysconf(SysconfVar::PAGE_SIZE)?.ok_or(format_err!("no page size"))? as usize;
        let r = thread_rng();
        let path_str = format!(
            "/tmp/{}",
            if let Some(file) = file {
                file.to_string()
            } else {
                r.sample_iter(Alphanumeric)
                    .take(10)
                    .map(char::from)
                    .collect::<String>()
            }
        );
        println!("path: {}", path_str);
        let path = PathBuf::from(&path_str);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
        let fd = file.into_raw_fd();
        Ok((
            SharedMemory {
                fd: Mutex::new(fd),
                page_size,
            },
            path,
        ))
    }

    pub fn get_slice<T>(&self, start: usize, n: NonZeroUsize) -> Result<&mut [T]> {
        let size = n
            .checked_mul(NonZeroUsize::new(size_of::<T>()).ok_or(format_err!("cant be zero"))?)
            .ok_or(format_err!("overflow"))?;

        let page_boundry = (start / self.page_size) * self.page_size;
        let offset = start - page_boundry;

        let map_size = if (size.get() % self.page_size) + offset > self.page_size {
            size.checked_add(self.page_size)
                .ok_or(format_err!("overflow"))?
        } else {
            size
        };
        fallocate(
            *self.fd.lock().unwrap(),
            FallocateFlags::empty(),
            page_boundry.try_into()?,
            map_size.get().try_into()?,
        )?;
        unsafe {
            let map = mmap(
                None,
                map_size,
                ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
                MapFlags::MAP_SHARED | MapFlags::MAP_POPULATE,
                *self.fd.lock().unwrap(),
                page_boundry.try_into()?,
            )? as *mut T;
            println!("map: {:#?},{:#?}", map, map.byte_add(map_size.get()));
            Ok(slice::from_raw_parts_mut(map.byte_add(offset), size.into()))
        }
    }
}

impl Drop for SharedMemory {
    fn drop(&mut self) {
        unsafe {
            nix::libc::close(*self.fd.lock().unwrap());
        }
    }
}
