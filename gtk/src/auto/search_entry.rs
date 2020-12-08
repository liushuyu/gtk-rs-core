// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Align;
use crate::Buildable;
use crate::CellEditable;
use crate::Container;
use crate::Editable;
use crate::Entry;
use crate::EntryBuffer;
use crate::EntryCompletion;
use crate::InputHints;
use crate::InputPurpose;
use crate::ShadowType;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
#[cfg(any(feature = "v3_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct SearchEntry(Object<ffi::GtkSearchEntry, ffi::GtkSearchEntryClass>) @extends Entry, Widget, @implements Buildable, CellEditable, Editable;

    match fn {
        get_type => || ffi::gtk_search_entry_get_type(),
    }
}

impl SearchEntry {
    #[doc(alias = "gtk_search_entry_new")]
    pub fn new() -> SearchEntry {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_search_entry_new()).unsafe_cast() }
    }
}

impl Default for SearchEntry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct SearchEntryBuilder {
    activates_default: Option<bool>,
    attributes: Option<pango::AttrList>,
    buffer: Option<EntryBuffer>,
    caps_lock_warning: Option<bool>,
    completion: Option<EntryCompletion>,
    editable: Option<bool>,
    enable_emoji_completion: Option<bool>,
    has_frame: Option<bool>,
    im_module: Option<String>,
    input_hints: Option<InputHints>,
    input_purpose: Option<InputPurpose>,
    invisible_char: Option<u32>,
    invisible_char_set: Option<bool>,
    max_length: Option<i32>,
    max_width_chars: Option<i32>,
    overwrite_mode: Option<bool>,
    placeholder_text: Option<String>,
    populate_all: Option<bool>,
    primary_icon_activatable: Option<bool>,
    primary_icon_gicon: Option<gio::Icon>,
    primary_icon_name: Option<String>,
    primary_icon_pixbuf: Option<gdk_pixbuf::Pixbuf>,
    primary_icon_sensitive: Option<bool>,
    primary_icon_tooltip_markup: Option<String>,
    primary_icon_tooltip_text: Option<String>,
    progress_fraction: Option<f64>,
    progress_pulse_step: Option<f64>,
    secondary_icon_activatable: Option<bool>,
    secondary_icon_gicon: Option<gio::Icon>,
    secondary_icon_name: Option<String>,
    secondary_icon_pixbuf: Option<gdk_pixbuf::Pixbuf>,
    secondary_icon_sensitive: Option<bool>,
    secondary_icon_tooltip_markup: Option<String>,
    secondary_icon_tooltip_text: Option<String>,
    shadow_type: Option<ShadowType>,
    show_emoji_icon: Option<bool>,
    tabs: Option<pango::TabArray>,
    text: Option<String>,
    truncate_multiline: Option<bool>,
    visibility: Option<bool>,
    width_chars: Option<i32>,
    xalign: Option<f32>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    editing_canceled: Option<bool>,
}

