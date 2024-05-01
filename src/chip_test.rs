use luwen::luwen_core::Arch;
use luwen::ttkmd_if::PciDevice;

#[test]
fn read_write_test() {
    for id in PciDevice::scan() {
        let chip = if let Ok(chip) = crate::open(id) {
            chip
        } else {
            continue;
        };

        let addr = 3 as u64;
        let aligned_addr = (addr + 3) & !3;

        let noc_id = 0;
        let (noc_x, noc_y) = chip.noc.tensix[0];
        let (noc_x, noc_y) = (noc_x as u8, noc_y as u8);

        chip.noc_write32(noc_id, noc_x, noc_y, aligned_addr, 0xfaca)
            .unwrap();
        let readback = chip.noc_read32(noc_id, noc_x, noc_y, aligned_addr).unwrap();
        assert_eq!(readback, 0xfaca, "{:x} != faca", readback);

        chip.noc_write32(noc_id, noc_x, noc_y, aligned_addr, 0xcdcd_cdcd)
            .unwrap();
        let readback = chip.noc_read32(noc_id, noc_x, noc_y, aligned_addr).unwrap();
        assert_eq!(readback, 0xcdcd_cdcd, "{:x} != cdcdcdcd", readback);

        chip.noc_write32(noc_id, noc_x, noc_y, aligned_addr + 4, 0xcdcd_cdcd)
            .unwrap();
        let readback = chip
            .noc_read32(noc_id, noc_x, noc_y, aligned_addr + 4)
            .unwrap();
        assert_eq!(readback, 0xcdcd_cdcd, "{:x} != cdcdcdcd", readback);

        chip.noc_write32(noc_id, noc_x, noc_y, aligned_addr + 1, 0xdead)
            .unwrap();
        let readback = chip.noc_read32(noc_id, noc_x, noc_y, aligned_addr).unwrap();
        assert_eq!(readback, 0xdeadcd, "{:x} != deadcd", readback);
        let readback = chip
            .noc_read32(noc_id, noc_x, noc_y, aligned_addr + 4)
            .unwrap();
        assert_eq!(readback, 0xcdcdcd00, "{:x} != 00cdcdcd", readback);

        chip.noc_write32(noc_id, noc_x, noc_y, aligned_addr, 0xcdcd_cdcd)
            .unwrap();
        let readback = chip.noc_read32(noc_id, noc_x, noc_y, aligned_addr).unwrap();
        assert_eq!(readback, 0xcdcd_cdcd, "{:x} != cdcdcdcd", readback);

        chip.noc_write32(noc_id, noc_x, noc_y, aligned_addr + 4, 0xcdcd_cdcd)
            .unwrap();
        let readback = chip
            .noc_read32(noc_id, noc_x, noc_y, aligned_addr + 4)
            .unwrap();
        assert_eq!(readback, 0xcdcd_cdcd, "{:x} != cdcdcdcd", readback);

        chip.noc_write32(noc_id, noc_x, noc_y, aligned_addr + 3, 0xc0ffe)
            .unwrap();
        let readback = chip.noc_read32(noc_id, noc_x, noc_y, aligned_addr).unwrap();
        assert_eq!(readback, 0xfecdcdcd, "{:x} != fecdcdcd", readback);
        let readback = chip
            .noc_read32(noc_id, noc_x, noc_y, aligned_addr + 4)
            .unwrap();
        assert_eq!(readback, 0xcd000c0f, "{:x} != c0f", readback);

        chip.noc_write32(noc_id, noc_x, noc_y, aligned_addr, 0x01234567)
            .unwrap();
        let readback = chip.noc_read32(noc_id, noc_x, noc_y, aligned_addr).unwrap();
        assert_eq!(readback, 0x01234567, "{:x} != 01234567", readback);

        chip.noc_write32(noc_id, noc_x, noc_y, aligned_addr + 4, 0xabcdef)
            .unwrap();
        let readback = chip
            .noc_read32(noc_id, noc_x, noc_y, aligned_addr + 4)
            .unwrap();
        assert_eq!(readback, 0xabcdef, "{:x} != abcdef", readback);

        let readback = chip
            .noc_read32(noc_id, noc_x, noc_y, aligned_addr + 1)
            .unwrap();
        assert_eq!(readback, 0xef012345, "{:x} != ef012345", readback);

        let readback = chip
            .noc_read32(noc_id, noc_x, noc_y, aligned_addr + 3)
            .unwrap();
        assert_eq!(readback, 0xabcdef01, "{:x} != abcdef01", readback);

        // Block write test
        let mut write_buffer = Vec::new();
        write_buffer.extend(0xcdcd_cdcdu32.to_le_bytes());
        write_buffer.extend(0xcdcd_cdcdu32.to_le_bytes());
        chip.noc_write(noc_id, noc_x, noc_y, aligned_addr, &write_buffer)
            .unwrap();

        let mut readback_buffer = vec![0u8; write_buffer.len()];
        chip.noc_read(noc_id, noc_x, noc_y, aligned_addr, &mut readback_buffer)
            .unwrap();
        assert_eq!(write_buffer, readback_buffer);

        let mut write_buffer = Vec::new();
        write_buffer.push(0xad);
        write_buffer.push(0xde);
        chip.noc_write(noc_id, noc_x, noc_y, aligned_addr + 1, &write_buffer)
            .unwrap();

        let mut readback_buffer = vec![0u8; 4];
        chip.noc_read(noc_id, noc_x, noc_y, aligned_addr, &mut readback_buffer)
            .unwrap();
        assert_eq!([0xcd, 0xad, 0xde, 0xcd], readback_buffer.as_slice());

        let mut write_buffer = Vec::new();
        write_buffer.extend(0xcdcd_cdcdu32.to_le_bytes());
        write_buffer.extend(0xcdcd_cdcdu32.to_le_bytes());
        chip.noc_write(noc_id, noc_x, noc_y, aligned_addr, &write_buffer)
            .unwrap();

        let mut readback_buffer = vec![0u8; write_buffer.len()];
        chip.noc_read(noc_id, noc_x, noc_y, aligned_addr, &mut readback_buffer)
            .unwrap();
        assert_eq!(write_buffer, readback_buffer);

        let mut write_buffer = Vec::new();
        write_buffer.push(0xad);
        write_buffer.push(0xde);
        chip.noc_write(noc_id, noc_x, noc_y, aligned_addr + 3, &write_buffer)
            .unwrap();

        let mut readback_buffer = vec![0u8; 7];
        chip.noc_read(noc_id, noc_x, noc_y, aligned_addr, &mut readback_buffer)
            .unwrap();
        assert_eq!(
            [0xcd, 0xcd, 0xcd, 0xad, 0xde, 0xcd, 0xcd],
            readback_buffer.as_slice()
        );

        let mut write_buffer = Vec::new();
        write_buffer.extend(0x01234567u32.to_le_bytes());
        write_buffer.extend(0xabcdefu32.to_le_bytes());
        chip.noc_write(noc_id, noc_x, noc_y, aligned_addr, &write_buffer)
            .unwrap();

        let mut readback_buffer = vec![0u8; write_buffer.len()];
        chip.noc_read(noc_id, noc_x, noc_y, aligned_addr, &mut readback_buffer)
            .unwrap();
        assert_eq!(write_buffer, readback_buffer);

        let readback = chip
            .noc_read32(noc_id, noc_x, noc_y, aligned_addr + 1)
            .unwrap();
        assert_eq!(readback, 0xef012345, "{:x} != ef012345", readback);

        let mut readback_buffer = vec![0u8; 4];
        chip.noc_read(noc_id, noc_x, noc_y, aligned_addr + 1, &mut readback_buffer)
            .unwrap();
        assert_eq!([0x45, 0x23, 0x01, 0xef], readback_buffer.as_slice());

        let mut readback_buffer = vec![0u8; 4];
        chip.noc_read(noc_id, noc_x, noc_y, aligned_addr + 3, &mut readback_buffer)
            .unwrap();
        assert_eq!([0x01, 0xef, 0xcd, 0xab], readback_buffer.as_slice());

        let mut write_buffer = vec![0; 1024];
        for (index, r) in write_buffer.iter_mut().enumerate() {
            *r = index as u8;
        }
        chip.noc_write(noc_id, noc_x, noc_y, aligned_addr, &write_buffer)
            .unwrap();

        let mut readback_buffer = vec![0u8; write_buffer.len()];
        chip.noc_read(noc_id, noc_x, noc_y, aligned_addr, &mut readback_buffer)
            .unwrap();
        assert_eq!(write_buffer, readback_buffer);

        let mut write_buffer = vec![0; 1024];
        for (index, r) in write_buffer.iter_mut().enumerate() {
            *r = index as u8;
        }
        chip.noc_write(noc_id, noc_x, noc_y, aligned_addr, &write_buffer)
            .unwrap();

        let mut readback_buffer = vec![0u8; write_buffer.len()];
        chip.noc_read(noc_id, noc_x, noc_y, aligned_addr + 3, &mut readback_buffer)
            .unwrap();
        assert_eq!(
            write_buffer[3..],
            readback_buffer[..readback_buffer.len() - 3]
        );

        let mut write_buffer = vec![0; 1024];
        for (index, r) in write_buffer.iter_mut().enumerate() {
            *r = index as u8;
        }
        chip.noc_write(noc_id, noc_x, noc_y, aligned_addr + 1, &write_buffer)
            .unwrap();

        let mut readback_buffer = vec![0u8; write_buffer.len()];
        chip.noc_read(noc_id, noc_x, noc_y, aligned_addr + 1, &mut readback_buffer)
            .unwrap();
        assert_eq!(write_buffer, readback_buffer);
    }
}

