use crate::{common::{self, matrix::*, *}, table};

use qt_widgets::{QTableWidget, QTableWidgetItem};
use qt_widgets::cpp_core::{CppBox, Ptr, NullPtr};
use qt_core::{AsReceiver, Receiver, QMargins, QModelIndex, Orientation, SlotOfIntInt, SlotOfInt, SlotOfOrientationIntInt, ScrollBarPolicy};

pub type Table = AMember<AControl<AContainer<AAdapted<ATable<QtTable>>>>>;

#[repr(C)]
pub struct QtTable {
    base: common::QtControlBase<Table, QTableWidget>,
    data: Matrix<Ptr<QTableWidgetItem>>,
    h_left_clicked: (Option<callbacks::OnItemClick>, QBox<SlotOfIntInt>, QBox<SlotOfInt>),
    headers_moved_slot: QBox<SlotOfInt>,
    headers_resized_slot: QBox<SlotOfInt>,
}
impl ItemClickableInner for QtTable {
    fn item_click(&mut self, i: &[usize], item_view: &mut dyn controls::Control, _skip_callbacks: bool) {
        let this = common::cast_qobject_to_uimember_mut::<Table>(&mut self.base.as_qwidget()).unwrap();
        if let Some(ref mut callback) = self.h_left_clicked.0 {
            (callback.as_mut())(this, i, item_view)
        }
    }
    fn on_item_click(&mut self, cb: Option<callbacks::OnItemClick>) {
        self.h_left_clicked.0 = cb;
    }
}
impl QtTable {
    fn add_row_inner(&mut self, base: &mut MemberBase, index: usize) -> Option<&mut Row<Ptr<QTableWidgetItem>>> {
        let (_, control, _, _) = unsafe { Table::adapter_base_parts_mut(base) };
        unsafe { self.base.widget.insert_row(index as i32); }
        let row = Row {
            cells: self.data.cols.iter_mut().map(|_| None).collect(),
            native: unsafe { self.base.widget.vertical_header_item(index as i32) },
            control: None,
            height: self.data.default_row_height,
        };
        self.data.rows.insert(index, row);
        self.resize_row(control, index, self.data.default_row_height, true);
        self.data.row_at_mut(index)
    }
    fn remove_row_inner(&mut self, _base: &mut MemberBase, index: usize) {
        let widget = &self.base.widget;
        self.data.row_at_mut(index).map(|row| {
            (0..row.cells.len()).into_iter().for_each(|y| {
                row.cells.remove(y).map(|mut cell| unsafe {
                    widget.cell_widget(index as i32, y as i32).static_upcast::<QObject>().set_property(PROPERTY_PARENT.as_ptr() as *const i8, &QVariant::from_u64(0));
                });
            });
        });
        //unsafe { self.data.rows.remove(index).native.static_upcast::<QObject>().set_property(PROPERTY_PARENT.as_ptr() as *const i8, &QVariant::from_u64(0)); }
        unsafe { self.base.widget.remove_row(index as i32); }
    }
	fn add_column_inner(&mut self, base: &mut MemberBase, index: usize) {
        let parent_ptr = base as *mut MemberBase as u64;
        let (member, control, adapter, _) = unsafe { Table::adapter_base_parts_mut(base) };
        let (pw, ph) = control.measured;
        let width = utils::coord_to_size(pw as i32);
        let height = utils::coord_to_size(ph as i32);
        unsafe { 
            self.base.widget.insert_column(index as i32); 
            //self.base.widget.horizontal_header_item(index as i32).set_text(QString::new().as_ref());
        }
        let this: &mut Table = unsafe { utils::base_to_impl_mut(member) };
        let indices = &[index];
        let mut item = adapter.adapter.spawn_item_view(indices, this);
        let col = unsafe { self.base.widget.horizontal_header_item(index as i32) }; 
        item.as_mut().map(|item| {
            let widget = unsafe { Ptr::from_raw(common::cast_control_to_qwidget_mut(item.as_mut())) };
            item.set_layout_width(layout::Size::Exact(width));
            item.set_layout_height(self.data.default_row_height);
            item.on_added_to_container(this, 0, 0, width, height);
            unsafe { 
                widget.static_upcast::<QObject>().set_property(PROPERTY_PARENT.as_ptr() as *const i8, &QVariant::from_u64(parent_ptr));
                widget.set_parent_1a(self.base.widget.horizontal_header().as_ptr());
                widget.show(); 
            }
        }).or_else(|| adapter.adapter.alt_text_at(indices).map(|value| unsafe { col.set_text(&QString::from_std_str(value)) }));
        self.data.cols.insert(index, Column {
            control: item,
            native: col,
            width: layout::Size::MatchParent,
        });
        self.resize_column(control, index, self.data.cols[index].width);
        self.data.rows.iter_mut().enumerate().for_each(|(row_index, row)| {
            row.cells.insert(index, None);
            this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().resize_row(control, row_index, row.height, true);
        });
    }
	fn add_cell_inner(&mut self, base: &mut MemberBase, x: usize, y: usize) {
        let parent_ptr = base as *mut MemberBase as u64;
        let (member, control, adapter, _) = unsafe { Table::adapter_base_parts_mut(base) };
        let (pw, ph) = control.measured;
        if self.data.rows.len() <= y {
            self.add_row_inner(member, y);
        }
        if self.data.cols.len() <= x {
            self.add_column_inner(member, x);
        }
        let this: &mut Table = unsafe { utils::base_to_impl_mut(member) };
        adapter.adapter.spawn_item_view(&[x, y], this).map(|mut item| {
            dbg!("cell at ", x, y);
            let item_widget = unsafe { Ptr::from_raw(common::cast_control_to_qwidget_mut(item.as_mut())) };
            unsafe {
                item_widget.static_upcast::<QObject>().set_property(PROPERTY_PARENT.as_ptr() as *const i8, &QVariant::from_u64(parent_ptr));
            }
            let widget = &self.base.widget;
            let mut width = unsafe { widget.column_width(x as i32) };
            self.data.rows.get_mut(y).map(|row| {
                unsafe { widget.set_cell_widget(y as i32, x as i32, item_widget); }
                item.set_layout_width(layout::Size::Exact(width as u16));
                item.set_layout_height(row.height);
                item.on_added_to_container(this, 0, 0, pw, ph);
                
                row.cells.insert(x, Some(Cell {
                    control: Some(item),
                    native: unsafe { widget.item_at_2a(y as i32, x as i32) },
                }));
                if row.cells.len() > x {
                    // facepalm
                    row.cells.remove(x+1);
                }
            });
        });
    }
	fn remove_column_inner(&mut self, member: &mut MemberBase, index: usize) {
        let this: &mut Table = unsafe { utils::base_to_impl_mut(member) };
        let widget = &self.base.widget;
        self.data.rows.iter_mut().enumerate().for_each(|(row_index, row)| {
            //this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().remove_cell_inner(member, row_index, index);
            let mut cell = if index < row.cells.len() { row.cells.remove(index) } else { None };
            cell.map(|cell| {
                cell.control.map(|mut control| control.on_removed_from_container(this));
                unsafe { 
                    widget.cell_widget(row_index as i32, index as i32).static_upcast::<QObject>().set_property(PROPERTY_PARENT.as_ptr() as *const i8, &QVariant::from_u64(0));
	                widget.remove_cell_widget(row_index as i32, index as i32); 
                }
            });
        });
        let column = if index < self.data.cols.len() { Some(self.data.cols.remove(index)) } else { None };
        column.map(|column| {
            column.control.map(|mut column| column.on_removed_from_container(this));
            unsafe {
                //column.native.static_upcast::<QObject>().set_property(PROPERTY_PARENT.as_ptr() as *const i8, &QVariant::from_u64(0)); 
                widget.remove_column(index as i32);
            }
        });
    }
    fn remove_cell_inner(&mut self, member: &mut MemberBase, x: usize, y: usize) {
        let this: &mut Table = unsafe { utils::base_to_impl_mut(member) };
        let widget = &self.base.widget;
        self.data.rows.get_mut(y).map(|row| {
            row.cells.remove(x).map(|mut cell| {
                cell.control.as_mut().map(|mut control| control.on_removed_from_container(this));
                unsafe { 
                    widget.cell_widget(y as i32, x as i32).static_upcast::<QObject>().set_property(PROPERTY_PARENT.as_ptr() as *const i8, &QVariant::from_u64(0)); 
                    widget.remove_cell_widget(y as i32, x as i32); 
                }
            });
            row.cells.insert(x, None);
        });
    }
    fn change_column_inner(&mut self, base: &mut MemberBase, index: usize) {
        self.remove_column_inner(base, index);
        self.add_column_inner(base, index);
    }
    fn change_cell_inner(&mut self, base: &mut MemberBase, x: usize, y: usize) {
        self.remove_cell_inner(base, x, y);
        self.add_cell_inner(base, x, y);
    }
    fn resize_row(&mut self, base: &ControlBase, index: usize, size: layout::Size, force: bool) {
        let (w, h) = base.measured;
        let height = match size {
            layout::Size::Exact(height) => height,
            layout::Size::WrapContent => self.data.rows.iter()
                    .flat_map(|row| row.cells.iter())
                    .filter(|cell| cell.is_some())
                    .map(|cell| cell.as_ref().unwrap().control.as_ref())
                    .filter(|control| control.is_some())
                    .map(|control| control.unwrap().size().1)
                    .fold(0, |s, i| if s > i {s} else {i}),
            layout::Size::MatchParent => base.measured.1 / self.data.cols.len() as u16,
        };
        unsafe { self.base.widget.set_row_height(index as i32, height as i32); }
        self.data.cols.iter_mut().for_each(|col| {
            col.control.as_mut().map(|control| {
                control.set_layout_height(layout::Size::Exact(height));
                control.measure(w, h);
                control.draw(None);
            });
        });
        self.data.rows.iter_mut().for_each(|row| {
            row.height = size;
            row.control.as_mut().map(|control| {
                control.set_layout_height(layout::Size::Exact(height));
                control.measure(w, h);
                control.draw(None);
            });
            row.cells.iter_mut().for_each(|cell| {
                cell.as_mut().map(|cell| {
                    cell.control.as_mut().map(|control| {
                        control.set_layout_height(layout::Size::Exact(height));
                        control.measure(w, h);
                        control.draw(None);
                    });
                });
            });
        });
        /*if force || self.data.default_row_height != size {
            
            if !force {
                self.data.row_at_mut(index).map(|row| row.height = size);
            }
        } else {
            let row_height = self.data.default_row_height;
            self.data.row_at_mut(index).map(|mut row| row.height = row_height);
        }*/
    }
    fn resize_column(&mut self, base: &ControlBase, index: usize, size: layout::Size) {
        let (w, h) = base.measured;
        let mut width = match size {
            layout::Size::Exact(width) => width,
            layout::Size::WrapContent => self.data.rows.iter()
                    .flat_map(|row| row.cells.iter())
                    .filter(|cell| cell.is_some())
                    .map(|cell| cell.as_ref().unwrap().control.as_ref())
                    .filter(|control| control.is_some())
                    .map(|control| control.unwrap().size().0)
                    .fold(0, |s, i| if s > i {s} else {i}),
            layout::Size::MatchParent => w / self.data.cols.len() as u16,
        };
        unsafe { self.base.widget.set_column_width(index as i32, width as i32); }
        self.data.column_at_mut(index).map(|col| {
            col.width = size;
            col.control.as_mut().map(|control| {
                control.set_layout_width(layout::Size::Exact(width));
                control.measure(w, h);
                control.draw(None);
            });
        });
        self.data.rows.iter_mut().for_each(|row| {
            row.cell_at_mut(index).map(|cell| {
                cell.control.as_mut().map(|control| {
                    control.set_layout_width(layout::Size::Exact(width));
                    control.measure(w, h);
                    control.draw(None);
                });
            });
        });
    }
}