impl SearchEntryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> SearchEntry {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref activates_default) = self.activates_default {
            properties.push(("activates-default", activates_default));
        }
        if let Some(ref attributes) = self.attributes {
            properties.push(("attributes", attributes));
        }
        if let Some(ref buffer) = self.buffer {
            properties.push(("buffer", buffer));
        }
        if let Some(ref caps_lock_warning) = self.caps_lock_warning {
            properties.push(("caps-lock-warning", caps_lock_warning));
        }
        if let Some(ref completion) = self.completion {
            properties.push(("completion", completion));
        }
        if let Some(ref editable) = self.editable {
            properties.push(("editable", editable));
        }
        if let Some(ref enable_emoji_completion) = self.enable_emoji_completion {
            properties.push(("enable-emoji-completion", enable_emoji_completion));
        }
        if let Some(ref has_frame) = self.has_frame {
            properties.push(("has-frame", has_frame));
        }
        if let Some(ref im_module) = self.im_module {
            properties.push(("im-module", im_module));
        }
        if let Some(ref input_hints) = self.input_hints {
            properties.push(("input-hints", input_hints));
        }
        if let Some(ref input_purpose) = self.input_purpose {
            properties.push(("input-purpose", input_purpose));
        }
        if let Some(ref invisible_char) = self.invisible_char {
            properties.push(("invisible-char", invisible_char));
        }
        if let Some(ref invisible_char_set) = self.invisible_char_set {
            properties.push(("invisible-char-set", invisible_char_set));
        }
        if let Some(ref max_length) = self.max_length {
            properties.push(("max-length", max_length));
        }
        if let Some(ref max_width_chars) = self.max_width_chars {
            properties.push(("max-width-chars", max_width_chars));
        }
        if let Some(ref overwrite_mode) = self.overwrite_mode {
            properties.push(("overwrite-mode", overwrite_mode));
        }
        if let Some(ref placeholder_text) = self.placeholder_text {
            properties.push(("placeholder-text", placeholder_text));
        }
        if let Some(ref populate_all) = self.populate_all {
            properties.push(("populate-all", populate_all));
        }
        if let Some(ref primary_icon_activatable) = self.primary_icon_activatable {
            properties.push(("primary-icon-activatable", primary_icon_activatable));
        }
        if let Some(ref primary_icon_gicon) = self.primary_icon_gicon {
            properties.push(("primary-icon-gicon", primary_icon_gicon));
        }
        if let Some(ref primary_icon_name) = self.primary_icon_name {
            properties.push(("primary-icon-name", primary_icon_name));
        }
        if let Some(ref primary_icon_pixbuf) = self.primary_icon_pixbuf {
            properties.push(("primary-icon-pixbuf", primary_icon_pixbuf));
        }
        if let Some(ref primary_icon_sensitive) = self.primary_icon_sensitive {
            properties.push(("primary-icon-sensitive", primary_icon_sensitive));
        }
        if let Some(ref primary_icon_tooltip_markup) = self.primary_icon_tooltip_markup {
            properties.push(("primary-icon-tooltip-markup", primary_icon_tooltip_markup));
        }
        if let Some(ref primary_icon_tooltip_text) = self.primary_icon_tooltip_text {
            properties.push(("primary-icon-tooltip-text", primary_icon_tooltip_text));
        }
        if let Some(ref progress_fraction) = self.progress_fraction {
            properties.push(("progress-fraction", progress_fraction));
        }
        if let Some(ref progress_pulse_step) = self.progress_pulse_step {
            properties.push(("progress-pulse-step", progress_pulse_step));
        }
        if let Some(ref secondary_icon_activatable) = self.secondary_icon_activatable {
            properties.push(("secondary-icon-activatable", secondary_icon_activatable));
        }
        if let Some(ref secondary_icon_gicon) = self.secondary_icon_gicon {
            properties.push(("secondary-icon-gicon", secondary_icon_gicon));
        }
        if let Some(ref secondary_icon_name) = self.secondary_icon_name {
            properties.push(("secondary-icon-name", secondary_icon_name));
        }
        if let Some(ref secondary_icon_pixbuf) = self.secondary_icon_pixbuf {
            properties.push(("secondary-icon-pixbuf", secondary_icon_pixbuf));
        }
        if let Some(ref secondary_icon_sensitive) = self.secondary_icon_sensitive {
            properties.push(("secondary-icon-sensitive", secondary_icon_sensitive));
        }
        if let Some(ref secondary_icon_tooltip_markup) = self.secondary_icon_tooltip_markup {
            properties.push((
                "secondary-icon-tooltip-markup",
                secondary_icon_tooltip_markup,
            ));
        }
        if let Some(ref secondary_icon_tooltip_text) = self.secondary_icon_tooltip_text {
            properties.push(("secondary-icon-tooltip-text", secondary_icon_tooltip_text));
        }
        if let Some(ref shadow_type) = self.shadow_type {
            properties.push(("shadow-type", shadow_type));
        }
        if let Some(ref show_emoji_icon) = self.show_emoji_icon {
            properties.push(("show-emoji-icon", show_emoji_icon));
        }
        if let Some(ref tabs) = self.tabs {
            properties.push(("tabs", tabs));
        }
        if let Some(ref text) = self.text {
            properties.push(("text", text));
        }
        if let Some(ref truncate_multiline) = self.truncate_multiline {
            properties.push(("truncate-multiline", truncate_multiline));
        }
        if let Some(ref visibility) = self.visibility {
            properties.push(("visibility", visibility));
        }
        if let Some(ref width_chars) = self.width_chars {
            properties.push(("width-chars", width_chars));
        }
        if let Some(ref xalign) = self.xalign {
            properties.push(("xalign", xalign));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        #[cfg(any(feature = "v3_20", feature = "dox"))]
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref editing_canceled) = self.editing_canceled {
            properties.push(("editing-canceled", editing_canceled));
        }
        let ret = glib::Object::new(SearchEntry::static_type(), &properties)
            .expect("object new")
            .downcast::<SearchEntry>()
            .expect("downcast");
        ret
    }

    pub fn activates_default(mut self, activates_default: bool) -> Self {
        self.activates_default = Some(activates_default);
        self
    }

    pub fn attributes(mut self, attributes: &pango::AttrList) -> Self {
        self.attributes = Some(attributes.clone());
        self
    }

    pub fn buffer<P: IsA<EntryBuffer>>(mut self, buffer: &P) -> Self {
        self.buffer = Some(buffer.clone().upcast());
        self
    }

    pub fn caps_lock_warning(mut self, caps_lock_warning: bool) -> Self {
        self.caps_lock_warning = Some(caps_lock_warning);
        self
    }

    pub fn completion<P: IsA<EntryCompletion>>(mut self, completion: &P) -> Self {
        self.completion = Some(completion.clone().upcast());
        self
    }

    pub fn editable(mut self, editable: bool) -> Self {
        self.editable = Some(editable);
        self
    }

    pub fn enable_emoji_completion(mut self, enable_emoji_completion: bool) -> Self {
        self.enable_emoji_completion = Some(enable_emoji_completion);
        self
    }

    pub fn has_frame(mut self, has_frame: bool) -> Self {
        self.has_frame = Some(has_frame);
        self
    }

    pub fn im_module(mut self, im_module: &str) -> Self {
        self.im_module = Some(im_module.to_string());
        self
    }

    pub fn input_hints(mut self, input_hints: InputHints) -> Self {
        self.input_hints = Some(input_hints);
        self
    }

    pub fn input_purpose(mut self, input_purpose: InputPurpose) -> Self {
        self.input_purpose = Some(input_purpose);
        self
    }

    pub fn invisible_char(mut self, invisible_char: u32) -> Self {
        self.invisible_char = Some(invisible_char);
        self
    }

    pub fn invisible_char_set(mut self, invisible_char_set: bool) -> Self {
        self.invisible_char_set = Some(invisible_char_set);
        self
    }

    pub fn max_length(mut self, max_length: i32) -> Self {
        self.max_length = Some(max_length);
        self
    }

    pub fn max_width_chars(mut self, max_width_chars: i32) -> Self {
        self.max_width_chars = Some(max_width_chars);
        self
    }

    pub fn overwrite_mode(mut self, overwrite_mode: bool) -> Self {
        self.overwrite_mode = Some(overwrite_mode);
        self
    }

    pub fn placeholder_text(mut self, placeholder_text: &str) -> Self {
        self.placeholder_text = Some(placeholder_text.to_string());
        self
    }

    pub fn populate_all(mut self, populate_all: bool) -> Self {
        self.populate_all = Some(populate_all);
        self
    }

    pub fn primary_icon_activatable(mut self, primary_icon_activatable: bool) -> Self {
        self.primary_icon_activatable = Some(primary_icon_activatable);
        self
    }

    pub fn primary_icon_gicon<P: IsA<gio::Icon>>(mut self, primary_icon_gicon: &P) -> Self {
        self.primary_icon_gicon = Some(primary_icon_gicon.clone().upcast());
        self
    }

    pub fn primary_icon_name(mut self, primary_icon_name: &str) -> Self {
        self.primary_icon_name = Some(primary_icon_name.to_string());
        self
    }

    pub fn primary_icon_pixbuf(mut self, primary_icon_pixbuf: &gdk_pixbuf::Pixbuf) -> Self {
        self.primary_icon_pixbuf = Some(primary_icon_pixbuf.clone());
        self
    }

    pub fn primary_icon_sensitive(mut self, primary_icon_sensitive: bool) -> Self {
        self.primary_icon_sensitive = Some(primary_icon_sensitive);
        self
    }

    pub fn primary_icon_tooltip_markup(mut self, primary_icon_tooltip_markup: &str) -> Self {
        self.primary_icon_tooltip_markup = Some(primary_icon_tooltip_markup.to_string());
        self
    }

    pub fn primary_icon_tooltip_text(mut self, primary_icon_tooltip_text: &str) -> Self {
        self.primary_icon_tooltip_text = Some(primary_icon_tooltip_text.to_string());
        self
    }

    pub fn progress_fraction(mut self, progress_fraction: f64) -> Self {
        self.progress_fraction = Some(progress_fraction);
        self
    }

    pub fn progress_pulse_step(mut self, progress_pulse_step: f64) -> Self {
        self.progress_pulse_step = Some(progress_pulse_step);
        self
    }

    pub fn secondary_icon_activatable(mut self, secondary_icon_activatable: bool) -> Self {
        self.secondary_icon_activatable = Some(secondary_icon_activatable);
        self
    }

    pub fn secondary_icon_gicon<P: IsA<gio::Icon>>(mut self, secondary_icon_gicon: &P) -> Self {
        self.secondary_icon_gicon = Some(secondary_icon_gicon.clone().upcast());
        self
    }

    pub fn secondary_icon_name(mut self, secondary_icon_name: &str) -> Self {
        self.secondary_icon_name = Some(secondary_icon_name.to_string());
        self
    }

    pub fn secondary_icon_pixbuf(mut self, secondary_icon_pixbuf: &gdk_pixbuf::Pixbuf) -> Self {
        self.secondary_icon_pixbuf = Some(secondary_icon_pixbuf.clone());
        self
    }

    pub fn secondary_icon_sensitive(mut self, secondary_icon_sensitive: bool) -> Self {
        self.secondary_icon_sensitive = Some(secondary_icon_sensitive);
        self
    }

    pub fn secondary_icon_tooltip_markup(mut self, secondary_icon_tooltip_markup: &str) -> Self {
        self.secondary_icon_tooltip_markup = Some(secondary_icon_tooltip_markup.to_string());
        self
    }

    pub fn secondary_icon_tooltip_text(mut self, secondary_icon_tooltip_text: &str) -> Self {
        self.secondary_icon_tooltip_text = Some(secondary_icon_tooltip_text.to_string());
        self
    }

    pub fn shadow_type(mut self, shadow_type: ShadowType) -> Self {
        self.shadow_type = Some(shadow_type);
        self
    }

    pub fn show_emoji_icon(mut self, show_emoji_icon: bool) -> Self {
        self.show_emoji_icon = Some(show_emoji_icon);
        self
    }

    pub fn tabs(mut self, tabs: &pango::TabArray) -> Self {
        self.tabs = Some(tabs.clone());
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn truncate_multiline(mut self, truncate_multiline: bool) -> Self {
        self.truncate_multiline = Some(truncate_multiline);
        self
    }

    pub fn visibility(mut self, visibility: bool) -> Self {
        self.visibility = Some(visibility);
        self
    }

    pub fn width_chars(mut self, width_chars: i32) -> Self {
        self.width_chars = Some(width_chars);
        self
    }

    pub fn xalign(mut self, xalign: f32) -> Self {
        self.xalign = Some(xalign);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent<P: IsA<Container>>(mut self, parent: &P) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn editing_canceled(mut self, editing_canceled: bool) -> Self {
        self.editing_canceled = Some(editing_canceled);
        self
    }
}

pub const NONE_SEARCH_ENTRY: Option<&SearchEntry> = None;

pub trait SearchEntryExt: 'static {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
    #[doc(alias = "gtk_search_entry_handle_event")]
    fn handle_event(&self, event: &gdk::Event) -> bool;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
    fn connect_next_match<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
    fn emit_next_match(&self);

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
    fn connect_previous_match<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
    fn emit_previous_match(&self);

    fn connect_search_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
    fn connect_stop_search<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
    fn emit_stop_search(&self);
}

