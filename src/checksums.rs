use crc::Crc;

/// Compute the CRC-32 checksum of a byte slice as per ISO 3309 and ITU-T V.42.
pub fn crc32(bytes: &[u8]) -> u32 {
    const CRC_32_ISO_HDLC: Crc<u32> = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
    let mut hasher = CRC_32_ISO_HDLC.digest();
    hasher.update(bytes);
    hasher.finalize()
}

/// Compute the Adler-32 checksum of a byte slice
pub fn adler32(bytes: &[u8]) -> u32 {
    simd_adler32::adler32(&bytes)
}
