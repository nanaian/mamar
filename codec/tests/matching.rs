use std::{path::Path, fs::File, io::prelude::*, io::Cursor, rc::Rc};
use codec::{bgm::*, sbn::*};

use simple_logger::SimpleLogger;

/// Each test generated by this macro tests for the following property:
///
///     encode(decode(bin)) == bin
///
/// That is, decoding and re-encoding a valid song with no changes must equal the original input. We call this
/// **matching**, and it's required for the [decompilation project](https://github.com/ethteck/papermario).
/// It is also helpful as a generic test suite for any inconsistencies between the `de` and `en` modules.
macro_rules! test_matching {
    ($song:ident) => {
        #[allow(non_snake_case)]
        #[test]
        fn $song() {
            let _ = SimpleLogger::new().init();

            let bin_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("bin");

            // Decode the song
            log::info!("decoding...");
            let original = include_bytes!(concat!("bin/", stringify!($song), ".bin"));
            let bgm = Bgm::decode(&mut Cursor::new(original)).expect("decode error");

            // Encode the Bgm
            log::info!("encoding...");
            let mut encoded = Cursor::new(Vec::new());
            bgm.encode(&mut encoded).unwrap();
            let encoded = encoded.into_inner();

            // Check the output matches
            if encoded != original {
                log::error!("non-matching!! D:");

                println!("original tracks allocation:");
                for (seg_no, seg) in bgm.segments.iter().enumerate() {
                    if let Some(seg) = seg {
                        for (subseg_no, subseg) in seg.iter().enumerate() {
                            match subseg {
                                Subsegment::Tracks { flags, tracks } => println!("    {}.{} flags={:#X} -> tracks @ {:#X}", seg_no, subseg_no, flags, tracks.decoded_pos.unwrap()),
                                Subsegment::Unknown { flags, data } => println!("    {}.{} flags={:#X} ??? {:?}", seg_no, subseg_no, flags, data),
                            }
                        }
                    }
                }

                let bgm = Bgm::decode(&mut Cursor::new(encoded.clone())).expect("re-decode error");
                println!("non-matching tracks allocation:");
                for (seg_no, seg) in bgm.segments.iter().enumerate() {
                    if let Some(seg) = seg {
                        for (subseg_no, subseg) in seg.iter().enumerate() {
                            match subseg {
                                Subsegment::Tracks { flags, tracks } => println!("    {}.{} flags={:#X} -> tracks @ {:#X}", seg_no, subseg_no, flags, tracks.decoded_pos.unwrap()),
                                Subsegment::Unknown { flags, data } => println!("    {}.{} flags={:#X} ??? {:?}", seg_no, subseg_no, flags, data),
                            }
                        }
                    }
                }

                // Output `encoded` to a file for debugging...
                let nonmatching_bin = concat!(stringify!($song), ".nonmatching.bin");
                let mut out = File::create(bin_dir.join(nonmatching_bin)).expect("write nonmatching.bin");
                out.write_all(&encoded).unwrap();

                // ...and fail the test.
                panic!("Re-encoded song did not match original. Wrote non-matching output to tests/bin/{}", nonmatching_bin);
            }
        }
    };
}

/// Each test generated by this macro tests for the following property:
///
///     decode(encode(decode(bin))) == decode(bin)
///
/// Songs that use lossy_match! instead of matches! have garbage data with no pointers to it at the end of the file.
macro_rules! test_lossy_match {
    ($song:ident) => {
        #[allow(non_snake_case)]
        #[test]
        fn $song() {
            let _ = SimpleLogger::new().init();

            // Decode the song
            let original = include_bytes!(concat!("bin/", stringify!($song), ".bin"));
            let bgm = Bgm::decode(&mut Cursor::new(original)).expect("decode error");

            // Encode the Bgm
            let mut encoded = Cursor::new(Vec::new());
            bgm.encode(&mut encoded).expect("encode error");

            // Re-decode the encoded Bgm
            let new_bgm = Bgm::decode(&mut encoded).expect("re-decode error");

            // Check the structs match (this doesn't work because pointers are compared by address)
            //assert_eq!(bgm, new_bgm);

            // Encode the new Bgm
            let mut new_encoded = Cursor::new(Vec::new());
            new_bgm.encode(&mut new_encoded).expect("re-encode error");

            if new_encoded != encoded {
                panic!();
            }
        }
    };
}

