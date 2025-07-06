#[derive(
    candid::CandidType,
    candid::Deserialize,
    serde::Serialize,
    // comparable::Comparable,
    // strum_macros::EnumIter,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    // ::prost::Enumeration,
)]
#[repr(i32)]
pub enum Topic {
    /// The `Unspecified` topic is used as a fallback when
    /// following. That is, if no followees are specified for a given
    /// topic, the followees for this topic are used instead.
    Unspecified = 0,
    /// A special topic by means of which a neuron can be managed by the
    /// followees for this topic (in this case, there is no fallback to
    /// 'unspecified'). Votes on this topic are not included in the
    /// voting history of the neuron (cf., `recent_ballots` in `Neuron`).
    ///
    /// For proposals on this topic, only followees on the 'neuron
    /// management' topic of the neuron that the proposals pertains to
    /// are allowed to vote.
    ///
    /// As the set of eligible voters on this topic is restricted,
    /// proposals on this topic have a *short voting period*.
    NeuronManagement = 1,
    /// All proposals that provide “real time” information about the
    /// value of ICP, as measured by an IMF SDR, which allows the NNS to
    /// convert ICP to cycles (which power computation) at a rate which
    /// keeps their real world cost constant. Votes on this topic are not
    /// included in the voting history of the neuron (cf.,
    /// `recent_ballots` in `Neuron`).
    ///
    /// Proposals on this topic have a *short voting period* due to their
    /// frequency.
    ExchangeRate = 2,
    /// All proposals that administer network economics, for example,
    /// determining what rewards should be paid to node operators.
    NetworkEconomics = 3,
    /// All proposals that administer governance, for example to freeze
    /// malicious canisters that are harming the network.
    Governance = 4,
    /// All proposals that administer node machines, including, but not
    /// limited to, upgrading or configuring the OS, upgrading or
    /// configuring the virtual machine framework and upgrading or
    /// configuring the node replica software.
    NodeAdmin = 5,
    /// All proposals that administer network participants, for example,
    /// granting and revoking DCIDs (data center identities) or NOIDs
    /// (node operator identities).
    ParticipantManagement = 6,
    /// All proposals that administer network subnets, for example
    /// creating new subnets, adding and removing subnet nodes, and
    /// splitting subnets.
    SubnetManagement = 7,
    /// All proposals to manage NNS-controlled canisters not covered by other topics (Protocol Canister
    /// Management or Service Nervous System Management).
    NetworkCanisterManagement = 8,
    /// Proposals that update KYC information for regulatory purposes,
    /// for example during the initial Genesis distribution of ICP in the
    /// form of neurons.
    Kyc = 9,
    /// Topic for proposals to reward node providers.
    NodeProviderRewards = 10,
    /// IC OS upgrade proposals
    /// -----------------------
    /// ICP runs on a distributed network of nodes grouped into subnets. Each node runs a stack of
    /// operating systems, including HostOS (runs on bare metal) and GuestOS (runs inside HostOS;
    /// contains, e.g., the ICP replica process). HostOS and GuestOS are distributed via separate disk
    /// images. The umbrella term IC OS refers to the whole stack.
    ///
    /// The IC OS upgrade process involves two phases, where the first phase is the election of a new
    /// IC OS version and the second phase is the deployment of a previously elected IC OS version on
    /// all nodes of a subnet or on some number of nodes (including nodes comprising subnets and
    /// unassigned nodes).
    ///
    /// A special case is for API boundary nodes, special nodes that route API requests to a replica
    /// of the right subnet. API boundary nodes run a different process than the replica, but their
    /// executable is distributed via the same disk image as GuestOS. Therefore, electing a new GuestOS
    /// version also results in a new version of boundary node software being elected.
    ///
    /// Proposals handling the deployment of IC OS to some nodes. It is possible to deploy only
    /// the versions of IC OS that are in the set of elected IC OS versions.
    IcOsVersionDeployment = 12,
    /// Proposals for changing the set of elected IC OS versions.
    IcOsVersionElection = 13,
    /// Proposals related to SNS and Community Fund.
    SnsAndCommunityFund = 14,
    /// Proposals related to the management of API Boundary Nodes
    ApiBoundaryNodeManagement = 15,
    /// Proposals related to subnet rental.
    SubnetRental = 16,
    /// All proposals to manage protocol canisters, which are considered part of the ICP protocol and
    /// are essential for its proper functioning.
    ProtocolCanisterManagement = 17,
    /// All proposals to manage the canisters of service nervous systems (SNS), including upgrading
    /// relevant canisters and managing SNS framework canister WASMs through SNS-W.
    ServiceNervousSystemManagement = 18,
}
