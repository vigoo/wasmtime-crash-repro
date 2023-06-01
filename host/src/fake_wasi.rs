use std::time::Duration;
use async_trait::async_trait;

pub struct WasiCtx {}

wasmtime::component::bindgen!({
    path: "preview2_wit",
    interfaces: "
      import wall-clock: clocks.wall-clock
      import monotonic-clock: clocks.monotonic-clock
      import filesystem: filesystem.filesystem
      import poll: poll.poll
      import streams: io.streams
      import environment: wasi-base.environment
      import preopens: wasi-base.preopens
      import exit: wasi-base.exit
    ",
    tracing: true,
    async: true,
    trappable_error_type: {
        "filesystem"::"error-code": Error,
        "streams"::"stream-error": Error,
    }
});

#[async_trait]
impl poll::Host for WasiCtx {
    async fn drop_pollable(&mut self, this: poll::Pollable) -> anyhow::Result<()> {
        Ok(())
    }

    async fn poll_oneoff(
        &mut self,
        futures: Vec<poll::Pollable>,
    ) -> anyhow::Result<Vec<u8>> {
        println!("sleeping start");
        tokio::time::sleep(Duration::from_secs(2)).await;
        println!("sleeping done");
        Ok(vec!(1u8))
    }
}

#[async_trait]
impl wall_clock::Host for WasiCtx {
    async fn now(&mut self) -> anyhow::Result<wall_clock::Datetime> {
        Ok(wall_clock::Datetime {
            seconds: 0,
            nanoseconds: 0,
        })
    }

    async fn resolution(&mut self) -> anyhow::Result<wall_clock::Datetime> {
        Ok(wall_clock::Datetime {
            seconds: 0,
            nanoseconds: 1,
        })
    }
}

#[async_trait]
impl monotonic_clock::Host for WasiCtx {
    async fn now(&mut self) -> anyhow::Result<monotonic_clock::Instant> {
        Ok(0)
    }

    async fn resolution(&mut self) -> anyhow::Result<monotonic_clock::Instant> {
        Ok(1)
    }

    async fn subscribe(
        &mut self,
        when: monotonic_clock::Instant,
        absolute: bool,
    ) -> anyhow::Result<poll::Pollable> {
        Ok(0)
    }
}

#[async_trait]
impl filesystem::Host for WasiCtx {
    async fn read_via_stream(
        &mut self,
        this: filesystem::Descriptor,
        offset: filesystem::Filesize,
    ) -> anyhow::Result<filesystem::InputStream> {
        todo!()
    }

    async fn write_via_stream(
        &mut self,
        this: filesystem::Descriptor,
        offset: filesystem::Filesize,
    ) -> anyhow::Result<filesystem::OutputStream> {
        todo!()
    }

    async fn append_via_stream(
        &mut self,
        this: filesystem::Descriptor,
    ) -> anyhow::Result<filesystem::OutputStream> {
        todo!()
    }

    async fn advise(
        &mut self,
        _this: filesystem::Descriptor,
        _offset: filesystem::Filesize,
        _length: filesystem::Filesize,
        _advice: filesystem::Advice,
    ) -> Result<(), filesystem::Error> {
        todo!()
    }

    async fn sync_data(
        &mut self,
        _this: filesystem::Descriptor,
    ) -> Result<(), filesystem::Error> {
        todo!()
    }

    async fn get_flags(
        &mut self,
        _this: filesystem::Descriptor,
    ) -> Result<filesystem::DescriptorFlags, filesystem::Error> {
        todo!()
    }

    async fn get_type(
        &mut self,
        this: filesystem::Descriptor,
    ) -> Result<filesystem::DescriptorType, filesystem::Error> {
        todo!()
    }

    async fn set_size(
        &mut self,
        _this: filesystem::Descriptor,
        _size: filesystem::Filesize,
    ) -> Result<(), filesystem::Error> {
        todo!()
    }

    async fn set_times(
        &mut self,
        _this: filesystem::Descriptor,
        _data_access_timestamp: filesystem::NewTimestamp,
        _data_modification_timestamp: filesystem::NewTimestamp,
    ) -> Result<(), filesystem::Error> {
        todo!()
    }

    async fn read(
        &mut self,
        _this: filesystem::Descriptor,
        _length: filesystem::Filesize,
        _offset: filesystem::Filesize,
    ) -> Result<(Vec<u8>, bool), filesystem::Error> {
        todo!()
    }

