use rand::distributions::{Alphanumeric, DistString};

pub fn randomize(haystack: &mut [u8], needle: &[u8]) {
    let matches: Vec<usize> = memchr::memmem::find_iter(haystack, needle).collect();
    let times: usize = matches.len();
    eprintln!("Original Haystack: {:?}", String::from_utf8_lossy(haystack));
    
    let mut sampled: Vec<u8> = Alphanumeric.sample_string(&mut rand::thread_rng(), needle.len()).into_bytes();
    if b"012345789".contains(&sampled[0]) {
        sampled[0] = b'_';
    }

    for m in matches {
        let replaceat: &mut [u8] = &mut haystack[m..m + needle.len()];
        replaceat.copy_from_slice(&sampled);
    }
    
    eprintln!(
        "Replaced {} time(s) '{}' by '{}'",
        times,
        String::from_utf8_lossy(needle),
        String::from_utf8_lossy(&sampled),
    );
    eprintln!("Modified Haystack: {:?}", String::from_utf8_lossy(haystack));
}
