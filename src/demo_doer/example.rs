use std::{collections::HashMap, str::from_utf8};

use demosuperimpose_goldsrc::netmsg_doer::{
    client_data::ClientData,
    delta_description::DeltaDescription,
    parse_netmsg,
    print::Print,
    send_extra_info::SendExtraInfo,
    server_info::{self, ServerInfo},
    time::Time,
    utils::{get_initial_delta, BitReader},
    NetMsgDoer,
};
use hldemo::{Demo, FrameData};

use super::*;

pub fn example(demo: &mut Demo) {
    // for entry in &mut demo.directory.entries {
    //     for frame in &mut entry.frames {
    //         if let FrameData::NetMsg((_, data)) = &mut frame.data {
    //             let msg_type = data.msg[0];
    //             if msg_type == 7 {
    //                 let (rest, time) = Time::parse(&data.msg[1..]).unwrap();
    //                 // println!("{} {:#?}", time.time, rest[0]);
    //                 // let (res, client_data) = ClientData::parse(&rest[1..]).unwrap();
    //                 println!("{:?}", rest);
    //             }
    //             // if vec![14u8, 15, 3, 21, 22, 40, 41].contains(&msg_type) {
    //             //     println!("{:?}", data.msg);
    //             // }
    //             // println!("{}", msg_type);
    //         }
    //     }
    // }

    // let entry = &demo.directory.entries[1];
    // for i in 0..10 {
    //     let frame = &entry.frames[i];
    //     if let FrameData::NetMsg((_, data)) = &frame.data {
    //         let msg_type = data.msg[0];
    //         if msg_type == 7 {
    //             println!("{}", i);
    //             let (rest, time) = Time::parse(&data.msg[1..]).unwrap();
    //             // let what: BitVec<u8, Lsb0> = BitVec::from_slice(&rest[1..]);
    //             println!("{:?}", rest);
    //             // println!("{:?}", rest);
    //             // println!("{} {:#?}", time.time, rest[0]);
    //             let (rest, client_data) = ClientData::parse(&rest[1..]).unwrap();
    //             println!("{:?}", client_data);
    //             println!("{:?}", rest);
    //         }
    //         // if vec![14u8, 15, 3, 21, 22, 40, 41].contains(&msg_type) {
    //         //     println!("{:?}", data.msg);
    //         // }
    //         // println!("{}", msg_type);
    //     }
    // }

    // let entry = &demo.directory.entries[0];
    // println!("{}", );
    // let data = &entry.frames[0].data;
    // if let FrameData::NetMsg((_, data)) = &data {
    //     // println!("{:?}", data.msg);
    //     let mut delta_decoders = get_initial_delta();

    //     let (i, what) = parse_netmsg(data.msg, &mut delta_decoders).unwrap();
    //     // println!("{:?}", delta_decoders);

    //     println!("{:?}", i);

    //     // println!("{}", data.msg[0]);
    //     // let (rest, print) = Print::parse(&data.msg[1..], &mut delta_decoders).unwrap();
    //     // println!("{}", rest[0]);
    //     // let (rest, server_info) = ServerInfo::parse(&rest[1..], &mut delta_decoders).unwrap();
    //     // println!("{}", rest[0]);
    //     // let (rest, send_extra_info) =
    //     //     SendExtraInfo::parse(&rest[1..], &mut delta_decoders).unwrap();
    //     // println!("{}", rest[0]);
    //     // let (rest, delta1) = DeltaDescription::parse(&rest[1..], &mut delta_decoders).unwrap();
    //     // delta_decoders.insert(from_utf8(delta1.name).unwrap().to_string(), delta1.fields);
    //     // println!("{}", rest[0]);
    //     // let (rest, delta2) = DeltaDescription::parse(&rest[1..], &mut delta_decoders).unwrap();
    //     // println!("{}", rest[0]);
    //     // let (rest, delta3) = DeltaDescription::parse(&rest[1..], &mut delta_decoders).unwrap();
    //     // println!("{}", rest[0]);
    // }
    let mut delta_decoders = get_initial_delta();
    let mut custom_messages = HashMap::<u8, SvcNewUserMsg>::new();

    for (i, entry) in demo.directory.entries.iter().enumerate() {
        for (j, frame) in entry.frames.iter().enumerate() {
            if let FrameData::NetMsg((_, data)) = &frame.data {
                println!("{} {}", i, j);
                parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages);
            }
        }
    }

    // let mut delta_decoders = get_initial_delta();

    // c = lambda x: print(''.join(list(map(lambda y: chr(int(y.strip())), x.split(',')))))

    // let entry = &demo.directory.entries[1];
    // let data = &entry.frames[33].data;
    // if let FrameData::NetMsg((_, data)) = &data {
    //     parse_netmsg(data.msg, &mut delta_decoders, &mut custom_messages);
    // }
}
