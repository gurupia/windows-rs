#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ATTENDEE_DISCONNECT_REASON(pub i32);
pub const ATTENDEE_DISCONNECT_REASON_MIN: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(0i32);
pub const ATTENDEE_DISCONNECT_REASON_APP: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(0i32);
pub const ATTENDEE_DISCONNECT_REASON_ERR: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(1i32);
pub const ATTENDEE_DISCONNECT_REASON_CLI: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(2i32);
pub const ATTENDEE_DISCONNECT_REASON_MAX: ATTENDEE_DISCONNECT_REASON = ATTENDEE_DISCONNECT_REASON(2i32);
impl ::core::marker::Copy for ATTENDEE_DISCONNECT_REASON {}
impl ::core::clone::Clone for ATTENDEE_DISCONNECT_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CHANNEL_ACCESS_ENUM(pub i32);
pub const CHANNEL_ACCESS_ENUM_NONE: CHANNEL_ACCESS_ENUM = CHANNEL_ACCESS_ENUM(0i32);
pub const CHANNEL_ACCESS_ENUM_SENDRECEIVE: CHANNEL_ACCESS_ENUM = CHANNEL_ACCESS_ENUM(1i32);
impl ::core::marker::Copy for CHANNEL_ACCESS_ENUM {}
impl ::core::clone::Clone for CHANNEL_ACCESS_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CHANNEL_FLAGS(pub i32);
pub const CHANNEL_FLAGS_LEGACY: CHANNEL_FLAGS = CHANNEL_FLAGS(1i32);
pub const CHANNEL_FLAGS_UNCOMPRESSED: CHANNEL_FLAGS = CHANNEL_FLAGS(2i32);
pub const CHANNEL_FLAGS_DYNAMIC: CHANNEL_FLAGS = CHANNEL_FLAGS(4i32);
impl ::core::marker::Copy for CHANNEL_FLAGS {}
impl ::core::clone::Clone for CHANNEL_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CHANNEL_PRIORITY(pub i32);
pub const CHANNEL_PRIORITY_LO: CHANNEL_PRIORITY = CHANNEL_PRIORITY(0i32);
pub const CHANNEL_PRIORITY_MED: CHANNEL_PRIORITY = CHANNEL_PRIORITY(1i32);
pub const CHANNEL_PRIORITY_HI: CHANNEL_PRIORITY = CHANNEL_PRIORITY(2i32);
impl ::core::marker::Copy for CHANNEL_PRIORITY {}
impl ::core::clone::Clone for CHANNEL_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CTRL_LEVEL(pub i32);
pub const CTRL_LEVEL_MIN: CTRL_LEVEL = CTRL_LEVEL(0i32);
pub const CTRL_LEVEL_INVALID: CTRL_LEVEL = CTRL_LEVEL(0i32);
pub const CTRL_LEVEL_NONE: CTRL_LEVEL = CTRL_LEVEL(1i32);
pub const CTRL_LEVEL_VIEW: CTRL_LEVEL = CTRL_LEVEL(2i32);
pub const CTRL_LEVEL_INTERACTIVE: CTRL_LEVEL = CTRL_LEVEL(3i32);
pub const CTRL_LEVEL_REQCTRL_VIEW: CTRL_LEVEL = CTRL_LEVEL(4i32);
pub const CTRL_LEVEL_REQCTRL_INTERACTIVE: CTRL_LEVEL = CTRL_LEVEL(5i32);
pub const CTRL_LEVEL_MAX: CTRL_LEVEL = CTRL_LEVEL(5i32);
impl ::core::marker::Copy for CTRL_LEVEL {}
impl ::core::clone::Clone for CTRL_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DISPID_RDPAPI_EVENT_ON_BOUNDING_RECT_CHANGED: u32 = 340u32;
pub const DISPID_RDPSRAPI_EVENT_ON_APPFILTER_UPDATE: u32 = 322u32;
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_CLOSE: u32 = 317u32;
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_OPEN: u32 = 316u32;
pub const DISPID_RDPSRAPI_EVENT_ON_APPLICATION_UPDATE: u32 = 318u32;
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_CONNECTED: u32 = 301u32;
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_DISCONNECTED: u32 = 302u32;
pub const DISPID_RDPSRAPI_EVENT_ON_ATTENDEE_UPDATE: u32 = 303u32;
pub const DISPID_RDPSRAPI_EVENT_ON_CTRLLEVEL_CHANGE_REQUEST: u32 = 309u32;
pub const DISPID_RDPSRAPI_EVENT_ON_CTRLLEVEL_CHANGE_RESPONSE: u32 = 338u32;
pub const DISPID_RDPSRAPI_EVENT_ON_ERROR: u32 = 304u32;
pub const DISPID_RDPSRAPI_EVENT_ON_FOCUSRELEASED: u32 = 324u32;
pub const DISPID_RDPSRAPI_EVENT_ON_GRAPHICS_STREAM_PAUSED: u32 = 310u32;
pub const DISPID_RDPSRAPI_EVENT_ON_GRAPHICS_STREAM_RESUMED: u32 = 311u32;
pub const DISPID_RDPSRAPI_EVENT_ON_SHARED_DESKTOP_SETTINGS_CHANGED: u32 = 325u32;
pub const DISPID_RDPSRAPI_EVENT_ON_SHARED_RECT_CHANGED: u32 = 323u32;
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_CLOSED: u32 = 634u32;
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_DATARECEIVED: u32 = 633u32;
pub const DISPID_RDPSRAPI_EVENT_ON_STREAM_SENDCOMPLETED: u32 = 632u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_AUTHENTICATED: u32 = 307u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_CONNECTED: u32 = 305u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_CONNECTFAILED: u32 = 308u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIEWER_DISCONNECTED: u32 = 306u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_DATARECEIVED: u32 = 314u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_JOIN: u32 = 312u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_LEAVE: u32 = 313u32;
pub const DISPID_RDPSRAPI_EVENT_ON_VIRTUAL_CHANNEL_SENDCOMPLETED: u32 = 315u32;
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_CLOSE: u32 = 320u32;
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_OPEN: u32 = 319u32;
pub const DISPID_RDPSRAPI_EVENT_ON_WINDOW_UPDATE: u32 = 321u32;
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_BUTTON_RECEIVED: u32 = 700u32;
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_MOVE_RECEIVED: u32 = 701u32;
pub const DISPID_RDPSRAPI_EVENT_VIEW_MOUSE_WHEEL_RECEIVED: u32 = 702u32;
pub const DISPID_RDPSRAPI_METHOD_ADD_TOUCH_INPUT: u32 = 125u32;
pub const DISPID_RDPSRAPI_METHOD_BEGIN_TOUCH_FRAME: u32 = 124u32;
pub const DISPID_RDPSRAPI_METHOD_CLOSE: u32 = 101u32;
pub const DISPID_RDPSRAPI_METHOD_CONNECTTOCLIENT: u32 = 117u32;
pub const DISPID_RDPSRAPI_METHOD_CONNECTUSINGTRANSPORTSTREAM: u32 = 127u32;
pub const DISPID_RDPSRAPI_METHOD_CREATE_INVITATION: u32 = 107u32;
pub const DISPID_RDPSRAPI_METHOD_END_TOUCH_FRAME: u32 = 126u32;
pub const DISPID_RDPSRAPI_METHOD_GETFRAMEBUFFERBITS: u32 = 149u32;
pub const DISPID_RDPSRAPI_METHOD_GETSHAREDRECT: u32 = 103u32;
pub const DISPID_RDPSRAPI_METHOD_OPEN: u32 = 100u32;
pub const DISPID_RDPSRAPI_METHOD_PAUSE: u32 = 112u32;
pub const DISPID_RDPSRAPI_METHOD_REQUEST_COLOR_DEPTH_CHANGE: u32 = 115u32;
pub const DISPID_RDPSRAPI_METHOD_REQUEST_CONTROL: u32 = 108u32;
pub const DISPID_RDPSRAPI_METHOD_RESUME: u32 = 113u32;
pub const DISPID_RDPSRAPI_METHOD_SENDCONTROLLEVELCHANGERESPONSE: u32 = 148u32;
pub const DISPID_RDPSRAPI_METHOD_SEND_KEYBOARD_EVENT: u32 = 122u32;
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_BUTTON_EVENT: u32 = 119u32;
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_MOVE_EVENT: u32 = 120u32;
pub const DISPID_RDPSRAPI_METHOD_SEND_MOUSE_WHEEL_EVENT: u32 = 121u32;
pub const DISPID_RDPSRAPI_METHOD_SEND_SYNC_EVENT: u32 = 123u32;
pub const DISPID_RDPSRAPI_METHOD_SETSHAREDRECT: u32 = 102u32;
pub const DISPID_RDPSRAPI_METHOD_SET_RENDERING_SURFACE: u32 = 118u32;
pub const DISPID_RDPSRAPI_METHOD_SHOW_WINDOW: u32 = 114u32;
pub const DISPID_RDPSRAPI_METHOD_STARTREVCONNECTLISTENER: u32 = 116u32;
pub const DISPID_RDPSRAPI_METHOD_STREAMCLOSE: u32 = 426u32;
pub const DISPID_RDPSRAPI_METHOD_STREAMOPEN: u32 = 425u32;
pub const DISPID_RDPSRAPI_METHOD_STREAMREADDATA: u32 = 424u32;
pub const DISPID_RDPSRAPI_METHOD_STREAMSENDDATA: u32 = 423u32;
pub const DISPID_RDPSRAPI_METHOD_STREAM_ALLOCBUFFER: u32 = 421u32;
pub const DISPID_RDPSRAPI_METHOD_STREAM_FREEBUFFER: u32 = 422u32;
pub const DISPID_RDPSRAPI_METHOD_TERMINATE_CONNECTION: u32 = 106u32;
pub const DISPID_RDPSRAPI_METHOD_VIEWERCONNECT: u32 = 104u32;
pub const DISPID_RDPSRAPI_METHOD_VIEWERDISCONNECT: u32 = 105u32;
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_CREATE: u32 = 109u32;
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_SEND_DATA: u32 = 110u32;
pub const DISPID_RDPSRAPI_METHOD_VIRTUAL_CHANNEL_SET_ACCESS: u32 = 111u32;
pub const DISPID_RDPSRAPI_PROP_APPFILTERENABLED: u32 = 219u32;
pub const DISPID_RDPSRAPI_PROP_APPFILTER_ENABLED: u32 = 218u32;
pub const DISPID_RDPSRAPI_PROP_APPFLAGS: u32 = 223u32;
pub const DISPID_RDPSRAPI_PROP_APPLICATION: u32 = 211u32;
pub const DISPID_RDPSRAPI_PROP_APPLICATION_FILTER: u32 = 215u32;
pub const DISPID_RDPSRAPI_PROP_APPLICATION_LIST: u32 = 217u32;
pub const DISPID_RDPSRAPI_PROP_APPNAME: u32 = 214u32;
pub const DISPID_RDPSRAPI_PROP_ATTENDEELIMIT: u32 = 235u32;
pub const DISPID_RDPSRAPI_PROP_ATTENDEES: u32 = 203u32;
pub const DISPID_RDPSRAPI_PROP_ATTENDEE_FLAGS: u32 = 230u32;
pub const DISPID_RDPSRAPI_PROP_CHANNELMANAGER: u32 = 206u32;
pub const DISPID_RDPSRAPI_PROP_CODE: u32 = 241u32;
pub const DISPID_RDPSRAPI_PROP_CONINFO: u32 = 231u32;
pub const DISPID_RDPSRAPI_PROP_CONNECTION_STRING: u32 = 232u32;
pub const DISPID_RDPSRAPI_PROP_COUNT: u32 = 244u32;
pub const DISPID_RDPSRAPI_PROP_CTRL_LEVEL: u32 = 242u32;
pub const DISPID_RDPSRAPI_PROP_DBG_CLX_CMDLINE: u32 = 222u32;
pub const DISPID_RDPSRAPI_PROP_DISCONNECTED_STRING: u32 = 237u32;
pub const DISPID_RDPSRAPI_PROP_DISPIDVALUE: u32 = 200u32;
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER: u32 = 254u32;
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_BPP: u32 = 253u32;
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_HEIGHT: u32 = 251u32;
pub const DISPID_RDPSRAPI_PROP_FRAMEBUFFER_WIDTH: u32 = 252u32;
pub const DISPID_RDPSRAPI_PROP_GROUP_NAME: u32 = 233u32;
pub const DISPID_RDPSRAPI_PROP_ID: u32 = 201u32;
pub const DISPID_RDPSRAPI_PROP_INVITATION: u32 = 205u32;
pub const DISPID_RDPSRAPI_PROP_INVITATIONITEM: u32 = 221u32;
pub const DISPID_RDPSRAPI_PROP_INVITATIONS: u32 = 204u32;
pub const DISPID_RDPSRAPI_PROP_LOCAL_IP: u32 = 227u32;
pub const DISPID_RDPSRAPI_PROP_LOCAL_PORT: u32 = 226u32;
pub const DISPID_RDPSRAPI_PROP_PASSWORD: u32 = 234u32;
pub const DISPID_RDPSRAPI_PROP_PEER_IP: u32 = 229u32;
pub const DISPID_RDPSRAPI_PROP_PEER_PORT: u32 = 228u32;
pub const DISPID_RDPSRAPI_PROP_PROTOCOL_TYPE: u32 = 225u32;
pub const DISPID_RDPSRAPI_PROP_REASON: u32 = 240u32;
pub const DISPID_RDPSRAPI_PROP_REMOTENAME: u32 = 243u32;
pub const DISPID_RDPSRAPI_PROP_REVOKED: u32 = 236u32;
pub const DISPID_RDPSRAPI_PROP_SESSION_COLORDEPTH: u32 = 239u32;
pub const DISPID_RDPSRAPI_PROP_SESSION_PROPERTIES: u32 = 202u32;
pub const DISPID_RDPSRAPI_PROP_SHARED: u32 = 220u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_CONTEXT: u32 = 560u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_FLAGS: u32 = 561u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_PAYLOADOFFSET: u32 = 559u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_PAYLOADSIZE: u32 = 558u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_STORAGE: u32 = 555u32;
pub const DISPID_RDPSRAPI_PROP_STREAMBUFFER_STORESIZE: u32 = 562u32;
pub const DISPID_RDPSRAPI_PROP_USESMARTSIZING: u32 = 238u32;
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETFLAGS: u32 = 208u32;
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETNAME: u32 = 207u32;
pub const DISPID_RDPSRAPI_PROP_VIRTUAL_CHANNEL_GETPRIORITY: u32 = 209u32;
pub const DISPID_RDPSRAPI_PROP_WINDOWID: u32 = 210u32;
pub const DISPID_RDPSRAPI_PROP_WINDOWNAME: u32 = 213u32;
pub const DISPID_RDPSRAPI_PROP_WINDOWSHARED: u32 = 212u32;
pub const DISPID_RDPSRAPI_PROP_WINDOW_LIST: u32 = 216u32;
pub const DISPID_RDPSRAPI_PROP_WNDFLAGS: u32 = 224u32;
#[repr(transparent)]
pub struct IRDPSRAPIApplication(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIApplicationFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIApplicationList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIAttendee(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIAttendeeDisconnectInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIAttendeeManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIAudioStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIClipboardUseEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIDebug(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIFrameBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIInvitation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIInvitationManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIPerfCounterLogger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIPerfCounterLoggingManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPISessionProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPISharingSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPISharingSession2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPITcpConnectionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPITransportStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPITransportStreamBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPITransportStreamEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIViewer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIVirtualChannel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIVirtualChannelManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIWindow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPSRAPIWindowList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRDPViewerInputSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RDPENCOMAPI_ATTENDEE_FLAGS(pub i32);
pub const ATTENDEE_FLAGS_LOCAL: RDPENCOMAPI_ATTENDEE_FLAGS = RDPENCOMAPI_ATTENDEE_FLAGS(1i32);
impl ::core::marker::Copy for RDPENCOMAPI_ATTENDEE_FLAGS {}
impl ::core::clone::Clone for RDPENCOMAPI_ATTENDEE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RDPSRAPIApplication: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3239486596,
    data2: 19237,
    data3: 19359,
    data4: [138, 84, 185, 52, 176, 110, 87, 250],
};
pub const RDPSRAPIApplicationFilter: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3814379145, data2: 51176, data3: 17022, data4: [164, 249, 185, 218, 7, 40, 38, 189] };
pub const RDPSRAPIApplicationList: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2654062613,
    data2: 29747,
    data3: 18550,
    data4: [151, 251, 237, 89, 254, 43, 170, 34],
};
pub const RDPSRAPIAttendee: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1962490805, data2: 30047, data3: 18574, data4: [138, 41, 35, 144, 16, 138, 239, 85] };
pub const RDPSRAPIAttendeeDisconnectInfo: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3028120144,
    data2: 23515,
    data3: 16477,
    data4: [180, 135, 202, 173, 156, 86, 244, 248],
};
pub const RDPSRAPIAttendeeManager: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3618716161,
    data2: 63444,
    data3: 17062,
    data4: [133, 149, 18, 252, 140, 36, 232, 81],
};
pub const RDPSRAPIFrameBuffer: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2767612876, data2: 21390, data3: 16641, data4: [149, 29, 48, 132, 122, 219, 81, 1] };
pub const RDPSRAPIInvitation: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1226264006,
    data2: 1841,
    data3: 19294,
    data4: [142, 225, 131, 166, 61, 56, 104, 250],
};
pub const RDPSRAPIInvitationManager: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1406781915,
    data2: 30123,
    data3: 17009,
    data4: [148, 138, 76, 78, 179, 106, 143, 43],
};
pub const RDPSRAPISessionProperties: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3715470591,
    data2: 59946,
    data3: 19462,
    data4: [143, 223, 19, 45, 228, 139, 101, 16],
};
pub const RDPSRAPITcpConnectionInfo: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3192511295,
    data2: 60342,
    data3: 17016,
    data4: [140, 224, 213, 69, 88, 51, 234, 238],
};
pub const RDPSRAPIWindow: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 63915739, data2: 52805, data3: 19766, data4: [134, 237, 237, 40, 183, 67, 152, 191] };
pub const RDPSRAPIWindowList: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2619466424, data2: 24020, data3: 17100, data4: [129, 186, 28, 9, 152, 82, 230, 250] };
#[repr(transparent)]
pub struct RDPSRAPI_APP_FLAGS(pub i32);
pub const APP_FLAG_PRIVILEGED: RDPSRAPI_APP_FLAGS = RDPSRAPI_APP_FLAGS(1i32);
impl ::core::marker::Copy for RDPSRAPI_APP_FLAGS {}
impl ::core::clone::Clone for RDPSRAPI_APP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RDPSRAPI_KBD_CODE_TYPE(pub i32);
pub const RDPSRAPI_KBD_CODE_SCANCODE: RDPSRAPI_KBD_CODE_TYPE = RDPSRAPI_KBD_CODE_TYPE(0i32);
pub const RDPSRAPI_KBD_CODE_UNICODE: RDPSRAPI_KBD_CODE_TYPE = RDPSRAPI_KBD_CODE_TYPE(1i32);
impl ::core::marker::Copy for RDPSRAPI_KBD_CODE_TYPE {}
impl ::core::clone::Clone for RDPSRAPI_KBD_CODE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RDPSRAPI_KBD_SYNC_FLAG(pub i32);
pub const RDPSRAPI_KBD_SYNC_FLAG_SCROLL_LOCK: RDPSRAPI_KBD_SYNC_FLAG = RDPSRAPI_KBD_SYNC_FLAG(1i32);
pub const RDPSRAPI_KBD_SYNC_FLAG_NUM_LOCK: RDPSRAPI_KBD_SYNC_FLAG = RDPSRAPI_KBD_SYNC_FLAG(2i32);
pub const RDPSRAPI_KBD_SYNC_FLAG_CAPS_LOCK: RDPSRAPI_KBD_SYNC_FLAG = RDPSRAPI_KBD_SYNC_FLAG(4i32);
pub const RDPSRAPI_KBD_SYNC_FLAG_KANA_LOCK: RDPSRAPI_KBD_SYNC_FLAG = RDPSRAPI_KBD_SYNC_FLAG(8i32);
impl ::core::marker::Copy for RDPSRAPI_KBD_SYNC_FLAG {}
impl ::core::clone::Clone for RDPSRAPI_KBD_SYNC_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RDPSRAPI_MOUSE_BUTTON_TYPE(pub i32);
pub const RDPSRAPI_MOUSE_BUTTON_BUTTON1: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(0i32);
pub const RDPSRAPI_MOUSE_BUTTON_BUTTON2: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(1i32);
pub const RDPSRAPI_MOUSE_BUTTON_BUTTON3: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(2i32);
pub const RDPSRAPI_MOUSE_BUTTON_XBUTTON1: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(3i32);
pub const RDPSRAPI_MOUSE_BUTTON_XBUTTON2: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(4i32);
pub const RDPSRAPI_MOUSE_BUTTON_XBUTTON3: RDPSRAPI_MOUSE_BUTTON_TYPE = RDPSRAPI_MOUSE_BUTTON_TYPE(5i32);
impl ::core::marker::Copy for RDPSRAPI_MOUSE_BUTTON_TYPE {}
impl ::core::clone::Clone for RDPSRAPI_MOUSE_BUTTON_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RDPSRAPI_WND_FLAGS(pub i32);
pub const WND_FLAG_PRIVILEGED: RDPSRAPI_WND_FLAGS = RDPSRAPI_WND_FLAGS(1i32);
impl ::core::marker::Copy for RDPSRAPI_WND_FLAGS {}
impl ::core::clone::Clone for RDPSRAPI_WND_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RDPSession: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2608394470,
    data2: 15877,
    data3: 19035,
    data4: [178, 232, 231, 67, 168, 149, 107, 101],
};
pub const RDPTransportStreamBuffer: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2370444393,
    data2: 61823,
    data3: 17737,
    data4: [166, 153, 118, 28, 110, 107, 92, 10],
};
pub const RDPTransportStreamEvents: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 837004064,
    data2: 21328,
    data3: 18495,
    data4: [157, 198, 103, 72, 102, 94, 253, 235],
};
pub const RDPViewer: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 851336914, data2: 23686, data3: 18447, data4: [169, 20, 15, 248, 136, 90, 27, 63] };
#[repr(transparent)]
pub struct _IRDPSessionEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001(pub i32);
pub const CONST_MAX_CHANNEL_MESSAGE_SIZE: __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 = __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001(1024i32);
pub const CONST_MAX_CHANNEL_NAME_LEN: __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 = __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001(8i32);
pub const CONST_MAX_LEGACY_CHANNEL_MESSAGE_SIZE: __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 = __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001(409600i32);
pub const CONST_ATTENDEE_ID_EVERYONE: __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 = __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001(-1i32);
pub const CONST_ATTENDEE_ID_HOST: __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 = __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001(0i32);
pub const CONST_CONN_INTERVAL: __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 = __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001(50i32);
pub const CONST_ATTENDEE_ID_DEFAULT: __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 = __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001(-1i32);
impl ::core::marker::Copy for __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 {}
impl ::core::clone::Clone for __MIDL___MIDL_itf_rdpencomapi_0000_0027_0001 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct __ReferenceRemainingTypes__ {
    pub __ctrlLevel__: CTRL_LEVEL,
    pub __attendeeDisconnectReason__: ATTENDEE_DISCONNECT_REASON,
    pub __channelPriority__: CHANNEL_PRIORITY,
    pub __channelFlags__: CHANNEL_FLAGS,
    pub __channelAccessEnum__: CHANNEL_ACCESS_ENUM,
    pub __rdpencomapiAttendeeFlags__: RDPENCOMAPI_ATTENDEE_FLAGS,
    pub __rdpsrapiWndFlags__: RDPSRAPI_WND_FLAGS,
    pub __rdpsrapiAppFlags__: RDPSRAPI_APP_FLAGS,
}
impl ::core::marker::Copy for __ReferenceRemainingTypes__ {}
impl ::core::clone::Clone for __ReferenceRemainingTypes__ {
    fn clone(&self) -> Self {
        *self
    }
}