impl<O: controls::Table> NewTableInner<O> for QtTable {
    fn with_uninit_params(ptr: &mut mem::MaybeUninit<O>, width: usize, height: usize) -> Self {
        let mut ll = QtTable {
            base: common::QtControlBase::with_params(unsafe { QTableWidget::new_0a() }, event_handler::<O>),
            data: Default::default(),
            h_left_clicked: (
                None, 
                unsafe { SlotOfIntInt::new(NullPtr, move |_,_| {}) },
                unsafe { SlotOfInt::new(NullPtr, move |_| {}) }
            ),
            headers_moved_slot: unsafe { SlotOfInt::new(NullPtr, move |_| {})},
            headers_resized_slot: unsafe { SlotOfInt::new(NullPtr, move |_| {})},
        };
        unsafe {
            let ptr = ptr as *const _ as u64;
            let obj = ll.base.widget.static_upcast::<QObject>().as_mut_raw_ptr();
            ll.h_left_clicked.1 = SlotOfIntInt::new(NullPtr, move |row, col| {
                let this = cast_qobject_to_uimember_mut::<Table>(&mut *obj).unwrap();
                this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().data.cell_at_mut(&[row as usize, col as usize]).and_then(|cell| cell.control.as_mut()).map(|mut control| {
                    let this = cast_qobject_to_uimember_mut::<Table>(&mut *obj).unwrap();
                    if let Some(ref mut cb) = this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().h_left_clicked.0 {
                        let this = cast_qobject_to_uimember_mut::<O>(&mut *obj).unwrap();
                        (cb.as_mut())(this, &[col as usize, row as usize], control.as_control_mut());
                    }
                });
            });
            ll.h_left_clicked.2 = SlotOfInt::new(NullPtr, move |col| {
                let this = cast_qobject_to_uimember_mut::<Table>(&mut *obj).unwrap();
                this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().data.column_at_mut(col as usize).and_then(|col| col.control.as_mut()).map(|mut control| {
                    let this = cast_qobject_to_uimember_mut::<Table>(&mut *obj).unwrap();
                    if let Some(ref mut cb) = this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().h_left_clicked.0 {
                        let this = cast_qobject_to_uimember_mut::<O>(&mut *obj).unwrap();
                        (cb.as_mut())(this, &[col as usize], control.as_control_mut());
                    }
                });
            });
            ll.headers_resized_slot = SlotOfInt::new(NullPtr, move |i| {
                let this = cast_qobject_to_uimember_mut::<Table>(&mut *obj).unwrap();
                let header = this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().base.widget.horizontal_header();
                (header.visual_index(i)..header.count()).for_each(|j| {
                    let logical = header.logical_index(j);
                    this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().data.column_at_mut(logical as usize).and_then(|col| col.control.as_mut()).map(|col| {
                        //col.as_control_mut().set_size(header.section_size(logical) as u16 - 2 - 2 - 1, header.height() as u16 - 2 - 2 - 1);
                        let item_widget = unsafe { Ptr::from_raw(common::cast_control_to_qwidget_mut(col.as_control_mut())) };
                        item_widget.set_geometry_4a(
                            header.section_viewport_position(logical) + 2, 
                            2, 
                            header.section_size(logical) - 2 - 2 - 1, 
                            header.height() - 2 - 2 - 1
                        );
                    });
                });
            });
            ll.headers_moved_slot = SlotOfInt::new(NullPtr, move |logical| {
                let this = cast_qobject_to_uimember_mut::<Table>(&mut *obj).unwrap();
                let header = this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().base.widget.horizontal_header();
                (0..header.count()).for_each(|j| {
                    this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().data.column_at_mut(logical as usize).and_then(|col| col.control.as_mut()).map(|col| {
                        
                        let item_widget = unsafe { Ptr::from_raw(common::cast_control_to_qwidget_mut(col.as_control_mut())) };
                        item_widget.set_geometry_4a(
                            header.section_viewport_position(logical) + 2, 
                            2, 
                            header.section_size(logical) - 2 - 2 - 1, 
                            header.height() - 2 - 2 - 1
                        );
                    });
                });
            });
            ll.base.widget.cell_clicked().connect(&ll.h_left_clicked.1);
            ll.base.widget.horizontal_header().section_clicked().connect(&ll.h_left_clicked.2);
            ll.base.widget.horizontal_header().section_resized().connect(&ll.headers_resized_slot);
            ll.base.widget.horizontal_header().section_moved().connect(&ll.headers_moved_slot);
            ll.base.widget.vertical_header().hide();
            ll.base.widget.set_vertical_scroll_bar_policy(ScrollBarPolicy::ScrollBarAlwaysOn);
            ll.base.widget.set_horizontal_scroll_bar_policy(ScrollBarPolicy::ScrollBarAlwaysOn);
            ll.base.widget.set_selection_mode(::qt_widgets::q_abstract_item_view::SelectionMode::NoSelection);
            ll.base.widget.set_show_grid(false);
            let qo = ll.base.widget.static_upcast::<QObject>();
            qo.set_property(PROPERTY.as_ptr() as *const i8, &QVariant::from_u64(ptr));
        }
        ll
    }
}
impl TableInner for QtTable {
    fn with_adapter_initial_size(adapter: Box<dyn types::Adapter>, width: usize, height: usize) -> Box<dyn controls::Table> {
        let mut b: Box<mem::MaybeUninit<Table>> = Box::new_uninit();
        let ab = AMember::with_inner(
            AControl::with_inner(
                AContainer::with_inner(
                    AAdapted::with_inner(
                        ATable::with_inner(
                            <Self as NewTableInner<Table>>::with_uninit_params(b.as_mut(), width, height)
                        ),
                        adapter,
                        &mut b,
                    ),
                )
            ),
        );
        let mut bb = unsafe {
	        b.as_mut_ptr().write(ab);
	        b.assume_init()
        };
        let (member, _, adapter, table) = unsafe { Table::adapter_base_parts_mut(&mut bb.base) };
        adapter.adapter.for_each(&mut (|indexes, node| {
            match node {
                adapter::Node::Leaf => table.inner_mut().add_cell_inner(member, indexes[0], indexes[1]),
                adapter::Node::Branch(_) => table.inner_mut().add_column_inner(member, indexes[0])
            }
        }));
        bb
    }
    fn headers_visible(&self, _: &MemberBase, _: &ControlBase, _: &AdaptedBase) -> bool {
        unsafe { self.base.widget.horizontal_header().is_visible() }
    }
    fn set_headers_visible(&mut self, _: &mut MemberBase, _: &mut ControlBase, _: &mut AdaptedBase, visible: bool) {
        unsafe { self.base.widget.horizontal_header().set_visible(visible); }
    }
    fn set_column_width(&mut self, _: &mut MemberBase, control: &mut ControlBase, _: &mut AdaptedBase, index: usize, size: layout::Size) {
        self.resize_column(control, index, size)
    }
    fn set_row_height(&mut self, _: &mut MemberBase, control: &mut ControlBase, _: &mut AdaptedBase, index: usize, size: layout::Size) {
        self.resize_row(control, index, size, false)
    }
/*    fn resize(&mut self, member: &mut MemberBase, control: &mut ControlBase, adapted: &mut AdaptedBase, width: usize, height: usize) -> (usize, usize) {
        let old_size = self.size(member, control, adapted);
        let (max_width, max_height) = (cmp::max(width, old_size.0), cmp::max(height, old_size.1));
        let (min_width, min_height) = (cmp::min(width, old_size.0), cmp::min(height, old_size.1));
        (min_height..max_height).rev().for_each(|x| 
            if self.data.rows.len() > x {
                if old_size.0 > x {
                    self.remove_row_inner(member, x);
                }
            } else {
                if old_size.0 < x {
                     self.add_row_inner(member, x);
                }
            }
        );
        (min_width..max_width).rev().for_each(|y| 
            if self.data.cols.len() > y {
                if old_size.0 > y {
                    self.remove_column_inner(member, y);
                }
            } else {
                if old_size.0 < y {
                     self.add_column_inner(member, y, false);
                }
            }
        );
        old_size
    }*/
}
impl AdaptedInner for QtTable {
	fn on_item_change(&mut self, base: &mut MemberBase, value: adapter::Change) {
		match value {
            adapter::Change::Added(at, node) => {
                if adapter::Node::Leaf == node || at.len() > 1 {
                    self.add_cell_inner(base, at[0], at[1]);
                } else {
                    self.add_column_inner(base, at[0]);
                }
            },
            adapter::Change::Removed(at) => {
                if at.len() > 1 {
                    self.remove_cell_inner(base, at[0], at[1]);
                } else {
                    self.remove_column_inner(base, at[0]);
                }
            },
            adapter::Change::Edited(at, node) => {
                if adapter::Node::Leaf == node || at.len() > 1 {
                    self.change_cell_inner(base, at[0], at[1]);
                } else {
                    self.change_column_inner(base, at[0]);
                }
            },
        }
        self.base.invalidate();
	}
}
impl HasNativeIdInner for QtTable {
    type Id = common::QtId;

