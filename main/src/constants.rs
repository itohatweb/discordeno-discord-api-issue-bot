use twilight_model::{
    channel::ReactionType,
    id::{ChannelId, EmojiId, UserId},
};

pub const API_DOCS_CHANNEL: ChannelId = unsafe { ChannelId::new_unchecked(881991954676715653_u64) };
pub const API_DOCS_BOT_ID: UserId = unsafe { UserId::new_unchecked(881992163855065089_u64) };
pub const ISSUE_MANAGEMENT_USERS: [UserId; 1] =
    unsafe { [UserId::new_unchecked(615542460151496705_u64)] };

pub const ISSUE_BUTTON_EMOJI: ReactionType = unsafe {
    ReactionType::Custom {
        animated: false,
        id: EmojiId::new_unchecked(754789242412073010_u64),
        name: None,
    }
};

pub const REMOVE_BUTTON_EMOJI: ReactionType = unsafe {
    ReactionType::Custom {
        animated: false,
        id: EmojiId::new_unchecked(853559407027683328_u64),
        name: None,
    }
};
