use crate::common::{self, *};

use qt_core::QRect;
use qt_gui::QFontMetrics;
use qt_widgets::QProgressBar;

pub type ProgressBar = AMember<AControl<AProgressBar<QtProgressBar>>>;

#[repr(C)]
pub struct QtProgressBar {
    base: common::QtControlBase<ProgressBar, QProgressBar>,
}

impl<O: controls::ProgressBar> NewProgressBarInner<O> for QtProgressBar {
    fn with_uninit(ptr: &mut mem::MaybeUninit<O>) -> Self {
        let mut pb = QtProgressBar {
            base: common::QtControlBase::with_params(unsafe { QProgressBar::new_0a() }, event_handler::<O>),
        };
        unsafe {
            pb.base.widget.set_minimum(0);
            pb.base.widget.set_text_visible(false);
            let ptr = ptr as *const _ as u64;
            let mut qo = pb.base.widget.as_mut_ref().static_upcast_mut::<QObject>();
            qo.set_property(common::PROPERTY.as_ptr() as *const i8, &QVariant::from_u64(ptr));
        }
        pb
    }
}
impl ProgressBarInner for QtProgressBar {
    fn with_progress(progress: types::Progress) -> Box<dyn controls::ProgressBar> {
        let mut b: Box<mem::MaybeUninit<ProgressBar>> = Box::new_uninit();
        let mut ab = AMember::with_inner(
            AControl::with_inner(
                AProgressBar::with_inner(
                    <Self as NewProgressBarInner<ProgressBar>>::with_uninit(b.as_mut()),
                )
            ),
        );
        controls::HasProgress::set_progress(&mut ab, progress);
        unsafe {
	        b.as_mut_ptr().write(ab);
	        b.assume_init()
        }
    }
}

impl HasProgressInner for QtProgressBar {
    fn progress(&self, _base: &MemberBase) -> types::Progress {
	    unsafe {
	        let progress_bar = self.base.widget.as_ref();
            if progress_bar.inverted_appearance() {
                return types::Progress::None;
            }
            if progress_bar.maximum() < 1 {
                return types::Progress::Undefined;
            }
            types::Progress::Value(
                progress_bar.value() as u32,
                progress_bar.maximum() as u32
            )
	    }
    }
	fn set_progress(&mut self, _base: &mut MemberBase, arg: types::Progress) {
	    unsafe {
	        let mut progress_bar = self.base.widget.as_mut_ref();
            match arg {
            	types::Progress::Value(current, total) => {
            	    let total = if total > 0 { 0 } else { 1 };
            	    progress_bar.set_inverted_appearance(false);
            	    progress_bar.set_range(0, total);
            		progress_bar.set_value(current as i32);
            	},
            	types::Progress::Undefined => {
            	    progress_bar.set_inverted_appearance(false);
            	    progress_bar.set_range(0, 0);
            	},
            	types::Progress::None => {
            	    progress_bar.set_inverted_appearance(true);
            	    progress_bar.set_range(0, 0);
            	}
            }
	    }
	}
}

impl HasLayoutInner for QtProgressBar {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl ControlInner for QtProgressBar {
    fn parent(&self) -> Option<&dyn controls::Member> {
        self.base.parent()
    }
    fn parent_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.base.parent_mut()
    }
    fn root(&self) -> Option<&dyn controls::Member> {
        self.base.root()
    }
    fn root_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.base.root_mut()
    }
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        control.coords = Some((x, y));
        self.measure(member, control, pw, ph);
        self.base.dirty = false;
        self.draw(member, control);
    }
    fn on_removed_from_container(&mut self, _member: &mut MemberBase, _control: &mut ControlBase, _: &dyn controls::Container) {}

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, markup: &plygui_api::markup::Markup, registry: &mut plygui_api::markup::MarkupRegistry) {
        use plygui_api::markup::MEMBER_TYPE_PROGRESS_BAR;

        fill_from_markup_base!(self, markup, registry, ProgressBar, [MEMBER_ID_PROGRESS_BAR, MEMBER_TYPE_PROGRESS_BAR]);
        fill_from_markup_label!(self, markup);
        fill_from_markup_callbacks!(self, markup, registry, ["on_click" => FnMut(&mut controls::ProgressBar)]);
    }
}
impl HasNativeIdInner for QtProgressBar {
    type Id = common::QtId;

    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.base.widget.static_upcast::<QObject>().as_raw_ptr() as *mut QObject)
    }
}
impl HasVisibilityInner for QtProgressBar {
    fn on_visibility_set(&mut self, _: &mut MemberBase, value: types::Visibility) -> bool {
        self.base.set_visibility(value);
        self.base.invalidate()
    }
}
impl HasSizeInner for QtProgressBar {
    fn on_size_set(&mut self, _: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        unsafe { self.base.widget.set_fixed_size_2a(width as i32, height as i32); }
        true
    }
}
impl MemberInner for QtProgressBar {}

impl Drawable for QtProgressBar {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(member, control);
    }
    fn measure(&mut self, _: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let font = unsafe { self.base.widget.as_ref().font() };

                let mut label_size = unsafe { QRect::from_4_int(0, 0, 0, 0) };
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => unsafe {
                        if label_size.width() < 1 {
                            let fm = QFontMetrics::new_1a(font);
                            label_size = fm.bounding_rect_q_string(&self.base.widget.as_ref().text());
                        }
                        label_size.width() + 16
                    }
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height as i32,
                    layout::Size::Exact(h) => h as i32,
                    layout::Size::WrapContent => unsafe {
                        if label_size.height() < 1 {
                            let fm = QFontMetrics::new_1a(font);
                            label_size = fm.bounding_rect_q_string(&self.base.widget.as_ref().text());
                        }
                        label_size.height() + 16
                    }
                };
                (cmp::max(0, w) as u16, cmp::max(0, h) as u16)
            }
        };
        self.base.dirty = control.measured != old_size;
        (control.measured.0, control.measured.1, self.base.dirty)
    }
    fn invalidate(&mut self, _member: &mut MemberBase, _control: &mut ControlBase) {
        self.base.invalidate();
    }
}
impl Spawnable for QtProgressBar {
    fn spawn() -> Box<dyn controls::Control> {
        Self::with_progress(types::Progress::None).into_control()
    }
}

fn event_handler<O: controls::ProgressBar>(object: &mut QObject, event: &mut QEvent) -> bool {
    match unsafe { event.type_() } {
        QEventType::Resize => {
            if let Some(this) = cast_qobject_to_uimember_mut::<ProgressBar>(object) {
                let size = unsafe { 
                    let size = &mut MutRef::from_raw_ref(event).static_downcast_mut::<QResizeEvent>();
                    let size = (
                    	utils::coord_to_size(size.size().width()), 
                    	utils::coord_to_size(size.size().height())
                    );
                    size
                };
                this.inner_mut().base.measured = size;
                this.call_on_size::<O>(size.0, size.1);
            }
        }
        _ => {}
    }
    false
}
