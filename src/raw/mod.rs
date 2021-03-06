pub mod convert;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct Hangouts {
    pub conversations: Vec<Conversation>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct Conversation {
    #[cfg_attr(feature = "serde-impl", serde(rename = "conversation"))]
    pub header: ConversationHeader,
    pub events: Vec<Event>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct ConversationHeader {
    pub conversation_id: ConversationId,
    #[cfg_attr(feature = "serde-impl", serde(rename = "conversation"))]
    pub details: ConversationDetails,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct ConversationDetails {
    pub id: ConversationId,
    #[cfg_attr(feature = "serde-impl", serde(rename = "type"))]
    pub typ: ConversationType,
    // Set for type="GROUP" only.
    pub name: Option<String>,
    pub self_conversation_state: SelfConversationState,
    pub read_state: Vec<ReadState>,
    pub has_active_hangout: bool,
    pub otr_status: String,
    pub otr_toggle: String,
    pub current_participant: Vec<ParticipantId>,
    pub participant_data: Vec<ParticipantData>,
    pub fork_on_external_invite: bool,
    pub network_type: Vec<String>,
    pub force_history_state: String,
    pub group_link_sharing_status: LinkSharingStatus,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct ConversationId {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub enum ConversationType {
    #[cfg_attr(feature = "serde-impl", serde(rename = "GROUP"))]
    Group,
    #[cfg_attr(feature = "serde-impl", serde(rename = "STICKY_ONE_TO_ONE"))]
    OneToOne,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum LinkSharingStatus {
    #[cfg_attr(feature = "serde-impl", serde(rename = "LINK_SHARING_OFF"))]
    Off,
    #[cfg_attr(feature = "serde-impl", serde(rename = "LINK_SHARING_ON"))]
    On,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    // serde(deny_unknown_fields)
)]
pub struct SelfConversationState {
    pub self_read_state: ReadState,
    pub status: ConversationStatus,
    pub notification_level: NotificationLevel,
    pub view: Vec<View>,
    pub inviter_id: ParticipantId,
    pub invite_timestamp: String,
    pub invitation_display_type: Option<String>,
    pub invite_affinity: Option<InvitationAffinity>,
    pub sort_timestamp: String,
    pub active_timestamp: Option<String>,
    // TODO:
    // pub delivery_medium_option: Option<serde_json::Value>,
    pub is_guest: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub enum View {
    #[cfg_attr(feature = "serde-impl", serde(rename = "INBOX_VIEW"))]
    Inbox,
    #[cfg_attr(feature = "serde-impl", serde(rename = "ARCHIVED_VIEW"))]
    Archived,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct ReadState {
    pub participant_id: ParticipantId,
    pub latest_read_timestamp: String,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub enum ConversationStatus {
    #[cfg_attr(feature = "serde-impl", serde(rename = "ACTIVE"))]
    Active,
    #[cfg_attr(feature = "serde-impl", serde(rename = "INVITED"))]
    Invited,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum InvitationAffinity {
    #[cfg_attr(feature = "serde-impl", serde(rename = "LOW"))]
    Low,
    #[cfg_attr(feature = "serde-impl", serde(rename = "HIGH"))]
    High,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct ParticipantId {
    pub gaia_id: String,
    pub chat_id: String,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    // serde(deny_unknown_fields)
)]
pub struct ParticipantData {
    pub id: ParticipantId,
    pub fallback_name: Option<String>,
    pub invitation_status: Option<InvitationStatus>,
    pub participant_type: Option<ParticipantType>,
    pub new_invitation_status: Option<InvitationStatus>,
    pub in_different_customer_as_requester: Option<bool>,
    pub domain_id: Option<String>,
    // TODO:
    // pub phone_number: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum ParticipantType {
    #[cfg_attr(feature = "serde-impl", serde(rename = "GAIA"))]
    Gaia,
    #[cfg_attr(feature = "serde-impl", serde(rename = "OFF_NETWORK_PHONE"))]
    OffNetworkPhone,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum InvitationStatus {
    #[cfg_attr(feature = "serde-impl", serde(rename = "PENDING_INVITATION"))]
    Pending,
    #[cfg_attr(feature = "serde-impl", serde(rename = "ACCEPTED_INVITATION"))]
    Accepted,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    // serde(deny_unknown_fields)
)]
pub struct Event {
    #[cfg_attr(feature = "serde-impl", serde(flatten))]
    pub header: EventHeader,
    #[cfg_attr(feature = "serde-impl", serde(flatten))]
    pub data: EventData,
    pub event_type: EventType,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum EventType {
    #[cfg_attr(feature = "serde-impl", serde(rename = "REGULAR_CHAT_MESSAGE"))]
    RegularChatMessage,
    #[cfg_attr(feature = "serde-impl", serde(rename = "SMS"))]
    Sms,
    #[cfg_attr(feature = "serde-impl", serde(rename = "HANGOUT_EVENT"))]
    HangoutEvent,
    #[cfg_attr(feature = "serde-impl", serde(rename = "ADD_USER"))]
    AddUser,
    #[cfg_attr(feature = "serde-impl", serde(rename = "REMOVE_USER"))]
    RemoveUser,
    #[cfg_attr(feature = "serde-impl", serde(rename = "RENAME_CONVERSATION"))]
    RenameConversation,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    // serde(deny_unknown_fields)
)]
pub struct EventHeader {
    pub conversation_id: ConversationId,
    pub sender_id: ParticipantId,
    pub timestamp: String,
    pub self_event_state: SelfEventState,
    pub event_id: String,
    pub advances_sort_timestamp: bool,
    pub event_otr: EventOtr,
    // TODO:
    // pub delivery_medium: serde_json::Value,
    pub event_version: String,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum EventOtr {
    #[cfg_attr(feature = "serde-impl", serde(rename = "ON_THE_RECORD"))]
    OnTheRecord,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct SelfEventState {
    pub user_id: ParticipantId,
    pub client_generated_id: Option<String>,
    pub notification_level: Option<NotificationLevel>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub enum NotificationLevel {
    #[cfg_attr(feature = "serde-impl", serde(rename = "QUIET"))]
    Quiet,
    #[cfg_attr(feature = "serde-impl", serde(rename = "RING"))]
    Ring,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    // serde(deny_unknown_fields)
)]
pub enum EventData {
    #[cfg_attr(feature = "serde-impl", serde(rename = "chat_message"))]
    ChatMessage {
        message_content: ChatSegments,
        annotation: Option<Vec<Annotation>>,
    },

    #[cfg_attr(feature = "serde-impl", serde(rename = "hangout_event"))]
    HangoutEvent {
        #[cfg_attr(feature = "serde-impl", serde(flatten))]
        data: HangoutEvent,
        media_type: Option<MediaType>,
        participant_id: Vec<ParticipantId>,
    },

    #[cfg_attr(feature = "serde-impl", serde(rename = "membership_change"))]
    MembershipChange {
        #[cfg_attr(feature = "serde-impl", serde(rename = "type"))]
        typ: MembershipChangeType,
        participant_id: Vec<ParticipantId>,
    },

    #[cfg_attr(feature = "serde-impl", serde(rename = "conversation_rename"))]
    ConversationRename { new_name: String, old_name: String },
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum MembershipChangeType {
    #[cfg_attr(feature = "serde-impl", serde(rename = "JOIN"))]
    Join,
    #[cfg_attr(feature = "serde-impl", serde(rename = "LEAVE"))]
    Leave,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum MediaType {
    #[cfg_attr(feature = "serde-impl", serde(rename = "AUDIO_ONLY"))]
    AudioOnly,
    #[cfg_attr(feature = "serde-impl", serde(rename = "VIDEO"))]
    Video,
    #[cfg_attr(feature = "serde-impl", serde(rename = "AUDIO_VIDEO"))]
    AudioVideo,
    #[cfg_attr(feature = "serde-impl", serde(rename = "PHOTO"))]
    Photo,
    #[cfg_attr(feature = "serde-impl", serde(rename = "ANIMATED_PHOTO"))]
    AnimatedPhoto,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct ChatSegments {
    #[cfg_attr(feature = "serde-impl", serde(rename = "segment", default))]
    pub segments: Vec<ChatSegment>,
    #[cfg_attr(feature = "serde-impl", serde(rename = "attachment", default))]
    pub attachments: Vec<AttachmentSegment>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(tag = "type"),
    serde(deny_unknown_fields)
)]
pub enum ChatSegment {
    #[cfg_attr(feature = "serde-impl", serde(rename = "TEXT"))]
    Text {
        text: String,
        #[cfg_attr(feature = "serde-impl", serde(default))]
        formatting: Formatting,
    },

    #[cfg_attr(feature = "serde-impl", serde(rename = "LINK"))]
    Link {
        text: String,
        link_data: LinkData,
        #[cfg_attr(feature = "serde-impl", serde(default))]
        formatting: Formatting,
    },

    #[cfg_attr(feature = "serde-impl", serde(rename = "LINE_BREAK"))]
    LineBreak {
        text: Option<String>,
        #[cfg_attr(feature = "serde-impl", serde(default))]
        formatting: Formatting,
    },
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct Annotation {
    #[cfg_attr(feature = "serde-impl", serde(rename = "type"))]
    pub typ: i32,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct LinkData {
    pub link_target: String,
    pub display_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct Formatting {
    #[cfg_attr(feature = "serde-impl", serde(default))]
    pub bold: bool,
    #[cfg_attr(feature = "serde-impl", serde(default))]
    pub italics: bool,
    #[cfg_attr(feature = "serde-impl", serde(default))]
    pub strikethrough: bool,
    #[cfg_attr(feature = "serde-impl", serde(default))]
    pub underline: bool,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct AttachmentSegment {
    pub embed_item: EmbedItem,
    pub id: String,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct EmbedItem {
    pub id: Option<String>,
    pub plus_photo: Option<PlusPhoto>,
    pub place_v2: Option<PlaceV2>,
    pub thing_v2: Option<ThingV2>,
    #[cfg_attr(feature = "serde-impl", serde(rename = "type"))]
    pub types: Vec<EmbedItemType>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum EmbedItemType {
    #[cfg_attr(feature = "serde-impl", serde(rename = "PLUS_PHOTO"))]
    PlusPhoto,
    #[cfg_attr(feature = "serde-impl", serde(rename = "PLACE_V2"))]
    PlaceV2,
    #[cfg_attr(feature = "serde-impl", serde(rename = "THING"))]
    Thing,
    #[cfg_attr(feature = "serde-impl", serde(rename = "THING_V2"))]
    ThingV2,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct PlusPhoto {
    pub album_id: String,
    pub media_type: MediaType,
    pub original_content_url: String,
    pub owner_obfuscated_id: String,
    pub photo_id: String,
    pub stream_id: Vec<String>,
    pub thumbnail: Thumbnail,
    pub url: String,
    pub download_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct Thumbnail {
    pub height_px: u64,
    pub width_px: u64,
    pub image_url: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct PlaceV2 {
    pub url: String,
    pub name: Option<String>,
    pub address: Address,
    pub geo: Geo,
    pub representative_image: RepresentativeImage,
    pub place_id: Option<String>,
    pub cluster_id: Option<String>,
    pub reference_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct Address {
    #[cfg_attr(feature = "serde-impl", serde(rename = "type", default))]
    pub types: Vec<AddressType>,
    pub postal_address_v2: PostalAddressV2,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum AddressType {
    #[cfg_attr(feature = "serde-impl", serde(rename = "POSTAL_ADDRESS_V2"))]
    PostalAddressV2,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct PostalAddressV2 {
    pub name: Option<String>,
    pub street_address: Option<String>,
    pub address_locality: Option<String>,
    pub address_region: Option<String>,
    pub address_country: Option<String>,
    pub postal_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct Geo {
    #[cfg_attr(feature = "serde-impl", serde(rename = "type", default))]
    pub types: Vec<GeoType>,
    pub geo_coordinates_v2: GeoCoordinatesV2,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub enum GeoType {
    #[cfg_attr(feature = "serde-impl", serde(rename = "GEO_COORDINATES_V2"))]
    GeoCoordinatesV2,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct GeoCoordinatesV2 {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct RepresentativeImage {
    #[cfg_attr(feature = "serde-impl", serde(rename = "type"))]
    pub types: Vec<RepresentativeImageType>,
    pub id: String,
    pub image_object_v2: ImageObjectV2,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde-impl", derive(serde::Deserialize, serde::Serialize))]
pub enum RepresentativeImageType {
    #[cfg_attr(feature = "serde-impl", serde(rename = "IMAGE_OBJECT_V2"))]
    ImageObjectV2,
    #[cfg_attr(feature = "serde-impl", serde(rename = "MEDIA_OBJECT_V2"))]
    MediaObjectV2,
    #[cfg_attr(feature = "serde-impl", serde(rename = "CREATIVE_WORK_V2"))]
    CreativeWorkV2,
    #[cfg_attr(feature = "serde-impl", serde(rename = "THING_V2"))]
    ThingV2,
    #[cfg_attr(feature = "serde-impl", serde(rename = "THING"))]
    Thing,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct ImageObjectV2 {
    pub url: String,
    pub width: Option<String>,
    pub height: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields)
)]
pub struct ThingV2 {
    pub url: String,
    pub name: Option<String>,
    pub representative_image: RepresentativeImage,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde-impl",
    derive(serde::Deserialize, serde::Serialize),
    serde(tag = "event_type"),
    serde(deny_unknown_fields)
)]
pub enum HangoutEvent {
    #[cfg_attr(feature = "serde-impl", serde(rename = "START_HANGOUT"))]
    StartHangout,
    #[cfg_attr(feature = "serde-impl", serde(rename = "END_HANGOUT"))]
    EndHangout { hangout_duration_secs: String },
}
