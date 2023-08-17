use super::{utils::BitSliceCast, *};

pub struct Event {}
impl<'a> NetMsgDoer<'a, SvcEvent> for Event {
    fn parse(i: &'a [u8], delta_decoders: &mut DeltaDecoderTable) -> IResult<&'a [u8], SvcEvent> {
        let mut br = BitReader::new(i);

        let event_count = br.read_n_bit(5).to_owned();

        let events = (0..event_count.to_u8())
            .map(|_| {
                let event_index = br.read_n_bit(10).to_owned();
                let has_packet_index = br.read_1_bit();
                let packet_index = if has_packet_index {
                    Some(br.read_n_bit(11).to_owned())
                } else {
                    None
                };
                let has_delta = if has_packet_index {
                    Some(br.read_1_bit())
                } else {
                    None
                };
                let delta = if has_delta.is_some() {
                    Some(parse_delta(delta_decoders.get("event_t").unwrap(), &mut br))
                } else {
                    None
                };
                let has_fire_time = br.read_1_bit();
                let fire_time = if has_fire_time {
                    Some(br.read_n_bit(16).to_owned())
                } else {
                    None
                };

                EventS {
                    event_index,
                    has_packet_index,
                    packet_index,
                    has_delta,
                    delta,
                    has_fire_time,
                    fire_time,
                }
            })
            .collect();

        let (i, _) = take(br.get_consumed_bytes())(i)?;

        Ok((
            i,
            SvcEvent {
                event_count,
                events,
            },
        ))
    }

    fn write(i: SvcEvent) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        todo!();

        writer.data
    }
}