    fn native_id(&self) -> Self::Id {
        QtId::from(unsafe { self.base.widget.static_upcast::<QObject>().as_raw_ptr() } as *mut QObject)
    }
}
impl HasVisibilityInner for QtTable {
    fn on_visibility_set(&mut self, _: &mut MemberBase, value: types::Visibility) -> bool {
        self.base.set_visibility(value);
        self.base.invalidate()
    }
}
impl HasSizeInner for QtTable {
    fn on_size_set(&mut self, _: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        unsafe { self.base.widget.set_fixed_size_2a(width as i32, height as i32); }
        true
    }
}
impl MemberInner for QtTable {}

impl Drawable for QtTable {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(member, control);
    }
    fn measure(&mut self, _: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width,
                    layout::Size::Exact(w) => w,
                    layout::Size::WrapContent => defaults::THE_ULTIMATE_ANSWER_TO_EVERYTHING,
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height,
                    layout::Size::Exact(h) => h,
                    layout::Size::WrapContent => defaults::THE_ULTIMATE_ANSWER_TO_EVERYTHING,
                };
                (cmp::max(0, w as i32) as u16, cmp::max(0, h as i32) as u16)
            }
        };
        self.base.dirty = control.measured != old_size;
        (control.measured.0, control.measured.1, self.base.dirty)
    }
    fn invalidate(&mut self, _member: &mut MemberBase, _control: &mut ControlBase) {
        self.base.invalidate();
    }
}