    async fn write(
        &mut self,
        _this: filesystem::Descriptor,
        _buffer: Vec<u8>,
        _offset: filesystem::Filesize,
    ) -> Result<filesystem::Filesize, filesystem::Error> {
        todo!()
    }

    async fn read_directory(
        &mut self,
        this: filesystem::Descriptor,
    ) -> Result<filesystem::DirectoryEntryStream, filesystem::Error> {
        todo!()
    }

    async fn sync(
        &mut self,
        _this: filesystem::Descriptor,
    ) -> Result<(), filesystem::Error> {
        todo!()
    }

    async fn create_directory_at(
        &mut self,
        this: filesystem::Descriptor,
        path: String,
    ) -> Result<(), filesystem::Error> {
        todo!()
    }

    async fn stat(
        &mut self,
        this: filesystem::Descriptor,
    ) -> Result<filesystem::DescriptorStat, filesystem::Error> {
        todo!()
    }

    async fn stat_at(
        &mut self,
        this: filesystem::Descriptor,
        path_flags: filesystem::PathFlags,
        path: String,
    ) -> Result<filesystem::DescriptorStat, filesystem::Error> {
        todo!()
    }

    async fn set_times_at(
        &mut self,
        _this: filesystem::Descriptor,
        _path_flags: filesystem::PathFlags,
        _path: String,
        _data_access_timestamp: filesystem::NewTimestamp,
        _data_modification_timestamp: filesystem::NewTimestamp,
    ) -> Result<(), filesystem::Error> {
        todo!()
    }

    async fn link_at(
        &mut self,
        _this: filesystem::Descriptor,
        _old_path_flags: filesystem::PathFlags,
        _old_path: String,
        _new_descriptor: filesystem::Descriptor,
        _new_path: String,
    ) -> Result<(), filesystem::Error> {
        todo!()
    }

    async fn open_at(
        &mut self,
        this: filesystem::Descriptor,
        path_flags: filesystem::PathFlags,
        path: String,
        open_flags: filesystem::OpenFlags,
        flags: filesystem::DescriptorFlags,
        _modes: filesystem::Modes,
    ) -> Result<filesystem::Descriptor, filesystem::Error> {
        todo!()
    }

    async fn readlink_at(
        &mut self,
        _this: filesystem::Descriptor,
        _path: String,
    ) -> Result<String, filesystem::Error> {
        todo!()
    }

    async fn remove_directory_at(
        &mut self,
        this: filesystem::Descriptor,
        path: String,
    ) -> Result<(), filesystem::Error> {
        todo!()
    }

    async fn rename_at(
        &mut self,
        _this: filesystem::Descriptor,
        _old_path: String,
        _new_descriptor: filesystem::Descriptor,
        _new_path: String,
    ) -> Result<(), filesystem::Error> {
        todo!()
    }

    async fn symlink_at(
        &mut self,
        _this: filesystem::Descriptor,
        _old_path: String,
        _new_path: String,
    ) -> Result<(), filesystem::Error> {
        todo!()
    }

    async fn unlink_file_at(
        &mut self,
        this: filesystem::Descriptor,
        path: String,
    ) -> Result<(), filesystem::Error> {
        todo!()
    }

    async fn change_file_permissions_at(
        &mut self,
        _this: filesystem::Descriptor,
        _path_flags: filesystem::PathFlags,
        _path: String,
        _modes: filesystem::Modes,
    ) -> Result<(), filesystem::Error> {
        todo!()
    }

    async fn change_directory_permissions_at(
        &mut self,
        _this: filesystem::Descriptor,
        _path_flags: filesystem::PathFlags,
        _path: String,
        _modes: filesystem::Modes,
    ) -> Result<(), filesystem::Error> {
        todo!()
    }

    async fn lock_shared(
        &mut self,
        _this: filesystem::Descriptor,
    ) -> Result<(), filesystem::Error> {
        todo!()
    }

    async fn lock_exclusive(
        &mut self,
        _this: filesystem::Descriptor,
    ) -> Result<(), filesystem::Error> {
        todo!()
    }

    async fn try_lock_shared(
        &mut self,
        _this: filesystem::Descriptor,
    ) -> Result<(), filesystem::Error> {
        todo!()
    }

    async fn try_lock_exclusive(
        &mut self,
        _this: filesystem::Descriptor,
    ) -> Result<(), filesystem::Error> {
        todo!()
    }

    async fn unlock(
        &mut self,
        _this: filesystem::Descriptor,
    ) -> Result<(), filesystem::Error> {
        todo!()
    }

