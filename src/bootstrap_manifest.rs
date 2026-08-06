//! Authority-seated identities for the strict meta cloud Interface.
//!
//! These opaque identities and canonical-order values are minted state. None
//! is derived from spelling, source position, or Rust representation.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthoritySeat {
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl AuthoritySeat {
    pub const fn new(spelling: &'static str, local: u16, canonical: u64) -> Self {
        Self {
            spelling,
            local,
            canonical,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclarationSeat {
    pub owner_local: Option<u16>,
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}

impl DeclarationSeat {
    pub const fn new(
        owner_local: Option<u16>,
        spelling: &'static str,
        local: u16,
        canonical: u64,
    ) -> Self {
        Self {
            owner_local,
            spelling,
            local,
            canonical,
        }
    }
}

pub const AUTHORITY_IDENTITY: [u8; 32] = [
    10, 106, 121, 76, 3, 200, 227, 139, 166, 74, 157, 125, 94, 21, 215, 175, 21, 147, 121, 183,
    176, 17, 232, 12, 201, 57, 170, 180, 148, 142, 204, 185,
];
pub const AUTHORITY_REVISION: u64 = 1;
pub const GRAMMAR_DOCUMENT_LOCAL: u16 = 11213;
pub const GRAMMAR_SYNTAX_LOCAL: u16 = 2900;

pub const INTERFACE_SEAT: AuthoritySeat =
    AuthoritySeat::new("Interface", 19572, 0x643ac7ddaee60480);
pub const NEXUS_SEAT: AuthoritySeat = AuthoritySeat::new("Nexus", 17804, 0x78cc3576603db1fe);
pub const SEMA_SEAT: AuthoritySeat = AuthoritySeat::new("Sema", 34297, 0x415c054e0d5bcf60);
pub const INPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Input", 22647, 0x203f476f808f901d);
pub const OUTPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Output", 3734, 0x85983599620a5927);
pub const REFUSAL_SEAT: AuthoritySeat = AuthoritySeat::new("Refusal", 18397, 0xe329e0aede92df92);
pub const STRING_SEAT: AuthoritySeat = AuthoritySeat::new("String", 33026, 0x6efcebb5291baacd);
pub const INTEGER_SEAT: AuthoritySeat = AuthoritySeat::new("Integer", 6525, 0x962be719e9bd6957);
pub const BOOLEAN_SEAT: AuthoritySeat = AuthoritySeat::new("Boolean", 28590, 0x1dfb013641f72a83);
pub const UNIT_SEAT: AuthoritySeat = AuthoritySeat::new("Unit", 13857, 0xb9cc904f40367d24);
pub const VECTOR_SEAT: AuthoritySeat = AuthoritySeat::new("Vector", 46717, 0x21e2f47e1db2b3f3);
pub const OPTION_SEAT: AuthoritySeat = AuthoritySeat::new("Option", 24397, 0xd1cb8ed3672acb7c);
pub const MAP_SEAT: AuthoritySeat = AuthoritySeat::new("Map", 19255, 0x7f37653381b290c8);
pub const RESULT_SEAT: AuthoritySeat = AuthoritySeat::new("Result", 49400, 0x028b33056d3655f3);
pub const STREAM_SEAT: AuthoritySeat = AuthoritySeat::new("Stream", 29110, 0x6e4ded7af5064cfd);
pub const STREAMIDENTITY_SEAT: AuthoritySeat =
    AuthoritySeat::new("StreamIdentity", 60788, 0x92c3708e7af4bae4);

pub const RUST_VOCABULARY_LOCALS: [u16; 10] = [
    3024, 31299, 40691, 32486, 36286, 29329, 5320, 8264, 11991, 12150,
];

pub const CREDENTIAL_HANDLE_LOCAL: u16 = 6912;
pub const CAPABILITY_DIRECTIVE_LOCAL: u16 = 30117;
pub const HOST_INTENT_LOCAL: u16 = 60862;
pub const REJECTION_REASON_LOCAL: u16 = 806;

pub const DECLARATION_SEATS: &[DeclarationSeat] = &[
    DeclarationSeat::new(
        None,
        "CredentialHandle",
        CREDENTIAL_HANDLE_LOCAL,
        0x6f2c4d1c67247096,
    ),
    DeclarationSeat::new(
        None,
        "CapabilityDirective",
        CAPABILITY_DIRECTIVE_LOCAL,
        0xdacb49b69cacd69e,
    ),
    DeclarationSeat::new(
        Some(CAPABILITY_DIRECTIVE_LOCAL),
        "Enable",
        44675,
        0x1fd7104b1816f7d3,
    ),
    DeclarationSeat::new(
        Some(CAPABILITY_DIRECTIVE_LOCAL),
        "Disable",
        30263,
        0x9e9d914f734783b1,
    ),
    DeclarationSeat::new(None, "HostIntent", HOST_INTENT_LOCAL, 0x1fa8b34ba76a16f9),
    DeclarationSeat::new(Some(HOST_INTENT_LOCAL), "Create", 22559, 0x67eb34bf4fb56318),
    DeclarationSeat::new(
        Some(HOST_INTENT_LOCAL),
        "Destroy",
        52148,
        0xacc0135261701e2a,
    ),
    DeclarationSeat::new(
        None,
        "RejectionReason",
        REJECTION_REASON_LOCAL,
        0x843504d5f2e21e7d,
    ),
    DeclarationSeat::new(
        Some(REJECTION_REASON_LOCAL),
        "CredentialHandleUnknown",
        53924,
        0x31ea36f50b645110,
    ),
    DeclarationSeat::new(
        Some(REJECTION_REASON_LOCAL),
        "ProviderNotConfigured",
        60113,
        0x567e9fda0327ecaf,
    ),
    DeclarationSeat::new(
        Some(REJECTION_REASON_LOCAL),
        "AccountUnknown",
        27278,
        0xe3102d5454dad015,
    ),
    DeclarationSeat::new(
        Some(REJECTION_REASON_LOCAL),
        "PlanUnknown",
        23482,
        0xd4987446371df6e6,
    ),
    DeclarationSeat::new(
        Some(REJECTION_REASON_LOCAL),
        "PlanNotApproved",
        9964,
        0xa87ad3ad932c89d6,
    ),
    DeclarationSeat::new(
        Some(REJECTION_REASON_LOCAL),
        "PlanGenerationFailed",
        35105,
        0xca7a2a54e9c647ac,
    ),
    DeclarationSeat::new(
        Some(REJECTION_REASON_LOCAL),
        "CapabilityUnauthorized",
        50668,
        0xcc02f33faea47537,
    ),
    DeclarationSeat::new(None, "ServerType", 21001, 0xa523a30d6eac7b65),
    DeclarationSeat::new(None, "ImageName", 4475, 0x250110ed267b61ce),
    DeclarationSeat::new(None, "SshKeyName", 2850, 0x08de4e92f0d037f5),
];
