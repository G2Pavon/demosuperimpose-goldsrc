use super::{utils::BitSliceCast, *};

pub struct ResourceList {}
impl<'a> NetMsgDoer<'a, SvcResourceList> for ResourceList {
    fn parse(i: &'a [u8], _: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcResourceList> {
        let mut br = BitReader::new(i);

        let resource_count = br.read_n_bit(12).to_owned();

        let resources: Vec<Resource> = (0..resource_count.to_u16())
            .map(|_| {
                let type_ = br.read_n_bit(4).to_owned();
                let name = br.read_string().to_owned();
                let index = br.read_n_bit(12).to_owned();
                let size = br.read_n_bit(24).to_owned();
                let flags = br.read_n_bit(3).to_owned();
                let md5_hash = if flags.to_u8() & 4 != 0 {
                    Some(br.read_n_bit(128).to_owned())
                } else {
                    None
                };
                let has_extra_info = br.read_1_bit();
                let extra_info = if has_extra_info {
                    Some(br.read_n_bit(256).to_owned())
                } else {
                    None
                };

                Resource {
                    type_,
                    name,
                    index,
                    size,
                    flags,
                    md5_hash,
                    has_extra_info,
                    extra_info,
                }
            })
            .collect();

        let mut consistencies: Vec<Consistency> = vec![];

        loop {
            let has_check_file_flag = br.read_1_bit();

            if has_check_file_flag {
                let is_short_index = br.read_1_bit();

                let (short_index, long_index) = if is_short_index {
                    (Some(br.read_n_bit(5).to_owned()), None)
                } else {
                    (None, Some(br.read_n_bit(10).to_owned()))
                };

                // (Some(is_short_index), short_index, long_index)
                consistencies.push(Consistency {
                    has_check_file_flag,
                    is_short_index: Some(is_short_index),
                    short_index,
                    long_index,
                });
            } else {
                consistencies.push(Consistency {
                    has_check_file_flag: false,
                    is_short_index: None,
                    short_index: None,
                    long_index: None,
                });
                break;
            }
        }

        let (i, _) = take(br.get_consumed_bytes())(i)?;

        Ok((
            i,
            SvcResourceList {
                resource_count,
                resources,
                consistencies,
            },
        ))
    }

    fn write(i: SvcResourceList) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        writer.append_u8(EngineMessageType::SvcResourceList as u8);

        let mut bw = BitWriter::new();
        bw.append_vec(i.resource_count);

        for resource in i.resources {
            bw.append_vec(resource.type_);
            bw.append_vec(resource.name);
            bw.append_vec(resource.index);
            bw.append_vec(resource.size);

            let should_add_md5_hash = resource.flags.to_u8() & 4 != 0;

            bw.append_vec(resource.flags);

            if should_add_md5_hash {
                bw.append_vec(resource.md5_hash.unwrap());
            }

            bw.append_bit(resource.has_extra_info);

            if resource.has_extra_info {
                bw.append_vec(resource.extra_info.unwrap());
            }
        }

        for consistency in i.consistencies {
            bw.append_bit(consistency.has_check_file_flag);

            if consistency.has_check_file_flag {
                bw.append_bit(consistency.is_short_index.unwrap());
                if consistency.is_short_index.unwrap() {
                    bw.append_vec(consistency.short_index.unwrap());
                } else {
                    bw.append_vec(consistency.long_index.unwrap());
                }
            }
        }

        writer.append_u8_slice(&bw.get_u8_vec());

        writer.data
    }
}