use super::{
    utils::{write_delta, BitSliceCast},
    *,
};

pub struct PacketEntities {}
impl<'a> NetMsgDoerWithExtraInfo<'a, SvcPacketEntities> for PacketEntities {
    fn parse(
        i: &'a [u8],
        delta_decoders: &mut DeltaDecoderTable,
        max_client: u8,
    ) -> IResult<&'a [u8], SvcPacketEntities> {
        let mut br = BitReader::new(i);

        let entity_count = br.read_n_bit(16).to_owned();
        let mut entity_index = 0;
        let mut entity_states: Vec<EntityState> = vec![];

        loop {
            let footer = br.peek_n_bits(16).to_u16();
            if footer == 0 {
                br.read_n_bit(16);
                break;
            }

            let increment_entity_number = br.read_1_bit();
            let is_absolute_entity_index = if increment_entity_number {
                entity_index += 1;
                None
            } else {
                Some(br.read_1_bit())
            };
            let (absolute_entity_index, entity_index_difference) =
                if is_absolute_entity_index.is_some() {
                    if !is_absolute_entity_index.unwrap() {
                        let val = br.read_n_bit(6).to_owned();
                        entity_index += val.to_u16();
                        (None, Some(val))
                    } else {
                        let val = br.read_n_bit(11).to_owned();
                        entity_index = val.to_u16();
                        (Some(val), None)
                    }
                } else {
                    (None, None)
                };

            let has_custom_delta = br.read_1_bit();
            let has_baseline_index = br.read_1_bit();
            let baseline_index = if has_baseline_index {
                Some(br.read_n_bit(6).to_owned())
            } else {
                None
            };
            let between = entity_index > 0 && entity_index <= max_client as u16;

            let delta = if between {
                parse_delta(
                    delta_decoders.get("entity_state_player_t\0").unwrap(),
                    &mut br,
                )
            } else {
                if has_custom_delta {
                    parse_delta(
                        delta_decoders.get("custom_entity_state_t\0").unwrap(),
                        &mut br,
                    )
                } else {
                    parse_delta(delta_decoders.get("entity_state_t\0").unwrap(), &mut br)
                }
            };

            entity_states.push(EntityState {
                entity_index,
                increment_entity_number,
                is_absolute_entity_index,
                absolute_entity_index,
                entity_index_difference,
                has_custom_delta,
                has_baseline_index,
                baseline_index,
                delta,
            })
        }

        let range = br.get_consumed_bytes();
        let (i, _) = take(range)(i)?;

        Ok((
            i,
            SvcPacketEntities {
                entity_count,
                entity_states,
            },
        ))
    }

    fn write(
        i: SvcPacketEntities,
        delta_decoders: &mut DeltaDecoderTable,
        max_client: u8,
    ) -> Vec<u8> {
        let mut writer = ByteWriter::new();
        let mut bw = BitWriter::new();

        writer.append_u8(EngineMessageType::SvcPacketEntities as u8);

        bw.append_vec(i.entity_count);

        for entity in i.entity_states {
            bw.append_bit(entity.increment_entity_number);

            if !entity.increment_entity_number {
                bw.append_bit(entity.is_absolute_entity_index.unwrap());

                if entity.is_absolute_entity_index.unwrap() {
                    bw.append_vec(entity.absolute_entity_index.unwrap());
                } else {
                    bw.append_vec(entity.entity_index_difference.unwrap());
                }
            }

            bw.append_bit(entity.has_custom_delta);
            bw.append_bit(entity.has_baseline_index);

            if entity.has_baseline_index {
                bw.append_vec(entity.baseline_index.unwrap());
            }

            let between = entity.entity_index > 0 && entity.entity_index <= max_client as u16;
            if between {
                write_delta(
                    &entity.delta,
                    delta_decoders.get_mut("entity_state_player_t\0").unwrap(),
                    &mut bw,
                )
            } else {
                if entity.has_custom_delta {
                    write_delta(
                        &entity.delta,
                        delta_decoders.get_mut("custom_entity_state_t\0").unwrap(),
                        &mut bw,
                    )
                } else {
                    write_delta(
                        &entity.delta,
                        delta_decoders.get_mut("entity_state_t\0").unwrap(),
                        &mut bw,
                    )
                }
            }
        }

        bw.append_vec(bitvec![u8, Lsb0; 0; 16]);

        writer.append_u8_slice(&bw.get_u8_vec());

        writer.data
    }
}
