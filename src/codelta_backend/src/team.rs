use icrc_ledger_types::icrc1::account::DEFAULT_SUBACCOUNT;

use crate::{
    member::{Member, MEMBERS},
    topic::Topic,
};

pub struct Team<'a> {
    /// Account derived from canister - canister controlled
    pub sub_account: &'a [u8; 32],

    pub topic: Topic,

    pub members: &'a [&'a Member<'a>],
}

/// Teams are split by topic which they review
pub const TEAMS: &[Team] = &[
    // OG - Subnet Management
    Team {
        sub_account: DEFAULT_SUBACCOUNT, // Zeros
        topic: Topic::SubnetManagement,
        members: &[
            &MEMBERS.aligatorr89,
            &MEMBERS.lorimer,
            &MEMBERS.malith_hatananchchige,
        ],
    },
    // OG - API BoundaryNode Management
    Team {
        sub_account: &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1,
        ], // Zeros with 1 in the end
        topic: Topic::ApiBoundaryNodeManagement,
        members: &[
            &MEMBERS.aligatorr89,
            &MEMBERS.lorimer,
            &MEMBERS.malith_hatananchchige,
        ],
    },
    // Protocol Canister Management
    Team {
        sub_account: &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 2,
        ], // Zeros with 2 in the end
        topic: Topic::ProtocolCanisterManagement,
        members: &[], // Fill members!
    },
    // Ic Os VersionE lection
    Team {
        sub_account: &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 3,
        ], // Zeros with 3 in the end
        topic: Topic::IcOsVersionElection,
        members: &[], // Fill members!
    },
    // Node Admin
    Team {
        sub_account: &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 4,
        ], // Zeros with 4 in the end
        topic: Topic::NodeAdmin,
        members: &[], // Fill members!
    },
    // Participant Management
    Team {
        sub_account: &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 5,
        ], // Zeros with 5 in the end
        topic: Topic::ParticipantManagement,
        members: &[], // Fill members!
    },
];

pub fn get_team_by_topic(topic: Topic) -> &'static Team<'static> {
    return TEAMS.iter().find(|t| t.topic == topic).unwrap_or_else(|| {
        ic_cdk::println!("team not found for topic {:?}", topic);
        panic!("team not found for topic");
    });
}
