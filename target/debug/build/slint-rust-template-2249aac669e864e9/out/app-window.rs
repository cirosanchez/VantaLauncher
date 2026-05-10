mod slint_generatedAppWindow {
     # ! [allow (non_snake_case , non_camel_case_types)] # ! [allow (unused_braces , unused_parens)] # ! [allow (clippy :: all , clippy :: pedantic , clippy :: nursery)] # ! [allow (unknown_lints , if_let_rescope , tail_expr_drop_order)] use slint :: private_unstable_api :: re_exports as sp ;
     # [allow (unused_imports)] use sp :: {
         RepeatedItemTree as _ , ModelExt as _ , Model as _ , Float as _ }
     ;
     const _THE_SAME_VERSION_MUST_BE_USED_FOR_THE_COMPILER_AND_THE_RUNTIME : slint :: VersionCheck_1_16_1 = slint :: VersionCheck_1_16_1 ;
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerFluentPalette_20 {
         r#accent_background : sp :: Property < slint :: Brush > , r#background : sp :: Property < slint :: Brush > , r#color_scheme : sp :: Property < sp :: r#ColorScheme > , r#dark_color_scheme : sp :: Property < bool > , r#foreground : sp :: Property < slint :: Brush > , globals : sp :: OnceCell < sp :: Weak < SharedGlobals >> , }
     impl InnerFluentPalette_20 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , globals : & sp :: Rc < SharedGlobals >) {
             # ! [allow (unused)] let _ = self . globals . set (sp :: Rc :: downgrade (globals)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#accent_background () }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self) . get () {
                         (_self . r#fn_accentify (sp :: Color :: from_argb_encoded ((4284534271f64) as u32) as _)) as _ }
                     else {
                         _self . r#fn_accentify (sp :: Color :: from_argb_encoded ((4278214584f64) as u32) as _) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#background () }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self) . get () {
                         (sp :: Color :: from_argb_encoded ((4280032284f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4294638330f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#color_scheme () }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () . window ()) . color_scheme ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#dark_color_scheme () }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     ({
                         let r#tmp_FluentPalette_20_color_scheme = {
                             * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#color_scheme () }
                         . apply_pin (_self) . get () ;
                         if ! ((((r#tmp_FluentPalette_20_color_scheme) . clone ())) == (((sp :: r#ColorScheme :: r#Unknown) . clone ()))) {
                             (((((r#tmp_FluentPalette_20_color_scheme) . clone ())) == (((sp :: r#ColorScheme :: r#Dark) . clone ())))) as _ }
                         else {
                             (((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () . window ()) . color_scheme ()) . clone ())) == (((sp :: r#ColorScheme :: r#Dark) . clone ())) }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#foreground () }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self) . get () {
                         (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                    )) as _ }
                ) ;
                 }
             }
         # [allow (dead_code , unused)] pub fn r#fn_accentify (self : :: core :: pin :: Pin < & Self > , arg_0 : sp :: Color ,) -> sp :: Color {
             let _self = self ;
             let args = (arg_0 ,) ;
             ({
                 let r#local_accent_color = sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () . window ()) . accent_color () ;
                 if ! ((((((r#local_accent_color) . clone () . to_argb_u8 ()) . r#alpha) . clone ()) as f64) > (((0f64) . clone ()) as f64)) {
                     (args . 0 . clone ()) as _ }
                 else {
                     {
                         let r#local_default_lch = (args . 0 . clone ()) . clone () . to_oklch () ;
                         let r#local_accent_lch = (r#local_accent_color) . clone () . to_oklch () ;
                         {
                             let l : f32 = (((r#local_default_lch) . r#lightness) . clone () as f32) . max (0.) . min (1.) as f32 ;
                             let c : f32 = (((r#local_accent_lch) . r#chroma) . clone () as f32) . max (0.) as f32 ;
                             let alpha : f32 = ((1f64) . clone () as f32) . max (0.) . min (1.) as f32 ;
                             sp :: Color :: from_oklch (l , c , ((r#local_accent_lch) . r#hue) . clone () as f32 , alpha) }
                         }
                     }
                 }
            ) as _ }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerFocusBorder_root_1 {
         r#root_1 : sp :: r#BasicBorderRectangle , r#rectangle_2 : sp :: r#BasicBorderRectangle , r#root_1_height : sp :: Property < sp :: LogicalLength > , r#root_1_width : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerFocusBorder_root_1 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerFocusBorder_root_1 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                    )) as _ }
                ) ;
                 }
             (InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#rectangle_2 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if {
                         * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#dark_color_scheme () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((3003121664f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#rectangle_2 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((2f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#rectangle_2 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#rectangle_2 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#rectangle_2 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info ((InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 1u32 => ((((((((InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((4f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , (((((((InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((4f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((2f64) . clone ()) . clone () as sp :: Coord , ((2f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerButton_root_3 {
         r#root_3 : sp :: r#Empty , r#i_background_4 : sp :: r#BasicBorderRectangle , r#i_border_5 : sp :: r#BasicBorderRectangle , r#i_touch_area_11 : sp :: r#TouchArea , r#i_focus_scope_12 : sp :: r#FocusScope , r#root_3_checked : sp :: Property < bool > , r#root_3_has_focus : sp :: Property < bool > , r#root_3_height : sp :: Property < sp :: LogicalLength > , r#root_3_i_background_4_width : sp :: Property < sp :: LogicalLength > , r#root_3_i_layout_6_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_3_i_layout_6_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_3_i_layout_6_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_3_icon : sp :: Property < sp :: Image > , r#root_3_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_3_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_3_pressed : sp :: Property < bool > , r#root_3_state : sp :: Property < i32 > , r#root_3_text : sp :: Property < sp :: SharedString > , r#root_3_text_color : sp :: Property < slint :: Brush > , r#root_3_width : sp :: Property < sp :: LogicalLength > , r#root_3_x : sp :: Property < sp :: LogicalLength > , r#root_3_y : sp :: Property < sp :: LogicalLength > , r#root_3_accessible_action_default : sp :: Callback < () , () > , r#root_3_clicked : sp :: Callback < () , () > , repeater0 : sp :: Conditional < InnerComponent_image_7 > , repeater1 : sp :: Conditional < InnerComponent_text_9 > , repeater2 : sp :: Conditional < InnerComponent_focusborder_13 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerButton_root_3 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerButton_root_3 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (((((((((((sp :: Image :: default ()) . clone () . size ()) . r#width) . clone ()) as f64) > (((0f64) . clone ()) as f64))) . clone ())) && (((((((((sp :: Image :: default ()) . clone () . size ()) . r#height) . clone ()) as f64) > (((0f64) . clone ()) as f64))) . clone ())))) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((((((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (_self) . get ()) . clone ())) != (((sp :: SharedString :: from ("")) . clone ())))) as _ }
                 }
            ) ;
             _self . repeater2 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((((((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_has_focus ()) . apply_pin (_self) . get ()) . clone ())) && ((((InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get ()) . clone ())))) as _ }
                 }
            ) ;
             sp :: Property :: link_two_way ((InnerButton_root_3 :: FIELD_OFFSETS . r#i_touch_area_11 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) , (InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self)) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_accessible_action_default ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerButton_root_3 :: FIELD_OFFSETS . r#i_touch_area_11 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_has_focus ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#has_focus ()) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_background_4_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_width ()) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         4usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_3 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_7 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         r#repeated_indices [0usize] = r#items_vec . len () as u32 ;
                         r#repeated_indices [0usize + 1] = _self . repeater0 . len () as u32 ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             }
                         InnerButton_root_3 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_9 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         r#repeated_indices [2usize] = r#items_vec . len () as u32 ;
                         r#repeated_indices [2usize + 1] = _self . repeater1 . len () as u32 ;
                         for i in 0 .. _self . repeater1 . len () {
                             if let Some (sub_comp) = _self . repeater1 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : (sp :: r#LayoutAlignment :: r#Center) . clone () as _ , r#cells : (r#cells) . clone () as _ , r#padding : ({
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = (12f64) . clone () as _ ;
                                 the_struct . r#end = (12f64) . clone () as _ ;
                                 the_struct }
                            ) . clone () as _ , r#size : ((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_background_4_width ()) . apply_pin (_self) . get () . get ()) . clone () as _ , r#spacing : (4f64) . clone () as _ , }
                         as _ , r#repeated_indices as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_3 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_7 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             }
                         InnerButton_root_3 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_9 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         for i in 0 .. _self . repeater1 . len () {
                             if let Some (sub_comp) = _self . repeater1 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Horizontal , None)) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells as _ , 4f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (12f64) . clone () as _ ;
                             the_struct . r#end = (12f64) . clone () as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Center as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_3 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_7 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         for i in 0 .. _self . repeater0 . len () {
                             if let Some (sub_comp) = _self . repeater0 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                                 }
                             }
                         InnerButton_root_3 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_9 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         for i in 0 .. _self . repeater1 . len () {
                             if let Some (sub_comp) = _self . repeater1 . instance_at (i) {
                                 items_vec . push (sub_comp . as_pin_ref () . layout_item_info (sp :: Orientation :: Vertical , None)) ;
                                 }
                             }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (5f64) . clone () as _ ;
                             the_struct . r#end = (5f64) . clone () as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let r#layout_info_0 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = ((r#layout_info_0) . r#max) . clone () as _ ;
                             the_struct . r#max_percent = ((r#layout_info_0) . r#max_percent) . clone () as _ ;
                             the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_h ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                             the_struct . r#min_percent = ((r#layout_info_0) . r#min_percent) . clone () as _ ;
                             the_struct . r#preferred = ((r#layout_info_0) . r#preferred) . clone () as _ ;
                             the_struct . r#stretch = (0f64) . clone () as _ ;
                             the_struct }
                         }
                    ) . clone ())) + ((((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                         the_struct . r#min = (0f64) . clone () as _ ;
                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                         the_struct . r#preferred = (0f64) . clone () as _ ;
                         the_struct . r#stretch = (1f64) . clone () as _ ;
                         the_struct }
                    ) . clone ())) + ((((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let r#layout_info_1 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                             the_struct . r#max_percent = (100f64) . clone () as _ ;
                             the_struct . r#min = (0f64) . clone () as _ ;
                             the_struct . r#min_percent = (0f64) . clone () as _ ;
                             the_struct . r#preferred = (0f64) . clone () as _ ;
                             the_struct . r#stretch = (1f64) . clone () as _ ;
                             the_struct }
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = ((r#layout_info_1) . r#max) . clone () as _ ;
                             the_struct . r#max_percent = ((r#layout_info_1) . r#max_percent) . clone () as _ ;
                             the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                             the_struct . r#min_percent = ((r#layout_info_1) . r#min_percent) . clone () as _ ;
                             the_struct . r#preferred = ((r#layout_info_1) . r#preferred) . clone () as _ ;
                             the_struct . r#stretch = (0f64) . clone () as _ ;
                             the_struct }
                         }
                    ) . clone ())) + ((((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (340282346638528860000000000000000000000f64) . clone () as _ ;
                         the_struct . r#max_percent = (100f64) . clone () as _ ;
                         the_struct . r#min = (0f64) . clone () as _ ;
                         the_struct . r#min_percent = (0f64) . clone () as _ ;
                         the_struct . r#preferred = (0f64) . clone () as _ ;
                         the_struct . r#stretch = (1f64) . clone () as _ ;
                         the_struct }
                    ) . clone ())) + ((((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_pressed ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((((InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get ()) . clone ())) && ((((InnerButton_root_3 :: FIELD_OFFSETS . r#i_touch_area_11 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed ()) . apply_pin (_self) . get ()) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_state ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ! (InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_pressed ()) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             if (InnerButton_root_3 :: FIELD_OFFSETS . r#i_touch_area_11 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover ()) . apply_pin (_self) . get () {
                                 (3f64) as _ }
                             else {
                                 if (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked ()) . apply_pin (_self) . get () {
                                     (4f64) as _ }
                                 else {
                                     0f64 }
                                 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_3_state = (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_state ()) . apply_pin (_self) . get () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_3_state) . clone () as f64) , & ((1f64) . clone () as f64)) {
                             (if (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked ()) . apply_pin (_self) . get () {
                                 (slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((2281701375f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                                )) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((1593835519f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1577058304f64) as u32) }
                                ) }
                            ) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_3_state) . clone () as f64) , & ((2f64) . clone () as f64)) {
                                 (if (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked ()) . apply_pin (_self) . get () {
                                     (slint :: Brush :: SolidColor (if {
                                         * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((2147483648f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((3019898879f64) as u32) }
                                    )) as _ }
                                 else {
                                     slint :: Brush :: SolidColor (if {
                                         * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((3388997631f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((2566914048f64) as u32) }
                                    ) }
                                ) as _ }
                             else {
                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_3_state) . clone () as f64) , & ((4f64) . clone () as f64)) {
                                     (slint :: Brush :: SolidColor (if {
                                         * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((4278190080f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                                    )) as _ }
                                 else {
                                     if (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked ()) . apply_pin (_self) . get () {
                                         (slint :: Brush :: SolidColor (if {
                                             * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((4278190080f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                                        )) as _ }
                                     else {
                                         slint :: Brush :: SolidColor (if {
                                             * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                                        ) }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerButton_root_3 :: FIELD_OFFSETS . r#i_background_4 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_3_state = (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_state ()) . apply_pin (_self) . get () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_3_state) . clone () as f64) , & ((1f64) . clone () as f64)) {
                             (if (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked ()) . apply_pin (_self) . get () {
                                 (slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((704643071f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((939524096f64) as u32) }
                                )) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((184549375f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1308228089f64) as u32) }
                                ) }
                            ) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_3_state) . clone () as f64) , & ((2f64) . clone () as f64)) {
                                 (if (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked ()) . apply_pin (_self) . get () {
                                     (({
                                         * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#accent_background () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get ()) . clone () . with_alpha ((0.8f64) . clone () as f32)) as _ }
                                 else {
                                     slint :: Brush :: SolidColor (if {
                                         * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((150994943f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((1308228089f64) as u32) }
                                    ) }
                                ) as _ }
                             else {
                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_3_state) . clone () as f64) , & ((3f64) . clone () as f64)) {
                                     (if (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked ()) . apply_pin (_self) . get () {
                                         (({
                                             * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#accent_background () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get ()) . clone () . with_alpha ((0.9f64) . clone () as f32)) as _ }
                                     else {
                                         slint :: Brush :: SolidColor (if {
                                             * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((2163866105f64) as u32) }
                                        ) }
                                    ) as _ }
                                 else {
                                     if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_3_state) . clone () as f64) , & ((4f64) . clone () as f64)) {
                                         ({
                                             * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#accent_background () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get ()) as _ }
                                     else {
                                         slint :: Brush :: SolidColor (if {
                                             * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                         . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((268435455f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((3019898879f64) as u32) }
                                        ) }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: Linear) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             (InnerButton_root_3 :: FIELD_OFFSETS . r#i_background_4 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_3 :: FIELD_OFFSETS . r#i_border_5 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_3_state = (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_state ()) . apply_pin (_self) . get () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_3_state) . clone () as f64) , & ((1f64) . clone () as f64)) {
                             (if (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked ()) . apply_pin (_self) . get () {
                                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32))) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                                ) }
                            ) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_3_state) . clone () as f64) , & ((2f64) . clone () as f64)) {
                                 (slint :: Brush :: SolidColor (if {
                                     * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                 . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                                )) as _ }
                             else {
                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& ((r#tmp_root_3_state) . clone () as f64) , & ((4f64) . clone () as f64)) {
                                     (if {
                                         * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get () {
                                         (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((352321535f64) as u32) , position : 0.9067000000000001f64 as _ }
                                         , sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((603979776f64) as u32) , position : 1f64 as _ }
                                        ]))) as _ }
                                     else {
                                         slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((352321535f64) as u32) , position : 0.9067000000000001f64 as _ }
                                         , sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((1711276032f64) as u32) , position : 1f64 as _ }
                                        ])) }
                                    ) as _ }
                                 else {
                                     if {
                                         * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#dark_color_scheme () }
                                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get () {
                                         (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((402653183f64) as u32) , position : 0f64 as _ }
                                         , sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((301989888f64) as u32) , position : 0.0833f64 as _ }
                                        ]))) as _ }
                                     else {
                                         slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((251658240f64) as u32) , position : 0.9058f64 as _ }
                                         , sp :: GradientStop {
                                             color : sp :: Color :: from_argb_encoded ((687865856f64) as u32) , position : 1f64 as _ }
                                        ])) }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             (InnerButton_root_3 :: FIELD_OFFSETS . r#i_border_5 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerButton_root_3 :: FIELD_OFFSETS . r#i_border_5 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerButton_root_3 :: FIELD_OFFSETS . r#i_touch_area_11 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if false {
                                 ({
                                     (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked ()) . apply_pin (_self) . set ((! (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked ()) . apply_pin (_self) . get ()) as _) }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             ;
                             (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_clicked ()) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             (InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             (InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation ()) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#key_pressed ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ! (((((((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from (" ")) . clone ())))) . clone ())) || ((((((((args . 0 . clone ()) . r#text) . clone ())) == (((sp :: SharedString :: from ("\n")) . clone ())))) . clone ()))) {
                                 ({
                                     {
                                         sp :: r#EventResult :: r#Reject }
                                     }
                                ) as _ }
                             else {
                                 {
                                     (InnerButton_root_3 :: FIELD_OFFSETS . r#i_touch_area_11 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked ()) . apply_pin (_self) . call (& ()) ;
                                     sp :: r#EventResult :: r#Accept }
                                 }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_icon ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_3 :: FIELD_OFFSETS . r#i_background_4 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_3 :: FIELD_OFFSETS . r#i_background_4 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_3 :: FIELD_OFFSETS . r#i_background_4 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_3 :: FIELD_OFFSETS . r#i_border_5 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_3 :: FIELD_OFFSETS . r#i_border_5 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_3 :: FIELD_OFFSETS . r#i_border_5 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_3 :: FIELD_OFFSETS . r#i_touch_area_11 () + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click ()) . apply_pin (_self) . set_constant () ;
             (InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_3 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent_image_7 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 => {
                     InnerButton_root_3 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| InnerComponent_text_9 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater1 . visit (order , visitor) }
                 2u32 => {
                     InnerButton_root_3 :: FIELD_OFFSETS . repeater2 () . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_13 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater2 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_h ()) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_h ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = (0f64) . clone () as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v ()) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = (0f64) . clone () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_3 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent_image_7 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 => {
                     InnerButton_root_3 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| InnerComponent_text_9 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 2u32 => {
                     InnerButton_root_3 :: FIELD_OFFSETS . repeater2 () . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_13 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater2 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_3 :: FIELD_OFFSETS . repeater0 () . apply_pin (_self) . ensure_updated (|| InnerComponent_image_7 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 => {
                     InnerButton_root_3 :: FIELD_OFFSETS . repeater1 () . apply_pin (_self) . ensure_updated (|| InnerComponent_text_9 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater1 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 2u32 => {
                     InnerButton_root_3 :: FIELD_OFFSETS . repeater2 () . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_13 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater2 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_x ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_y ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord ,) , 1u32 => ((((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 2u32 => ((((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 3u32 => ((((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 5u32 => ((((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Button , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if (InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ((InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (_self) . get ()) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , sp :: AccessibilityAction :: r#Default) => {
                     (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_accessible_action_default ()) . apply_pin (_self) . call (& ()) }
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: SupportedAccessibilityAction :: r#Default , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_image_7 {
         r#image_7 : sp :: r#ImageItem , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_image_7 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_3 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_image_7 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 let _ = _self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_icon ()) . apply_pin (x . as_pin_ref ())) . map (| x | sp :: Property :: link_two_way ((InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) , x)) ;
                 }
             ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - (((5f64) . clone ()) as f64))) . clone ()) as f64) - (((5f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             (InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             (InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#source ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 () + sp :: r#ImageItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = sp :: Item :: layout_info ((InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (20f64) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = (20f64) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = sp :: Item :: layout_info ((InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                         the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                         the_struct . r#min = (20f64) . clone () as _ ;
                         the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                         the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                         the_struct . r#stretch = ((r#layout_info) . r#stretch) . clone () as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - (((5f64) . clone ()) as f64))) . clone ()) as f64) - (((5f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((20f64) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [0usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default ()) . clone ()) . clone () as sp :: Coord , ((5f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Image , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_image_7 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_3 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_3 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_image_7 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_image_7 :: FIELD_OFFSETS . r#image_7 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_image_7) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_image_7 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_image_7 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_image_7 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_image_7 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 6u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_image_7 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_image_7 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn layout_item_info (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation , _child_index : sp :: Option < usize > ,) -> sp :: LayoutItemInfo {
             sp :: LayoutItemInfo {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_text_9 {
         r#text_9 : sp :: r#SimpleText , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_text_9 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_3 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_text_9 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding ((InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text_color ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                 , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut the_struct = sp :: PropertyAnimation :: default () ;
                         the_struct . r#delay = (0f64) . clone () as _ ;
                         the_struct . r#direction = (sp :: r#AnimationDirection :: r#Normal) . clone () as _ ;
                         the_struct . r#duration = (150f64) . clone () as _ ;
                         the_struct . r#easing = (sp :: EasingCurve :: Linear) . clone () as _ ;
                         the_struct . r#iteration_count = (1f64) . clone () as _ ;
                         the_struct }
                     , None) }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1.0766f64) . clone ()) as f64) * (((sp :: WindowItem :: resolved_default_font_size (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ())) . get ()) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set ({
                 (((400f64) as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - (((5f64) . clone ()) as f64))) . clone ()) as f64) - (((5f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             (InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info ((InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((((((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) as f64) - (((5f64) . clone ()) as f64))) . clone ()) as f64) - (((5f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default ()) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layout_cache ()) . apply_pin (x . as_pin_ref ())) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [2usize] as usize) + 0f64 as usize * 2usize) . unwrap_or (& (0 as _)) }
                ) . unwrap_or_default ()) . clone ()) . clone () as sp :: Coord , ((5f64) . clone ()) . clone () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_text_9 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_3 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_3 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_text_9 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_text_9 :: FIELD_OFFSETS . r#text_9 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_text_9) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_text_9 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_text_9 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_text_9 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_text_9 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 7u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_text_9 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_text_9 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn layout_item_info (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation , _child_index : sp :: Option < usize > ,) -> sp :: LayoutItemInfo {
             sp :: LayoutItemInfo {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_focusborder_13 {
         r#focusborder_13 : InnerFocusBorder_root_1 , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_focusborder_13 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_3 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_focusborder_13 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerFocusBorder_root_1 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index , tree_index_of_first_child + 1u32 - 1) ;
             (InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_width ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius ()) . apply_pin (_self) . set_constant () ;
             (InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 () + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerFocusBorder_root_1 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info ((InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#focusborder_13 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#focusborder_13 . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info ((InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#focusborder_13 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#focusborder_13 . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) . clone () as sp :: Coord , ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_width ()) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 1u32 ..= 1u32 => return InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (_self) . item_geometry (index - 1u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (_self) . accessible_role (0) , 1u32 ..= 1u32 => InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (_self) . accessible_role (index - 1u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , _) => InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (_self) . accessible_string_property (0 , what) , (1u32 ..= 1u32 , _) => InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (_self) . accessible_string_property (index - 1u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , _) => InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (_self) . accessibility_action (0 , action) , (1u32 ..= 1u32 , _) => InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (_self) . accessibility_action (index - 1u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (_self) . supported_accessibility_actions (0) , 1u32 ..= 1u32 => InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (_self) . supported_accessibility_actions (index - 1u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 ..= 1u32 => InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () . apply_pin (_self) . item_element_infos (index - 1u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_focusborder_13 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_3 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_3 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             2usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_focusborder_13 , sp :: ItemVTable , sp :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#root_1 ()) , sp :: VOffset :: new (InnerComponent_focusborder_13 :: FIELD_OFFSETS . r#focusborder_13 () + InnerFocusBorder_root_1 :: FIELD_OFFSETS . r#rectangle_2 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_focusborder_13) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_focusborder_13 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_focusborder_13 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_focusborder_13 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_focusborder_13 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 4u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_focusborder_13 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_focusborder_13 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerAppWindow {
         r#root_15 : sp :: r#WindowItem , r#text_17 : sp :: r#SimpleText , r#button_18 : InnerButton_root_3 , r#root_15_counter : sp :: Property < i32 > , r#root_15_empty_16_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_15_empty_16_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_15_empty_16_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_15_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_15_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_15_request_increase_value : sp :: Callback < () , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerAppWindow >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerAppWindow {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerButton_root_3 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerAppWindow :: FIELD_OFFSETS . r#button_18 () . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 2u32 - 1 , tree_index_of_first_child + 3u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_15 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#background ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#background () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#root_15_counter ()) . apply_pin (_self) . set ({
                 (((42f64) as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layout_cache ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : (sp :: r#LayoutAlignment :: r#Stretch) . clone () as _ , r#cells : (sp :: Slice :: from_slice (& [({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = (sp :: Item :: layout_info ((InnerAppWindow :: FIELD_OFFSETS . r#text_17 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 1u32 - 1))) . clone () as _ ;
                             the_struct }
                        ) . clone () , ({
                             let mut the_struct = sp :: LayoutItemInfo :: default () ;
                             the_struct . r#constraint = ({
                                 let r#layout_info = (InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v ()) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                     the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                     the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                     the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                     the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                     the_struct . r#stretch = (0f64) . clone () as _ ;
                                     the_struct }
                                 }
                            ) . clone () as _ ;
                             the_struct }
                        ) . clone ()])) . clone () as _ , r#padding : ({
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = (8f64) . clone () as _ ;
                             the_struct . r#end = (8f64) . clone () as _ ;
                             the_struct }
                        ) . clone () as _ , r#size : ((InnerAppWindow :: FIELD_OFFSETS . r#root_15 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . get () . get ()) . clone () as _ , r#spacing : (8f64) . clone () as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = (sp :: Item :: layout_info ((InnerAppWindow :: FIELD_OFFSETS . r#text_17 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 1u32 - 1))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_h ()) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_h ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = (0f64) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (8f64) . clone () as _ ;
                         the_struct . r#end = (8f64) . clone () as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = (sp :: Item :: layout_info ((InnerAppWindow :: FIELD_OFFSETS . r#text_17 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 1u32 - 1))) . clone () as _ ;
                         the_struct }
                    ) . clone () , ({
                         let mut the_struct = sp :: LayoutItemInfo :: default () ;
                         the_struct . r#constraint = ({
                             let r#layout_info = (InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_layoutinfo_v ()) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((r#layout_info) . r#max) . clone () as _ ;
                                 the_struct . r#max_percent = ((r#layout_info) . r#max_percent) . clone () as _ ;
                                 the_struct . r#min = ((32f64 as sp :: Coord) . max (((InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_i_layout_6_layoutinfo_v ()) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) . clone () as _ ;
                                 the_struct . r#min_percent = ((r#layout_info) . r#min_percent) . clone () as _ ;
                                 the_struct . r#preferred = ((r#layout_info) . r#preferred) . clone () as _ ;
                                 the_struct . r#stretch = (0f64) . clone () as _ ;
                                 the_struct }
                             }
                        ) . clone () as _ ;
                         the_struct }
                    ) . clone ()]) as _ , 8f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = (8f64) . clone () as _ ;
                         the_struct . r#end = (8f64) . clone () as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_15_layoutinfo_h ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((((sp :: Item :: layout_info ((InnerAppWindow :: FIELD_OFFSETS . r#root_15 ()) . apply_pin (_self) , sp :: Orientation :: Horizontal , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layoutinfo_h ()) . apply_pin (_self) . get ()) . clone ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#root_15_layoutinfo_v ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((((sp :: Item :: layout_info ((InnerAppWindow :: FIELD_OFFSETS . r#root_15 ()) . apply_pin (_self) , sp :: Orientation :: Vertical , - 1f64 as _ , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ()))) . clone ())) + ((((InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layoutinfo_v ()) . apply_pin (_self) . get ()) . clone ())))) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#root_15 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#title ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Slint Window")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#text_17 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#color ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         * & InnerFluentPalette_20 :: FIELD_OFFSETS . r#foreground () }
                     . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_20 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#text_17 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layout_cache ()) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#text_17 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((((((((sp :: SharedString :: from ("Counter: ")) . clone ())) + (((sp :: shared_string_from_number (((InnerAppWindow :: FIELD_OFFSETS . r#root_15_counter ()) . apply_pin (_self) . get ()) as f64)) . clone ()) . as_str ()))) . clone ())) + (((sp :: SharedString :: from ("")) . clone ()) . as_str ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#text_17 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_15 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler ((InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_clicked ()) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerAppWindow :: FIELD_OFFSETS . r#root_15_request_increase_value ()) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_height ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layout_cache ()) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Increase value")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_width ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_15 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_x ()) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding ((InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_y ()) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layout_cache ()) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             (InnerAppWindow :: FIELD_OFFSETS . r#root_15 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#always_on_top ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_15 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_family ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_15 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_size ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_15 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_15 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_15 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#no_frame ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_15 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#resize_border_width ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_15 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#title ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#text_17 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#text_17 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#text_17 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#text_17 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_icon ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (_self) . set_constant () ;
             (InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_x ()) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerButton_root_3 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | InnerAppWindow :: FIELD_OFFSETS . r#button_18 () . apply_pin (x)) ,) ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#button_18 () . apply_pin (_self) . visit_dynamic_children (dyn_index - 0u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => (InnerAppWindow :: FIELD_OFFSETS . r#root_15_layoutinfo_h ()) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => (InnerAppWindow :: FIELD_OFFSETS . r#root_15_layoutinfo_v ()) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#button_18 () . apply_pin (_self) . subtree_range (dyn_index - 0u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     InnerAppWindow :: FIELD_OFFSETS . r#button_18 () . apply_pin (_self) . subtree_component (dyn_index - 0u32 , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((((InnerAppWindow :: FIELD_OFFSETS . r#root_15 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#height ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_15 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord , ((0f64) . clone ()) . clone () as sp :: Coord ,) , 1u32 => ((((InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layout_cache ()) . apply_pin (_self) . get () [1usize]) . clone ()) . clone () as sp :: Coord , (((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_15 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((8f64) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layout_cache ()) . apply_pin (_self) . get () [0usize]) . clone ()) . clone () as sp :: Coord ,) , 2u32 => ((((InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layout_cache ()) . apply_pin (_self) . get () [3usize]) . clone ()) . clone () as sp :: Coord , (((((((((((InnerAppWindow :: FIELD_OFFSETS . r#root_15 () + sp :: r#WindowItem :: FIELD_OFFSETS . r#width ()) . apply_pin (_self) . get () . get ()) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) as f64) - (((8f64) . clone ()) as f64))) . clone ()) . clone () as sp :: Coord , ((8f64) . clone ()) . clone () as sp :: Coord , (((InnerAppWindow :: FIELD_OFFSETS . r#root_15_empty_16_layout_cache ()) . apply_pin (_self) . get () [2usize]) . clone ()) . clone () as sp :: Coord ,) , 3u32 ..= 9u32 => return InnerAppWindow :: FIELD_OFFSETS . r#button_18 () . apply_pin (_self) . item_geometry (index - 3u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => sp :: r#AccessibleRole :: r#Text , 2u32 => sp :: r#AccessibleRole :: r#Button , 2u32 => InnerAppWindow :: FIELD_OFFSETS . r#button_18 () . apply_pin (_self) . accessible_role (0) , 3u32 ..= 9u32 => InnerAppWindow :: FIELD_OFFSETS . r#button_18 () . apply_pin (_self) . accessible_role (index - 3u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (1u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ((InnerAppWindow :: FIELD_OFFSETS . r#text_17 () + sp :: r#SimpleText :: FIELD_OFFSETS . r#text ()) . apply_pin (_self) . get ()) , (2u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (2u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if (InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_checked ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (2u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if (InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 () + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled ()) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (2u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some ((InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_text ()) . apply_pin (_self) . get ()) , (2u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#button_18 () . apply_pin (_self) . accessible_string_property (0 , what) , (3u32 ..= 9u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#button_18 () . apply_pin (_self) . accessible_string_property (index - 3u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (2u32 , sp :: AccessibilityAction :: r#Default) => {
                     (InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#root_3_accessible_action_default ()) . apply_pin (_self) . call (& ()) }
                 (2u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#button_18 () . apply_pin (_self) . accessibility_action (0 , action) , (3u32 ..= 9u32 , _) => InnerAppWindow :: FIELD_OFFSETS . r#button_18 () . apply_pin (_self) . accessibility_action (index - 3u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 2u32 => sp :: SupportedAccessibilityAction :: r#Default , 2u32 => InnerAppWindow :: FIELD_OFFSETS . r#button_18 () . apply_pin (_self) . supported_accessibility_actions (0) , 3u32 ..= 9u32 => InnerAppWindow :: FIELD_OFFSETS . r#button_18 () . apply_pin (_self) . supported_accessibility_actions (index - 3u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 3u32 ..= 9u32 => InnerAppWindow :: FIELD_OFFSETS . r#button_18 () . apply_pin (_self) . item_element_infos (index - 3u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerAppWindow {
         fn new () -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = SharedGlobals :: new (sp :: VRc :: downgrade (& self_dyn_rc)) ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             10usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 3u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 3u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 7u32 , parent_index : 2u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 10u32 , parent_index : 2u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 10u32 , parent_index : 2u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 2u32 , parent_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 10u32 , parent_index : 3u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 3u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1u32 , parent_index : 3u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerAppWindow , sp :: ItemVTable , sp :: AllowPin > ;
             7usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#root_15 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#text_17 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#root_3 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#i_background_4 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#i_touch_area_11 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#i_focus_scope_12 ()) , sp :: VOffset :: new (InnerAppWindow :: FIELD_OFFSETS . r#button_18 () + InnerButton_root_3 :: FIELD_OFFSETS . r#i_border_5 ())] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerAppWindow) ;
         }
     ;
     impl sp :: PinnedDrop for InnerAppWindow {
         fn drop (self : :: core :: pin :: Pin < & mut InnerAppWindow >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerAppWindow {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerAppWindow > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             false }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     pub struct r#AppWindow (sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow >) ;
     impl r#AppWindow {
         pub fn new () -> :: core :: result :: Result < Self , slint :: PlatformError > {
             slint :: private_unstable_api :: ensure_backend () ? ;
             let inner = InnerAppWindow :: new () ? ;
             inner . globals . get () . unwrap () . window_adapter_ref () ? ;
             InnerAppWindow :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             :: core :: result :: Result :: Ok (Self (inner)) }
         # [cfg (false)] pub fn new_with_context (ctx : sp :: SlintContext) -> :: core :: result :: Result < Self , slint :: PlatformError > {
             let inner = InnerAppWindow :: new () ? ;
             inner . globals . get () . unwrap () . create_window_from_context (ctx) ? ;
             InnerAppWindow :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             :: core :: result :: Result :: Ok (Self (inner)) }
         # [allow (dead_code)] pub fn get_counter (& self) -> i32 {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_15_counter ()) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_counter (& self , value : i32) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_15_counter ()) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_request_increase_value (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             (InnerAppWindow :: FIELD_OFFSETS . r#root_15_request_increase_value ()) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_request_increase_value (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] (InnerAppWindow :: FIELD_OFFSETS . r#root_15_request_increase_value ()) . apply_pin (_self) . set_handler (move | args | f ()) }
         }
     impl From < r#AppWindow > for sp :: VRc < sp :: ItemTreeVTable , InnerAppWindow > {
         fn from (value : r#AppWindow) -> Self {
             value . 0 }
         }
     impl slint :: StrongHandle for r#AppWindow {
         type WeakInner = sp :: VWeak < sp :: ItemTreeVTable , InnerAppWindow > ;
         fn upgrade_from_weak_inner (inner : & Self :: WeakInner) -> sp :: Option < Self > {
             sp :: Some (Self (inner . upgrade () ?)) }
         }
     impl slint :: ComponentHandle for r#AppWindow {
         fn as_weak (& self) -> slint :: Weak < Self > {
             slint :: Weak :: new (sp :: VRc :: downgrade (& self . 0)) }
         fn clone_strong (& self) -> Self {
             Self (self . 0 . clone ()) }
         fn run (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . show () ? ;
             sp :: WindowInner :: from_pub (self . window ()) . context () . run_event_loop () ? ;
             self . hide () ? ;
             :: core :: result :: Result :: Ok (()) }
         fn show (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () ? . window () . show () }
         fn hide (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () ? . window () . hide () }
         fn window (& self) -> & slint :: Window {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () . unwrap () . window () }
         fn global < 'a , T : slint :: Global < 'a , Self >> (& 'a self) -> T {
             T :: get (& self) }
         }
     struct SharedGlobals {
         global_FluentPalette_20 : :: core :: pin :: Pin < sp :: Rc < InnerFluentPalette_20 >> , window_adapter : sp :: OnceCell < sp :: WindowAdapterRc > , root_item_tree_weak : sp :: VWeak < sp :: ItemTreeVTable > , }
     impl SharedGlobals {
         fn new (root_item_tree_weak : sp :: VWeak < sp :: ItemTreeVTable >) -> sp :: Rc < Self > {
             let _self = sp :: Rc :: new (Self {
                 global_FluentPalette_20 : InnerFluentPalette_20 :: new () , window_adapter : :: core :: default :: Default :: default () , root_item_tree_weak , }
            ) ;
             _self . global_FluentPalette_20 . clone () . init (& _self) ;
             _self }
         fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             sp :: Rc :: clone (self . window_adapter_ref () . unwrap ()) }
         fn window_adapter_ref (& self) -> sp :: Result < & sp :: Rc < dyn sp :: WindowAdapter > , slint :: PlatformError > {
             self . window_adapter . get_or_try_init (|| {
                 let adapter = slint :: private_unstable_api :: create_window_adapter () ? ;
                 let root_rc = self . root_item_tree_weak . upgrade () . unwrap () ;
                 sp :: WindowInner :: from_pub (adapter . window ()) . set_component (& root_rc) ;
                 :: core :: result :: Result :: Ok (adapter) }
            ) }
         # [cfg (false)] fn create_window_from_context (& self , ctx : sp :: SlintContext) -> sp :: Result < () , slint :: PlatformError > {
             let adapter = ctx . platform () . create_window_adapter () ? ;
             sp :: WindowInner :: from_pub (adapter . window ()) . set_context (ctx) ;
             let root_rc = self . root_item_tree_weak . upgrade () . unwrap () ;
             sp :: WindowInner :: from_pub (adapter . window ()) . set_component (& root_rc) ;
             self . window_adapter . set (adapter) . map_err (| _ | ()) . expect ("The window shouldn't be initialized before this call") ;
             sp :: Ok (()) }
         fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . window_adapter . get () . cloned () }
         }
     }
 # [allow (unused_imports)] pub use slint_generatedAppWindow :: {
     r#AppWindow , }
 ;
 # [allow (unused_imports)] pub use slint :: {
     ComponentHandle as _ , Global as _ , ModelExt as _ }
 ;
