extern crate envconfig;
extern crate envconfig_derive;

pub mod download_queue;
pub mod file_downloader;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