test_lossy_match!(Battle_Fanfare_02);
test_matching!(Hey_You_03);
test_matching!(The_Goomba_King_s_Decree_07);
test_matching!(Attack_of_the_Koopa_Bros_08);
test_lossy_match!(Trojan_Bowser_09);
test_matching!(Chomp_Attack_0A);
test_matching!(Ghost_Gulping_0B);
test_matching!(Keeping_Pace_0C);
test_matching!(Go_Mario_Go_0D);
test_matching!(Huffin_and_Puffin_0E);
test_matching!(Freeze_0F);
test_matching!(Winning_a_Battle_8B);
test_matching!(Winning_a_Battle_and_Level_Up_8E);
test_matching!(Jr_Troopa_Battle_04);
test_matching!(Final_Bowser_Battle_interlude_05);
test_matching!(Master_Battle_2C);
test_matching!(Game_Over_87);
test_matching!(Resting_at_the_Toad_House_88);
test_lossy_match!(Running_around_the_Heart_Pillar_in_Ch1_84);
test_matching!(Tutankoopa_s_Warning_45);
test_matching!(Kammy_Koopa_s_Theme_46);
test_matching!(Jr_Troopa_s_Theme_47);
test_matching!(Goomba_King_s_Theme_50);
test_matching!(Koopa_Bros_Defeated_51);
test_matching!(Koopa_Bros_Theme_52);
test_matching!(Tutankoopa_s_Warning_2_53);
test_lossy_match!(Tutankoopa_s_Theme_54);
test_matching!(Tubba_Blubba_s_Theme_55);
test_matching!(General_Guy_s_Theme_56);
test_matching!(Lava_Piranha_s_Theme_57);
test_matching!(Huff_N_Puff_s_Theme_58);
test_matching!(Crystal_King_s_Theme_59);
test_matching!(Blooper_s_Theme_5A);
test_matching!(Midboss_Theme_5B);
test_matching!(Monstar_s_Theme_5C);
test_matching!(Moustafa_s_Theme_86);
test_matching!(Fuzzy_Searching_Minigame_85);
test_matching!(Phonograph_in_Mansion_44);
test_matching!(Toad_Town_00);
test_matching!(Bill_Blaster_Theme_48);
test_matching!(Monty_Mole_Theme_in_Flower_Fields_49);
test_matching!(Shy_Guys_in_Toad_Town_4A);
test_matching!(Whale_s_Problem_4C);
test_matching!(Toad_Town_Sewers_4B);
test_lossy_match!(Unused_Theme_4D);
test_matching!(Mario_s_House_Prologue_3E);
test_matching!(Peach_s_Party_3F);
test_matching!(Goomba_Village_01);
test_matching!(Pleasant_Path_11);
test_matching!(Fuzzy_s_Took_My_Shell_12);
test_matching!(Koopa_Village_13);
test_matching!(Koopa_Bros_Fortress_14);
test_matching!(Dry_Dry_Ruins_18);
test_matching!(Dry_Dry_Ruins_Mystery_19);
test_matching!(Mt_Rugged_16);
test_matching!(Dry_Dry_Desert_Oasis_17);
test_matching!(Dry_Dry_Outpost_15);
test_matching!(Forever_Forest_1A);
test_matching!(Boo_s_Mansion_1B);
test_matching!(Bow_s_Theme_1C);
test_matching!(Gusty_Gulch_Adventure_1D);
test_matching!(Tubba_Blubba_s_Castle_1E);
test_matching!(The_Castle_Crumbles_1F);
test_matching!(Shy_Guy_s_Toy_Box_20);
test_matching!(Toy_Train_Travel_21);
test_matching!(Big_Lantern_Ghost_s_Theme_22);
test_matching!(Jade_Jungle_24);
test_matching!(Deep_Jungle_25);
test_matching!(Lavalava_Island_26);
test_matching!(Search_for_the_Fearsome_5_27);
test_matching!(Raphael_the_Raven_28);
test_matching!(Hot_Times_in_Mt_Lavalava_29);
test_matching!(Escape_from_Mt_Lavalava_2A);
test_matching!(Cloudy_Climb_32);
test_matching!(Puff_Puff_Machine_33);
test_matching!(Flower_Fields_30);
test_matching!(Flower_Fields_Sunny_31);
test_matching!(Sun_s_Tower_34);
test_matching!(Sun_s_Celebration_35);
test_matching!(Shiver_City_38);
test_matching!(Detective_Mario_39);
test_matching!(Snow_Road_3A);
test_matching!(Over_Shiver_Mountain_3B);
test_matching!(Starborn_Valley_3C);
test_matching!(Sanctuary_3D);
test_matching!(Crystal_Palace_37);
test_matching!(Star_Haven_60);
test_matching!(Shooting_Star_Summit_61);
test_matching!(Legendary_Star_Ship_62);
test_matching!(Star_Sanctuary_63);
test_matching!(Bowser_s_Castle___Caves_65);
test_matching!(Bowser_s_Castle_64);
test_matching!(Star_Elevator_2B);
test_matching!(Goomba_Bros_Defeated_7E);
test_matching!(Farewell_Twink_70);
test_matching!(Peach_Cooking_71);
test_matching!(Gourmet_Guy_72);
test_matching!(Hope_on_the_Balcony_Peach_1_73);
test_matching!(Peach_s_Theme_2_74);
test_matching!(Peach_Sneaking_75);
test_matching!(Peach_Captured_76);
test_matching!(Quiz_Show_Intro_77);
test_matching!(Unconscious_Mario_78);
test_matching!(Petunia_s_Theme_89);
test_matching!(Flower_Fields_Door_appears_8A);
test_matching!(Beanstalk_7B);
test_matching!(Lakilester_s_Theme_7D);
test_matching!(The_Sun_s_Back_7F);
test_matching!(Shiver_City_in_Crisis_79);
test_matching!(Solved_Shiver_City_Mystery_7A);
test_matching!(Merlon_s_Spell_7C);
test_matching!(Bowser_s_Theme_66);
test_matching!(Train_Travel_80);
test_matching!(Whale_Trip_81);
test_matching!(Chanterelle_s_Song_8C);
test_matching!(Boo_s_Game_8D);
test_matching!(Dry_Dry_Ruins_rises_up_83);
test_matching!(End_of_Chapter_40);
test_matching!(Beginning_of_Chapter_41);
test_matching!(Hammer_and_Jump_Upgrade_42);
test_matching!(Found_Baby_Yoshi_s_4E);
test_matching!(New_Partner_JAP_96);
test_matching!(Unused_YI_Fanfare_4F);
test_matching!(Unused_YI_Fanfare_2_5D);
test_lossy_match!(Peach_s_Castle_inside_Bubble_5E);
test_matching!(Angry_Bowser_67);
test_lossy_match!(Bowser_s_Castle_explodes_5F);
test_matching!(Peach_s_Wish_68);
test_matching!(File_Select_69);
test_matching!(Title_Screen_6A);
test_matching!(Peach_s_Castle_in_Crisis_6B);
test_matching!(Mario_falls_from_Bowser_s_Castle_6C);
test_matching!(Peach_s_Arrival_6D);
test_matching!(Star_Rod_Recovered_6F);
test_matching!(Mario_s_House_94);
test_matching!(Bowser_s_Attacks_95);
test_matching!(End_Parade_1_90);
test_matching!(End_Parade_2_91);
test_matching!(The_End_6E);
test_matching!(Koopa_Radio_Station_2D);
test_matching!(The_End_Low_Frequency__2E);
test_matching!(SMW_Remix_2F);
test_matching!(New_Partner_82);