impl HasLayoutInner for QtTable {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
    fn layout_margin(&self, _member: &MemberBase) -> layout::BoundarySize {
        unsafe {
            let margins = self.base.widget.contents_margins();
            layout::BoundarySize::Distinct(margins.left(), margins.top(), margins.right(), margins.bottom())
        }
    }
}

impl ControlInner for QtTable {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        control.coords = Some((x, y));
        self.measure(member, control, pw, ph);

        let this: &mut Table = unsafe { utils::base_to_impl_mut(member) };
        self.data.cols.iter_mut().enumerate().for_each(|(index, col)| {
            //col.control.as_mut().map(|control| set_parent(control.as_mut(), Some(&parent)));
            col.control.as_mut().map(|mut control| control.on_added_to_container(this, 0, 0, pw, ph));
            this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().resize_column(control, index, col.width);
        });
        self.data.rows.iter_mut().enumerate().for_each(|(index, row)| {
            this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().resize_row(control, index, row.height, false);
            //row.control.as_mut().map(|control| set_parent(control.as_mut(), Some(this)));
            row.cells.iter_mut()
                .filter(|cell| cell.is_some())
                .for_each(|cell| {
                    cell.as_mut().and_then(|cell| cell.control.as_mut())
                        .map(|control| control.on_added_to_container(this, 0, 0, pw, ph));
                });
        });
    }
    fn on_removed_from_container(&mut self, member: &mut MemberBase, _control: &mut ControlBase, _parent: &dyn controls::Container) {
        let this: &mut Table = unsafe { utils::base_to_impl_mut(member) };
        self.data.cols.iter_mut().enumerate().for_each(|(index, col)| {
            col.control.as_mut().map(|control| control.on_removed_from_container(this));
        });
        self.data.rows.iter_mut().enumerate().for_each(|(index, row)| {
            row.cells.iter_mut()
                .filter(|cell| cell.is_some())
                .for_each(|cell| {
                    cell.as_mut().unwrap().control.as_mut()
                        .map(|control| control.on_removed_from_container(this));
                });
        });
    }

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
    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, member: &mut MemberBase, control: &mut ControlBase, mberarkup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry) {
        use plygui_api::markup::MEMBER_TYPE_table;

        fill_from_markup_base!(self, markup, registry, Table, [MEMBER_ID_layout_linear, MEMBER_TYPE_table]);
        fill_from_markup_items!(self, markup, registry);
    }
}