#[test]
fn arc_postcode_sanity() {
    for id in PciDevice::scan() {
        let chip = if let Ok(chip) = crate::open(id) {
            chip
        } else {
            continue;
        };

        if chip.arch().is_wormhole() || chip.arch().is_grayskull() {
            let postcode = chip.device.read32(0x1ff30060).unwrap();
            assert_eq!(postcode >> 16, 0xC0DE);
        }
    }
}

#[test]
fn arc_read_write_test() {
    for id in PciDevice::scan() {
        let mut chip = if let Ok(chip) = crate::open(id) {
            chip
        } else {
            continue;
        };

        if chip.arch().is_wormhole() {
            let dump_addr = crate::chip::wormhole::arc_msg(
                &chip,
                &crate::chip::wormhole::ArcMsg::GetSpiDumpAddr,
                true,
                std::time::Duration::from_secs(1),
                5,
                3,
                &crate::chip::wormhole::ArcMsgAddr {
                    scratch_base: 0x1ff30060,
                    arc_misc_cntl: 0x1ff30100,
                },
            )
            .unwrap()
            .arg();

            assert!(dump_addr > 0);

            let raw_device = &mut chip.device;

            let csm_offset = 0x1fe80000 - 0x10000000_u64;

            let addr = csm_offset + (dump_addr as u64);

            let aligned_addr = (addr + 3) & !3;

            raw_device.write32(aligned_addr as u32, 0xfaca).unwrap();
            let readback = raw_device.read32(aligned_addr as u32).unwrap();
            assert_eq!(readback, 0xfaca, "{:x} != faca", readback);

            raw_device
                .write32(aligned_addr as u32, 0xcdcd_cdcd)
                .unwrap();
            let readback = raw_device.read32(aligned_addr as u32).unwrap();
            assert_eq!(readback, 0xcdcd_cdcd, "{:x} != cdcdcdcd", readback);

            raw_device
                .write32(aligned_addr as u32 + 4, 0xcdcd_cdcd)
                .unwrap();
            let readback = raw_device.read32(aligned_addr as u32 + 4).unwrap();
            assert_eq!(readback, 0xcdcd_cdcd, "{:x} != cdcdcdcd", readback);

            raw_device.write32(aligned_addr as u32 + 1, 0xdead).unwrap();
            let readback = raw_device.read32(aligned_addr as u32).unwrap();
            assert_eq!(readback, 0xdeadcd, "{:x} != deadcd", readback);
            let readback = raw_device.read32(aligned_addr as u32 + 4).unwrap();
            assert_eq!(readback, 0xcdcdcd00, "{:x} != 00cdcdcd", readback);

            raw_device
                .write32(aligned_addr as u32, 0xcdcd_cdcd)
                .unwrap();
            let readback = raw_device.read32(aligned_addr as u32).unwrap();
            assert_eq!(readback, 0xcdcd_cdcd, "{:x} != cdcdcdcd", readback);

            raw_device
                .write32(aligned_addr as u32 + 4, 0xcdcd_cdcd)
                .unwrap();
            let readback = raw_device.read32(aligned_addr as u32 + 4).unwrap();
            assert_eq!(readback, 0xcdcd_cdcd, "{:x} != cdcdcdcd", readback);

            raw_device
                .write32(aligned_addr as u32 + 3, 0xc0ffe)
                .unwrap();
            let readback = raw_device.read32(aligned_addr as u32).unwrap();
            assert_eq!(readback, 0xfecdcdcd, "{:x} != fecdcdcd", readback);
            let readback = raw_device.read32(aligned_addr as u32 + 4).unwrap();
            assert_eq!(readback, 0xcd000c0f, "{:x} != c0f", readback);

            raw_device.write32(aligned_addr as u32, 0x01234567).unwrap();
            let readback = raw_device.read32(aligned_addr as u32).unwrap();
            assert_eq!(readback, 0x01234567, "{:x} != 01234567", readback);

            raw_device
                .write32(aligned_addr as u32 + 4, 0xabcdef)
                .unwrap();
            let readback = raw_device.read32(aligned_addr as u32 + 4).unwrap();
            assert_eq!(readback, 0xabcdef, "{:x} != abcdef", readback);

            let readback = raw_device.read32(aligned_addr as u32 + 1).unwrap();
            assert_eq!(readback, 0xef012345, "{:x} != ef012345", readback);

            let readback = raw_device.read32(aligned_addr as u32 + 3).unwrap();
            assert_eq!(readback, 0xabcdef01, "{:x} != abcdef01", readback);

            // Block write test
            let mut write_buffer = Vec::new();
            write_buffer.extend(0xcdcd_cdcdu32.to_le_bytes());
            write_buffer.extend(0xcdcd_cdcdu32.to_le_bytes());
            raw_device
                .write_block(aligned_addr as u32, &write_buffer)
                .unwrap();

            let mut readback_buffer = vec![0u8; write_buffer.len()];
            raw_device
                .read_block(aligned_addr as u32, &mut readback_buffer)
                .unwrap();
            assert_eq!(write_buffer, readback_buffer);

            let mut write_buffer = Vec::new();
            write_buffer.push(0xad);
            write_buffer.push(0xde);
            raw_device
                .write_block(aligned_addr as u32 + 1, &write_buffer)
                .unwrap();

            let mut readback_buffer = vec![0u8; 4];
            raw_device
                .read_block(aligned_addr as u32, &mut readback_buffer)
                .unwrap();
            assert_eq!([0xcd, 0xad, 0xde, 0xcd], readback_buffer.as_slice());

            let mut write_buffer = Vec::new();
            write_buffer.extend(0xcdcd_cdcdu32.to_le_bytes());
            write_buffer.extend(0xcdcd_cdcdu32.to_le_bytes());
            raw_device
                .write_block(aligned_addr as u32, &write_buffer)
                .unwrap();

            let mut readback_buffer = vec![0u8; write_buffer.len()];
            raw_device
                .read_block(aligned_addr as u32, &mut readback_buffer)
                .unwrap();
            assert_eq!(write_buffer, readback_buffer);

            let mut write_buffer = Vec::new();
            write_buffer.push(0xad);
            write_buffer.push(0xde);
            raw_device
                .write_block(aligned_addr as u32 + 3, &write_buffer)
                .unwrap();

            let mut readback_buffer = vec![0u8; 7];
            raw_device
                .read_block(aligned_addr as u32, &mut readback_buffer)
                .unwrap();
            assert_eq!(
                [0xcd, 0xcd, 0xcd, 0xad, 0xde, 0xcd, 0xcd],
                readback_buffer.as_slice()
            );

            let mut write_buffer = Vec::new();
            write_buffer.extend(0x01234567u32.to_le_bytes());
            write_buffer.extend(0xabcdefu32.to_le_bytes());
            raw_device
                .write_block(aligned_addr as u32, &write_buffer)
                .unwrap();

            let mut readback_buffer = vec![0u8; write_buffer.len()];
            raw_device
                .read_block(aligned_addr as u32, &mut readback_buffer)
                .unwrap();
            assert_eq!(write_buffer, readback_buffer);

            let readback = raw_device.read32(aligned_addr as u32 + 1).unwrap();
            assert_eq!(readback, 0xef012345, "{:x} != ef012345", readback);

            let mut readback_buffer = vec![0u8; 4];
            raw_device
                .read_block(aligned_addr as u32 + 1, &mut readback_buffer)
                .unwrap();
            assert_eq!([0x45, 0x23, 0x01, 0xef], readback_buffer.as_slice());

            let mut readback_buffer = vec![0u8; 4];
            raw_device
                .read_block(aligned_addr as u32 + 3, &mut readback_buffer)
                .unwrap();
            assert_eq!([0x01, 0xef, 0xcd, 0xab], readback_buffer.as_slice());

            let mut write_buffer = vec![0; 1024];
            for (index, r) in write_buffer.iter_mut().enumerate() {
                *r = index as u8;
            }
            raw_device
                .write_block(aligned_addr as u32, &write_buffer)
                .unwrap();

            let mut readback_buffer = vec![0u8; write_buffer.len()];
            raw_device
                .read_block(aligned_addr as u32, &mut readback_buffer)
                .unwrap();
            assert_eq!(write_buffer, readback_buffer);

            let mut write_buffer = vec![0; 1024];
            for (index, r) in write_buffer.iter_mut().enumerate() {
                *r = index as u8;
            }
            raw_device
                .write_block(aligned_addr as u32, &write_buffer)
                .unwrap();

            let mut readback_buffer = vec![0u8; write_buffer.len()];
            raw_device
                .read_block(aligned_addr as u32 + 3, &mut readback_buffer)
                .unwrap();
            assert_eq!(
                write_buffer[3..],
                readback_buffer[..readback_buffer.len() - 3]
            );

            let mut write_buffer = vec![0; 1024];
            for (index, r) in write_buffer.iter_mut().enumerate() {
                *r = index as u8;
            }
            raw_device
                .write_block(aligned_addr as u32 + 1, &write_buffer)
                .unwrap();

            let mut readback_buffer = vec![0u8; write_buffer.len()];
            raw_device
                .read_block(aligned_addr as u32 + 1, &mut readback_buffer)
                .unwrap();
            assert_eq!(write_buffer, readback_buffer);
        }
    }
}