#[test]
fn shared_subsegment_tracks_ptr() {
    let _ = SimpleLogger::new().init();

    // Decode the song
    let original = include_bytes!("bin/Fuzzy_s_Took_My_Shell_12.bin");
    let bgm = Bgm::decode(&mut Cursor::new(original)).expect("decode error");

    let tracks_0_2 = if let Subsegment::Tracks { ref tracks, .. } = bgm.segments[0].as_ref().unwrap()[2] {
        tracks
    } else {
        panic!();
    };

    let tracks_1_1 = if let Subsegment::Tracks { ref tracks, .. } = bgm.segments[1].as_ref().unwrap()[1] {
        tracks
    } else {
        panic!();
    };

    // The two tracks share a pointer to their CommandSeq, so they should be decoded to share a reference
    assert!(Rc::ptr_eq(tracks_0_2, tracks_1_1));
}

#[test]
#[ignore] // TEMP: bgm::en not yet implemented
fn sbn() {
    let original = include_bytes!("bin/sbn.bin");
    let sbn = Sbn::from_bytes(original).unwrap();

    for file in &sbn.files {
        println!("file {} {} size={:#X}", file.magic().unwrap(), file.name, file.data.len());
    }

    for song in &sbn.songs {
        println!("song {} -> {}", song.bgm_file, sbn.files[song.bgm_file as usize].name);
    }

    assert!(sbn.as_bytes().unwrap() == original);
}