impl ContainerInner for QtTable {
    fn find_control_mut<'a>(&'a mut self, arg: types::FindBy<'a>) -> Option<&'a mut dyn controls::Control> {
        for column in self.data.cols.as_mut_slice() {
            let maybe = column.control.as_mut().and_then(|control| utils::find_by_mut(control.as_mut(), arg));
            if maybe.is_some() {
                return maybe;
            }
        }
        for row in self.data.rows.as_mut_slice() {
            for cell in row.cells.as_mut_slice() {
                if let Some(cell) = cell {
                    let maybe = cell.control.as_mut().and_then(|control| utils::find_by_mut(control.as_mut(), arg));
                    if maybe.is_some() {
                        return maybe;
                    }
                }
            }
        }
        None
    }
    fn find_control<'a>(&'a self, arg: types::FindBy<'a>) -> Option<&'a dyn controls::Control> {
        for column in self.data.cols.as_slice() {
            let maybe = column.control.as_ref().and_then(|control| utils::find_by(control.as_ref(), arg));
            if maybe.is_some() {
                return maybe;
            }
        }
        for row in self.data.rows.as_slice() {
            for cell in row.cells.as_slice() {
                if let Some(cell) = cell {
                    let maybe = cell.control.as_ref().and_then(|control| utils::find_by(control.as_ref(), arg));
                    if maybe.is_some() {
                        return maybe;
                    }
                }
            }
        }
        None
    }
}
impl Spawnable for QtTable {
    fn spawn() -> Box<dyn controls::Control> {
        Self::with_adapter(Box::new(types::imp::StringVecAdapter::<crate::imp::Text>::new())).into_control()
    }
}
/*
impl Drop for QtTable {
	fn drop(&mut self) {
		for item in self.items {
            unsafe {
                ptr::write(&mut item.1, common::MaybeCppBox::None);
            }
    	}
	}
}
*/
fn event_handler<O: controls::Table>(object: &mut QObject, event: &mut QEvent) -> bool {
    match unsafe { event.type_() } {
        QEventType::Resize => {
            if let Some(this) = cast_qobject_to_uimember_mut::<Table>(object) {
                let size = unsafe { 
                    let size = Ref::from_raw(event).unwrap().static_downcast::<QResizeEvent>();
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
        QEventType::Destroy => {
            if let Some(ll) = cast_qobject_to_uimember_mut::<Table>(object) {
            	for col in ll.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().data.cols.as_mut_slice() {
	                unsafe {
	                    ptr::write(&mut col.native, Ptr::null());
	                }
            	}
                for row in ll.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().data.rows.as_mut_slice() {
	                for cell in row.cells.as_mut_slice() {
                        cell.as_mut().map(|ref mut cell| unsafe {
                            ptr::write(&mut cell.native, Ptr::null());
                        });
                    }
                    unsafe {
	                    ptr::write(&mut row.native, Ptr::null());
	                }
            	}
            }
        }
        _ => {}
    }
    false
}
