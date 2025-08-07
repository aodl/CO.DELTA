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
        sub_account: DEFAULT_SUBACCOUNT, // Zeros - shares account with API BN
        topic: Topic::SubnetManagement,
        members: &[
            &MEMBERS.aligatorr89,
            &MEMBERS.lorimer,
            &MEMBERS.malith_hatananchchige,
        ],
    },
    // OG - API BoundaryNode Management
    Team {
        sub_account: DEFAULT_SUBACCOUNT, // Zeros - shares account with SM
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
            0, 0, 1,
        ], // Zeros with 1 in the end
        topic: Topic::ProtocolCanisterManagement,
        members: &[
            &MEMBERS.remcodes,
            &MEMBERS.gabriel,
            &MEMBERS.gautier,
        ],
    },
    // Ic Os VersionE lection
    Team {
        sub_account: &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 2,
        ], // Zeros with 2 in the end
        topic: Topic::IcOsVersionElection,
        members: &[
            &MEMBERS.aligatorr89,
            &MEMBERS.zane,
            &MEMBERS.gautier,
        ],
    },
    // Node Admin
    Team {
        sub_account: &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 3,
        ], // Zeros with 3 in the end - shares account with ParticipantManagement
        topic: Topic::NodeAdmin,
        members: &[
            //&MEMBERS.thyassa, uncomment if/when thyassa would like to start receiving a share of the distribution for her work
            &MEMBERS.lorimer,
            &MEMBERS.malith_hatananchchige,
            &MEMBERS.gabriel,
        ],
    },
    // Participant Management
    Team {
        sub_account: &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 3,
        ], // Zeros with 3 in the end - shares account with NodeAdmin
        topic: Topic::ParticipantManagement,
        members: &[
            //&MEMBERS.thyassa, uncomment if/when thyassa would like to start receiving a share of the distribution for her work
            &MEMBERS.lorimer,
            &MEMBERS.malith_hatananchchige,
            &MEMBERS.gabriel,
        ],
    },
    // Application Canister Management
    Team {
        sub_account: &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 4,
        ], // Zeros with 4 in the end
        topic: Topic::NetworkCanisterManagement,
        members: &[
            &MEMBERS.gautier,
            &MEMBERS.malith_hatananchchige,
            &MEMBERS.gekctek,
        ],
    },
    // Service Nervous System Management
    Team {
        sub_account: &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 5,
        ], // Zeros with 5 in the end
        topic: Topic::ServiceNervousSystemManagement,
        members: &[
            &MEMBERS.remcodes,
            &MEMBERS.gabriel,
            &MEMBERS.gautier,
        ],
    },
    // SNS & Neurons’ Fund
    Team {
        sub_account: &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 6,
        ], // Zeros with 6 in the end
        topic: Topic::SnsAndCommunityFund,
        members: &[
            //&MEMBERS.thyassa, uncomment if/when thyassa would like to start receiving a share of the distribution for her work
            &MEMBERS.remcodes,
            &MEMBERS.lorimer,
            &MEMBERS.gabriel,
        ],
    },
];

pub fn get_team_by_topic(topic: Topic) -> &'static Team<'static> {
    return TEAMS.iter().find(|t| t.topic == topic).unwrap_or_else(|| {
        ic_cdk::println!("team not found for topic {:?}", topic);
        panic!("team not found for topic");
    });
}