    async fn drop_descriptor(
        &mut self,
        this: filesystem::Descriptor,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn read_directory_entry(
        &mut self,
        this: filesystem::DirectoryEntryStream,
    ) -> Result<Option<filesystem::DirectoryEntry>, filesystem::Error> {
        todo!()
    }

    async fn drop_directory_entry_stream(
        &mut self,
        this: filesystem::DirectoryEntryStream,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    async fn set_flags(
        &mut self,
        _this: filesystem::Descriptor,
        _flags: filesystem::DescriptorFlags,
    ) -> Result<(), filesystem::Error> {
        todo!()
    }

    async fn access_at(
        &mut self,
        _this: filesystem::Descriptor,
        _path_flags: filesystem::PathFlags,
        _path: String,
        _type_: filesystem::AccessType,
    ) -> Result<(), filesystem::Error> {
        todo!()
    }
}

#[async_trait]
impl streams::Host for WasiCtx {
    async fn read(
        &mut self,
        this: streams::InputStream,
        len: u64,
    ) -> Result<(Vec<u8>, bool), streams::Error> {
        todo!()
    }

    async fn blocking_read(
        &mut self,
        this: streams::InputStream,
        len: u64,
    ) -> Result<(Vec<u8>, bool), streams::Error> {
        todo!()
    }

    async fn skip(
        &mut self,
        this: streams::InputStream,
        len: u64,
    ) -> Result<(u64, bool), streams::Error> {
        todo!()
    }

    async fn blocking_skip(
        &mut self,
        this: streams::InputStream,
        len: u64,
    ) -> Result<(u64, bool), streams::Error> {
        todo!()
    }

    async fn subscribe_to_input_stream(
        &mut self,
        this: streams::InputStream,
    ) -> anyhow::Result<poll::Pollable> {
        todo!()
    }

    async fn drop_input_stream(
        &mut self,
        this: streams::InputStream,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn write(
        &mut self,
        this: streams::OutputStream,
        buf: Vec<u8>,
    ) -> Result<u64, streams::Error> {
        Ok(buf.len() as u64)
    }

    async fn blocking_write(
        &mut self,
        this: streams::OutputStream,
        buf: Vec<u8>,
    ) -> Result<u64, streams::Error> {
        Ok(buf.len() as u64)
    }

    async fn write_zeroes(
        &mut self,
        this: streams::OutputStream,
        len: u64,
    ) -> Result<u64, streams::Error> {
        todo!()
    }

    async fn blocking_write_zeroes(
        &mut self,
        this: streams::OutputStream,
        len: u64,
    ) -> Result<u64, streams::Error> {
        todo!()
    }

    async fn splice(
        &mut self,
        _this: streams::OutputStream,
        _src: streams::InputStream,
        _len: u64,
    ) -> Result<(u64, bool), streams::Error> {
        todo!()
    }

    async fn blocking_splice(
        &mut self,
        _this: streams::OutputStream,
        _src: streams::InputStream,
        _len: u64,
    ) -> Result<(u64, bool), streams::Error> {
        todo!()
    }

    async fn forward(
        &mut self,
        _this: streams::OutputStream,
        _src: streams::InputStream,
    ) -> Result<u64, streams::Error> {
        todo!()
    }

    async fn subscribe_to_output_stream(
        &mut self,
        this: streams::OutputStream,
    ) -> anyhow::Result<poll::Pollable> {
        todo!()
    }

    async fn drop_output_stream(
        &mut self,
        this: streams::OutputStream,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

#[async_trait]
impl environment::Host for WasiCtx {
    async fn get_environment(&mut self) -> anyhow::Result<Vec<(String, String)>> {
        Ok(vec!())
    }

    async fn get_arguments(&mut self) -> anyhow::Result<Vec<String>> {
        Ok(vec!())
    }
}

#[async_trait]
impl preopens::Host for WasiCtx {
    async fn get_stdio(&mut self) -> anyhow::Result<preopens::StdioPreopens> {
        Ok(preopens::StdioPreopens {
            stdin: 0,
            stdout: 0,
            stderr: 0,
        })
    }

    async fn get_directories(
        &mut self,
    ) -> anyhow::Result<Vec<(preopens::Descriptor, String)>> {
        Ok(vec!())
    }
}

#[async_trait]
impl exit::Host for WasiCtx {
    async fn exit(&mut self, status: Result<(), ()>) -> anyhow::Result<()> {
        Ok(())
    }
}