impl<O: IsA<SearchEntry>> SearchEntryExt for O {
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
    fn handle_event(&self, event: &gdk::Event) -> bool {
        unsafe {
            from_glib(ffi::gtk_search_entry_handle_event(
                self.as_ref().to_glib_none().0,
                mut_override(event.to_glib_none().0),
            ))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
    fn connect_next_match<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn next_match_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSearchEntry,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<SearchEntry>,
        {
            let f: &F = &*(f as *const F);
            f(&SearchEntry::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"next-match\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    next_match_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
    fn emit_next_match(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("next-match", &[])
                .unwrap()
        };
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
    fn connect_previous_match<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn previous_match_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSearchEntry,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<SearchEntry>,
        {
            let f: &F = &*(f as *const F);
            f(&SearchEntry::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"previous-match\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    previous_match_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
    fn emit_previous_match(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("previous-match", &[])
                .unwrap()
        };
    }

    fn connect_search_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn search_changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSearchEntry,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<SearchEntry>,
        {
            let f: &F = &*(f as *const F);
            f(&SearchEntry::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"search-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    search_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
    fn connect_stop_search<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn stop_search_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkSearchEntry,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<SearchEntry>,
        {
            let f: &F = &*(f as *const F);
            f(&SearchEntry::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"stop-search\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    stop_search_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
    fn emit_stop_search(&self) {
        let _ = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit("stop-search", &[])
                .unwrap()
        };
    }
}

impl fmt::Display for SearchEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SearchEntry")
    }
}