#[test]
fn arc_msg_test() {
    for id in PciDevice::scan() {
        let chip = if let Ok(chip) = crate::open(id) {
            chip
        } else {
            continue;
        };

        let input = 100;

        let (rc, result) = match chip.device.arch {
            Arch::Grayskull => {
                let result = crate::chip::grayskull::arc_msg(
                    &chip,
                    &crate::chip::grayskull::ArcMsg::Test { arg: input },
                    true,
                    std::time::Duration::from_secs(1),
                    5,
                    3,
                    &crate::chip::grayskull::ArcMsgAddr {
                        scratch_base: 0x1ff30060,
                        arc_misc_cntl: 0x1ff30100,
                    },
                )
                .unwrap();

                (result.rc(), result.arg())
            }
            Arch::Wormhole => {
                let result = crate::chip::wormhole::arc_msg(
                    &chip,
                    &crate::chip::wormhole::ArcMsg::Test { arg: input },
                    true,
                    std::time::Duration::from_secs(1),
                    5,
                    3,
                    &crate::chip::wormhole::ArcMsgAddr {
                        scratch_base: 0x1ff30060,
                        arc_misc_cntl: 0x1ff30100,
                    },
                )
                .unwrap();

                (result.rc(), result.arg())
            }
            Arch::Blackhole => (0, input + 1),
        };

        assert_eq!(rc, 0, "For {}[{id}] ARC msg failed", chip.device.arch);
        assert_eq!(
            result,
            input + 1,
            "For {}[{id}] ARC test msg failed",
            chip.device.arch
        );
    }
}
