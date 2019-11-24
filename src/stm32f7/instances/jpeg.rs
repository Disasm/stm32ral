#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! JPEG codec
//!
//! Used by: stm32f765, stm32f7x7, stm32f7x9

#[cfg(not(feature = "nosync"))]
pub use crate::stm32f7::peripherals::jpeg::Instance;
pub use crate::stm32f7::peripherals::jpeg::{RegisterBlock, ResetValues};
pub use crate::stm32f7::peripherals::jpeg::{
    DHTMEM0, DHTMEM10, DHTMEM100, DHTMEM101, DHTMEM102, DHTMEM103, DHTMEM11, DHTMEM12, DHTMEM13,
    DHTMEM14, DHTMEM15, DHTMEM16, DHTMEM17, DHTMEM18, DHTMEM19, DHTMEM2, DHTMEM20, DHTMEM21,
    DHTMEM22, DHTMEM23, DHTMEM24, DHTMEM25, DHTMEM26, DHTMEM27, DHTMEM28, DHTMEM29, DHTMEM3,
    DHTMEM30, DHTMEM31, DHTMEM32, DHTMEM33, DHTMEM34, DHTMEM35, DHTMEM36, DHTMEM37, DHTMEM38,
    DHTMEM39, DHTMEM4, DHTMEM40, DHTMEM41, DHTMEM42, DHTMEM43, DHTMEM44, DHTMEM45, DHTMEM46,
    DHTMEM47, DHTMEM48, DHTMEM49, DHTMEM5, DHTMEM50, DHTMEM51, DHTMEM52, DHTMEM53, DHTMEM54,
    DHTMEM55, DHTMEM56, DHTMEM57, DHTMEM58, DHTMEM59, DHTMEM6, DHTMEM60, DHTMEM61, DHTMEM62,
    DHTMEM63, DHTMEM64, DHTMEM65, DHTMEM66, DHTMEM67, DHTMEM68, DHTMEM69, DHTMEM7, DHTMEM70,
    DHTMEM71, DHTMEM72, DHTMEM73, DHTMEM74, DHTMEM75, DHTMEM76, DHTMEM77, DHTMEM78, DHTMEM79,
    DHTMEM8, DHTMEM80, DHTMEM81, DHTMEM82, DHTMEM83, DHTMEM84, DHTMEM85, DHTMEM86, DHTMEM87,
    DHTMEM88, DHTMEM89, DHTMEM9, DHTMEM90, DHTMEM91, DHTMEM92, DHTMEM93, DHTMEM94, DHTMEM95,
    DHTMEM96, DHTMEM97, DHTMEM98, DHTMEM99, HUFFBASE0, HUFFBASE1, HUFFBASE10, HUFFBASE11,
    HUFFBASE12, HUFFBASE13, HUFFBASE14, HUFFBASE15, HUFFBASE16, HUFFBASE17, HUFFBASE18, HUFFBASE19,
    HUFFBASE2, HUFFBASE20, HUFFBASE21, HUFFBASE22, HUFFBASE23, HUFFBASE24, HUFFBASE25, HUFFBASE26,
    HUFFBASE27, HUFFBASE28, HUFFBASE29, HUFFBASE3, HUFFBASE30, HUFFBASE31, HUFFBASE4, HUFFBASE5,
    HUFFBASE6, HUFFBASE7, HUFFBASE8, HUFFBASE9, HUFFENC_AC0_0, HUFFENC_AC0_1, HUFFENC_AC0_10,
    HUFFENC_AC0_11, HUFFENC_AC0_12, HUFFENC_AC0_13, HUFFENC_AC0_14, HUFFENC_AC0_15, HUFFENC_AC0_16,
    HUFFENC_AC0_17, HUFFENC_AC0_18, HUFFENC_AC0_19, HUFFENC_AC0_2, HUFFENC_AC0_20, HUFFENC_AC0_21,
    HUFFENC_AC0_22, HUFFENC_AC0_23, HUFFENC_AC0_24, HUFFENC_AC0_25, HUFFENC_AC0_26, HUFFENC_AC0_27,
    HUFFENC_AC0_28, HUFFENC_AC0_29, HUFFENC_AC0_3, HUFFENC_AC0_30, HUFFENC_AC0_31, HUFFENC_AC0_32,
    HUFFENC_AC0_33, HUFFENC_AC0_34, HUFFENC_AC0_35, HUFFENC_AC0_36, HUFFENC_AC0_37, HUFFENC_AC0_38,
    HUFFENC_AC0_39, HUFFENC_AC0_4, HUFFENC_AC0_40, HUFFENC_AC0_41, HUFFENC_AC0_42, HUFFENC_AC0_43,
    HUFFENC_AC0_44, HUFFENC_AC0_45, HUFFENC_AC0_46, HUFFENC_AC0_47, HUFFENC_AC0_48, HUFFENC_AC0_49,
    HUFFENC_AC0_5, HUFFENC_AC0_50, HUFFENC_AC0_51, HUFFENC_AC0_52, HUFFENC_AC0_53, HUFFENC_AC0_54,
    HUFFENC_AC0_55, HUFFENC_AC0_56, HUFFENC_AC0_57, HUFFENC_AC0_58, HUFFENC_AC0_59, HUFFENC_AC0_6,
    HUFFENC_AC0_60, HUFFENC_AC0_61, HUFFENC_AC0_62, HUFFENC_AC0_63, HUFFENC_AC0_64, HUFFENC_AC0_65,
    HUFFENC_AC0_66, HUFFENC_AC0_67, HUFFENC_AC0_68, HUFFENC_AC0_69, HUFFENC_AC0_7, HUFFENC_AC0_70,
    HUFFENC_AC0_71, HUFFENC_AC0_72, HUFFENC_AC0_73, HUFFENC_AC0_74, HUFFENC_AC0_75, HUFFENC_AC0_76,
    HUFFENC_AC0_77, HUFFENC_AC0_78, HUFFENC_AC0_79, HUFFENC_AC0_8, HUFFENC_AC0_80, HUFFENC_AC0_81,
    HUFFENC_AC0_82, HUFFENC_AC0_83, HUFFENC_AC0_84, HUFFENC_AC0_85, HUFFENC_AC0_86, HUFFENC_AC0_87,
    HUFFENC_AC0_9, HUFFENC_AC1_0, HUFFENC_AC1_1, HUFFENC_AC1_10, HUFFENC_AC1_11, HUFFENC_AC1_12,
    HUFFENC_AC1_13, HUFFENC_AC1_14, HUFFENC_AC1_15, HUFFENC_AC1_16, HUFFENC_AC1_17, HUFFENC_AC1_18,
    HUFFENC_AC1_19, HUFFENC_AC1_2, HUFFENC_AC1_20, HUFFENC_AC1_21, HUFFENC_AC1_22, HUFFENC_AC1_23,
    HUFFENC_AC1_24, HUFFENC_AC1_25, HUFFENC_AC1_26, HUFFENC_AC1_27, HUFFENC_AC1_28, HUFFENC_AC1_29,
    HUFFENC_AC1_3, HUFFENC_AC1_30, HUFFENC_AC1_31, HUFFENC_AC1_32, HUFFENC_AC1_33, HUFFENC_AC1_34,
    HUFFENC_AC1_35, HUFFENC_AC1_36, HUFFENC_AC1_37, HUFFENC_AC1_38, HUFFENC_AC1_39, HUFFENC_AC1_4,
    HUFFENC_AC1_40, HUFFENC_AC1_41, HUFFENC_AC1_42, HUFFENC_AC1_43, HUFFENC_AC1_44, HUFFENC_AC1_45,
    HUFFENC_AC1_46, HUFFENC_AC1_47, HUFFENC_AC1_48, HUFFENC_AC1_49, HUFFENC_AC1_5, HUFFENC_AC1_50,
    HUFFENC_AC1_51, HUFFENC_AC1_52, HUFFENC_AC1_53, HUFFENC_AC1_54, HUFFENC_AC1_55, HUFFENC_AC1_56,
    HUFFENC_AC1_57, HUFFENC_AC1_58, HUFFENC_AC1_59, HUFFENC_AC1_6, HUFFENC_AC1_60, HUFFENC_AC1_61,
    HUFFENC_AC1_62, HUFFENC_AC1_63, HUFFENC_AC1_64, HUFFENC_AC1_65, HUFFENC_AC1_66, HUFFENC_AC1_67,
    HUFFENC_AC1_68, HUFFENC_AC1_69, HUFFENC_AC1_7, HUFFENC_AC1_70, HUFFENC_AC1_71, HUFFENC_AC1_72,
    HUFFENC_AC1_73, HUFFENC_AC1_74, HUFFENC_AC1_75, HUFFENC_AC1_76, HUFFENC_AC1_77, HUFFENC_AC1_78,
    HUFFENC_AC1_79, HUFFENC_AC1_8, HUFFENC_AC1_80, HUFFENC_AC1_81, HUFFENC_AC1_82, HUFFENC_AC1_83,
    HUFFENC_AC1_84, HUFFENC_AC1_85, HUFFENC_AC1_86, HUFFENC_AC1_87, HUFFENC_AC1_9, HUFFENC_DC0_0,
    HUFFENC_DC0_1, HUFFENC_DC0_2, HUFFENC_DC0_3, HUFFENC_DC0_4, HUFFENC_DC0_5, HUFFENC_DC0_6,
    HUFFENC_DC0_7, HUFFENC_DC1_0, HUFFENC_DC1_1, HUFFENC_DC1_2, HUFFENC_DC1_3, HUFFENC_DC1_4,
    HUFFENC_DC1_5, HUFFENC_DC1_6, HUFFENC_DC1_7, HUFFMIN_0, HUFFMIN_1, HUFFMIN_10, HUFFMIN_11,
    HUFFMIN_12, HUFFMIN_13, HUFFMIN_14, HUFFMIN_15, HUFFMIN_2, HUFFMIN_3, HUFFMIN_4, HUFFMIN_5,
    HUFFMIN_6, HUFFMIN_7, HUFFMIN_8, HUFFMIN_9, HUFFSYMB0, HUFFSYMB1, HUFFSYMB10, HUFFSYMB11,
    HUFFSYMB12, HUFFSYMB13, HUFFSYMB14, HUFFSYMB15, HUFFSYMB16, HUFFSYMB17, HUFFSYMB18, HUFFSYMB19,
    HUFFSYMB2, HUFFSYMB20, HUFFSYMB21, HUFFSYMB22, HUFFSYMB23, HUFFSYMB24, HUFFSYMB25, HUFFSYMB26,
    HUFFSYMB27, HUFFSYMB28, HUFFSYMB29, HUFFSYMB3, HUFFSYMB30, HUFFSYMB31, HUFFSYMB32, HUFFSYMB33,
    HUFFSYMB34, HUFFSYMB35, HUFFSYMB36, HUFFSYMB37, HUFFSYMB38, HUFFSYMB39, HUFFSYMB4, HUFFSYMB40,
    HUFFSYMB41, HUFFSYMB42, HUFFSYMB43, HUFFSYMB44, HUFFSYMB45, HUFFSYMB46, HUFFSYMB47, HUFFSYMB48,
    HUFFSYMB49, HUFFSYMB5, HUFFSYMB50, HUFFSYMB51, HUFFSYMB52, HUFFSYMB53, HUFFSYMB54, HUFFSYMB55,
    HUFFSYMB56, HUFFSYMB57, HUFFSYMB58, HUFFSYMB59, HUFFSYMB6, HUFFSYMB60, HUFFSYMB61, HUFFSYMB62,
    HUFFSYMB63, HUFFSYMB64, HUFFSYMB65, HUFFSYMB66, HUFFSYMB67, HUFFSYMB68, HUFFSYMB69, HUFFSYMB7,
    HUFFSYMB70, HUFFSYMB71, HUFFSYMB72, HUFFSYMB73, HUFFSYMB74, HUFFSYMB75, HUFFSYMB76, HUFFSYMB77,
    HUFFSYMB78, HUFFSYMB79, HUFFSYMB8, HUFFSYMB80, HUFFSYMB81, HUFFSYMB82, HUFFSYMB83, HUFFSYMB9,
    JPEG_CFR, JPEG_CONFR0, JPEG_CONFR1, JPEG_CONFR2, JPEG_CONFR3, JPEG_CONFR4, JPEG_CONFR5,
    JPEG_CONFR6, JPEG_CONFR7, JPEG_CR, JPEG_DIR, JPEG_DOR, JPEG_SR, QMEM0_0, QMEM0_1, QMEM0_10,
    QMEM0_11, QMEM0_12, QMEM0_13, QMEM0_14, QMEM0_15, QMEM0_2, QMEM0_3, QMEM0_4, QMEM0_5, QMEM0_6,
    QMEM0_7, QMEM0_8, QMEM0_9, QMEM1_0, QMEM1_1, QMEM1_10, QMEM1_11, QMEM1_12, QMEM1_13, QMEM1_14,
    QMEM1_15, QMEM1_2, QMEM1_3, QMEM1_4, QMEM1_5, QMEM1_6, QMEM1_7, QMEM1_8, QMEM1_9, QMEM2_0,
    QMEM2_1, QMEM2_10, QMEM2_11, QMEM2_12, QMEM2_13, QMEM2_14, QMEM2_15, QMEM2_2, QMEM2_3, QMEM2_4,
    QMEM2_5, QMEM2_6, QMEM2_7, QMEM2_8, QMEM2_9, QMEM3_0, QMEM3_1, QMEM3_10, QMEM3_11, QMEM3_12,
    QMEM3_13, QMEM3_14, QMEM3_15, QMEM3_2, QMEM3_3, QMEM3_4, QMEM3_5, QMEM3_6, QMEM3_7, QMEM3_8,
    QMEM3_9,
};

/// Access functions for the JPEG peripheral instance
pub mod JPEG {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x50051000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in JPEG
    pub const reset: ResetValues = ResetValues {
        JPEG_CONFR0: 0x00000000,
        JPEG_CONFR1: 0x00000000,
        JPEG_CONFR2: 0x00000000,
        JPEG_CONFR3: 0x00000000,
        JPEG_CONFR4: 0x00000000,
        JPEG_CONFR5: 0x00000000,
        JPEG_CONFR6: 0x00000000,
        JPEG_CONFR7: 0x00000000,
        JPEG_CR: 0x00000000,
        JPEG_SR: 0x00000000,
        JPEG_CFR: 0x00000000,
        JPEG_DIR: 0x00000000,
        JPEG_DOR: 0x00000000,
        QMEM0_0: 0x00000000,
        QMEM0_1: 0x00000000,
        QMEM0_2: 0x00000000,
        QMEM0_3: 0x00000000,
        QMEM0_4: 0x00000000,
        QMEM0_5: 0x00000000,
        QMEM0_6: 0x00000000,
        QMEM0_7: 0x00000000,
        QMEM0_8: 0x00000000,
        QMEM0_9: 0x00000000,
        QMEM0_10: 0x00000000,
        QMEM0_11: 0x00000000,
        QMEM0_12: 0x00000000,
        QMEM0_13: 0x00000000,
        QMEM0_14: 0x00000000,
        QMEM0_15: 0x00000000,
        QMEM1_0: 0x00000000,
        QMEM1_1: 0x00000000,
        QMEM1_2: 0x00000000,
        QMEM1_3: 0x00000000,
        QMEM1_4: 0x00000000,
        QMEM1_5: 0x00000000,
        QMEM1_6: 0x00000000,
        QMEM1_7: 0x00000000,
        QMEM1_8: 0x00000000,
        QMEM1_9: 0x00000000,
        QMEM1_10: 0x00000000,
        QMEM1_11: 0x00000000,
        QMEM1_12: 0x00000000,
        QMEM1_13: 0x00000000,
        QMEM1_14: 0x00000000,
        QMEM1_15: 0x00000000,
        QMEM2_0: 0x00000000,
        QMEM2_1: 0x00000000,
        QMEM2_2: 0x00000000,
        QMEM2_3: 0x00000000,
        QMEM2_4: 0x00000000,
        QMEM2_5: 0x00000000,
        QMEM2_6: 0x00000000,
        QMEM2_7: 0x00000000,
        QMEM2_8: 0x00000000,
        QMEM2_9: 0x00000000,
        QMEM2_10: 0x00000000,
        QMEM2_11: 0x00000000,
        QMEM2_12: 0x00000000,
        QMEM2_13: 0x00000000,
        QMEM2_14: 0x00000000,
        QMEM2_15: 0x00000000,
        QMEM3_0: 0x00000000,
        QMEM3_1: 0x00000000,
        QMEM3_2: 0x00000000,
        QMEM3_3: 0x00000000,
        QMEM3_4: 0x00000000,
        QMEM3_5: 0x00000000,
        QMEM3_6: 0x00000000,
        QMEM3_7: 0x00000000,
        QMEM3_8: 0x00000000,
        QMEM3_9: 0x00000000,
        QMEM3_10: 0x00000000,
        QMEM3_11: 0x00000000,
        QMEM3_12: 0x00000000,
        QMEM3_13: 0x00000000,
        QMEM3_14: 0x00000000,
        QMEM3_15: 0x00000000,
        HUFFMIN_0: 0x00000000,
        HUFFMIN_1: 0x00000000,
        HUFFMIN_2: 0x00000000,
        HUFFMIN_3: 0x00000000,
        HUFFMIN_4: 0x00000000,
        HUFFMIN_5: 0x00000000,
        HUFFMIN_6: 0x00000000,
        HUFFMIN_7: 0x00000000,
        HUFFMIN_8: 0x00000000,
        HUFFMIN_9: 0x00000000,
        HUFFMIN_10: 0x00000000,
        HUFFMIN_11: 0x00000000,
        HUFFMIN_12: 0x00000000,
        HUFFMIN_13: 0x00000000,
        HUFFMIN_14: 0x00000000,
        HUFFMIN_15: 0x00000000,
        HUFFBASE0: 0x00000000,
        HUFFBASE1: 0x00000000,
        HUFFBASE2: 0x00000000,
        HUFFBASE3: 0x00000000,
        HUFFBASE4: 0x00000000,
        HUFFBASE5: 0x00000000,
        HUFFBASE6: 0x00000000,
        HUFFBASE7: 0x00000000,
        HUFFBASE8: 0x00000000,
        HUFFBASE9: 0x00000000,
        HUFFBASE10: 0x00000000,
        HUFFBASE11: 0x00000000,
        HUFFBASE12: 0x00000000,
        HUFFBASE13: 0x00000000,
        HUFFBASE14: 0x00000000,
        HUFFBASE15: 0x00000000,
        HUFFBASE16: 0x00000000,
        HUFFBASE17: 0x00000000,
        HUFFBASE18: 0x00000000,
        HUFFBASE19: 0x00000000,
        HUFFBASE20: 0x00000000,
        HUFFBASE21: 0x00000000,
        HUFFBASE22: 0x00000000,
        HUFFBASE23: 0x00000000,
        HUFFBASE24: 0x00000000,
        HUFFBASE25: 0x00000000,
        HUFFBASE26: 0x00000000,
        HUFFBASE27: 0x00000000,
        HUFFBASE28: 0x00000000,
        HUFFBASE29: 0x00000000,
        HUFFBASE30: 0x00000000,
        HUFFBASE31: 0x00000000,
        HUFFSYMB0: 0x00000000,
        HUFFSYMB1: 0x00000000,
        HUFFSYMB2: 0x00000000,
        HUFFSYMB3: 0x00000000,
        HUFFSYMB4: 0x00000000,
        HUFFSYMB5: 0x00000000,
        HUFFSYMB6: 0x00000000,
        HUFFSYMB7: 0x00000000,
        HUFFSYMB8: 0x00000000,
        HUFFSYMB9: 0x00000000,
        HUFFSYMB10: 0x00000000,
        HUFFSYMB11: 0x00000000,
        HUFFSYMB12: 0x00000000,
        HUFFSYMB13: 0x00000000,
        HUFFSYMB14: 0x00000000,
        HUFFSYMB15: 0x00000000,
        HUFFSYMB16: 0x00000000,
        HUFFSYMB17: 0x00000000,
        HUFFSYMB18: 0x00000000,
        HUFFSYMB19: 0x00000000,
        HUFFSYMB20: 0x00000000,
        HUFFSYMB21: 0x00000000,
        HUFFSYMB22: 0x00000000,
        HUFFSYMB23: 0x00000000,
        HUFFSYMB24: 0x00000000,
        HUFFSYMB25: 0x00000000,
        HUFFSYMB26: 0x00000000,
        HUFFSYMB27: 0x00000000,
        HUFFSYMB28: 0x00000000,
        HUFFSYMB29: 0x00000000,
        HUFFSYMB30: 0x00000000,
        HUFFSYMB31: 0x00000000,
        HUFFSYMB32: 0x00000000,
        HUFFSYMB33: 0x00000000,
        HUFFSYMB34: 0x00000000,
        HUFFSYMB35: 0x00000000,
        HUFFSYMB36: 0x00000000,
        HUFFSYMB37: 0x00000000,
        HUFFSYMB38: 0x00000000,
        HUFFSYMB39: 0x00000000,
        HUFFSYMB40: 0x00000000,
        HUFFSYMB41: 0x00000000,
        HUFFSYMB42: 0x00000000,
        HUFFSYMB43: 0x00000000,
        HUFFSYMB44: 0x00000000,
        HUFFSYMB45: 0x00000000,
        HUFFSYMB46: 0x00000000,
        HUFFSYMB47: 0x00000000,
        HUFFSYMB48: 0x00000000,
        HUFFSYMB49: 0x00000000,
        HUFFSYMB50: 0x00000000,
        HUFFSYMB51: 0x00000000,
        HUFFSYMB52: 0x00000000,
        HUFFSYMB53: 0x00000000,
        HUFFSYMB54: 0x00000000,
        HUFFSYMB55: 0x00000000,
        HUFFSYMB56: 0x00000000,
        HUFFSYMB57: 0x00000000,
        HUFFSYMB58: 0x00000000,
        HUFFSYMB59: 0x00000000,
        HUFFSYMB60: 0x00000000,
        HUFFSYMB61: 0x00000000,
        HUFFSYMB62: 0x00000000,
        HUFFSYMB63: 0x00000000,
        HUFFSYMB64: 0x00000000,
        HUFFSYMB65: 0x00000000,
        HUFFSYMB66: 0x00000000,
        HUFFSYMB67: 0x00000000,
        HUFFSYMB68: 0x00000000,
        HUFFSYMB69: 0x00000000,
        HUFFSYMB70: 0x00000000,
        HUFFSYMB71: 0x00000000,
        HUFFSYMB72: 0x00000000,
        HUFFSYMB73: 0x00000000,
        HUFFSYMB74: 0x00000000,
        HUFFSYMB75: 0x00000000,
        HUFFSYMB76: 0x00000000,
        HUFFSYMB77: 0x00000000,
        HUFFSYMB78: 0x00000000,
        HUFFSYMB79: 0x00000000,
        HUFFSYMB80: 0x00000000,
        HUFFSYMB81: 0x00000000,
        HUFFSYMB82: 0x00000000,
        HUFFSYMB83: 0x00000000,
        DHTMEM0: 0x00000000,
        DHTMEM2: 0x00000000,
        DHTMEM3: 0x00000000,
        DHTMEM4: 0x00000000,
        DHTMEM5: 0x00000000,
        DHTMEM6: 0x00000000,
        DHTMEM7: 0x00000000,
        DHTMEM8: 0x00000000,
        DHTMEM9: 0x00000000,
        DHTMEM10: 0x00000000,
        DHTMEM11: 0x00000000,
        DHTMEM12: 0x00000000,
        DHTMEM13: 0x00000000,
        DHTMEM14: 0x00000000,
        DHTMEM15: 0x00000000,
        DHTMEM16: 0x00000000,
        DHTMEM17: 0x00000000,
        DHTMEM18: 0x00000000,
        DHTMEM19: 0x00000000,
        DHTMEM20: 0x00000000,
        DHTMEM21: 0x00000000,
        DHTMEM22: 0x00000000,
        DHTMEM23: 0x00000000,
        DHTMEM24: 0x00000000,
        DHTMEM25: 0x00000000,
        DHTMEM26: 0x00000000,
        DHTMEM27: 0x00000000,
        DHTMEM28: 0x00000000,
        DHTMEM29: 0x00000000,
        DHTMEM30: 0x00000000,
        DHTMEM31: 0x00000000,
        DHTMEM32: 0x00000000,
        DHTMEM33: 0x00000000,
        DHTMEM34: 0x00000000,
        DHTMEM35: 0x00000000,
        DHTMEM36: 0x00000000,
        DHTMEM37: 0x00000000,
        DHTMEM38: 0x00000000,
        DHTMEM39: 0x00000000,
        DHTMEM40: 0x00000000,
        DHTMEM41: 0x00000000,
        DHTMEM42: 0x00000000,
        DHTMEM43: 0x00000000,
        DHTMEM44: 0x00000000,
        DHTMEM45: 0x00000000,
        DHTMEM46: 0x00000000,
        DHTMEM47: 0x00000000,
        DHTMEM48: 0x00000000,
        DHTMEM49: 0x00000000,
        DHTMEM50: 0x00000000,
        DHTMEM51: 0x00000000,
        DHTMEM52: 0x00000000,
        DHTMEM53: 0x00000000,
        DHTMEM54: 0x00000000,
        DHTMEM55: 0x00000000,
        DHTMEM56: 0x00000000,
        DHTMEM57: 0x00000000,
        DHTMEM58: 0x00000000,
        DHTMEM59: 0x00000000,
        DHTMEM60: 0x00000000,
        DHTMEM61: 0x00000000,
        DHTMEM62: 0x00000000,
        DHTMEM63: 0x00000000,
        DHTMEM64: 0x00000000,
        DHTMEM65: 0x00000000,
        DHTMEM66: 0x00000000,
        DHTMEM67: 0x00000000,
        DHTMEM68: 0x00000000,
        DHTMEM69: 0x00000000,
        DHTMEM70: 0x00000000,
        DHTMEM71: 0x00000000,
        DHTMEM72: 0x00000000,
        DHTMEM73: 0x00000000,
        DHTMEM74: 0x00000000,
        DHTMEM75: 0x00000000,
        DHTMEM76: 0x00000000,
        DHTMEM77: 0x00000000,
        DHTMEM78: 0x00000000,
        DHTMEM79: 0x00000000,
        DHTMEM80: 0x00000000,
        DHTMEM81: 0x00000000,
        DHTMEM82: 0x00000000,
        DHTMEM83: 0x00000000,
        DHTMEM84: 0x00000000,
        DHTMEM85: 0x00000000,
        DHTMEM86: 0x00000000,
        DHTMEM87: 0x00000000,
        DHTMEM88: 0x00000000,
        DHTMEM89: 0x00000000,
        DHTMEM90: 0x00000000,
        DHTMEM91: 0x00000000,
        DHTMEM92: 0x00000000,
        DHTMEM93: 0x00000000,
        DHTMEM94: 0x00000000,
        DHTMEM95: 0x00000000,
        DHTMEM96: 0x00000000,
        DHTMEM97: 0x00000000,
        DHTMEM98: 0x00000000,
        DHTMEM99: 0x00000000,
        DHTMEM100: 0x00000000,
        DHTMEM101: 0x00000000,
        DHTMEM102: 0x00000000,
        DHTMEM103: 0x00000000,
        HUFFENC_AC0_0: 0x00000000,
        HUFFENC_AC0_1: 0x00000000,
        HUFFENC_AC0_2: 0x00000000,
        HUFFENC_AC0_3: 0x00000000,
        HUFFENC_AC0_4: 0x00000000,
        HUFFENC_AC0_5: 0x00000000,
        HUFFENC_AC0_6: 0x00000000,
        HUFFENC_AC0_7: 0x00000000,
        HUFFENC_AC0_8: 0x00000000,
        HUFFENC_AC0_9: 0x00000000,
        HUFFENC_AC0_10: 0x00000000,
        HUFFENC_AC0_11: 0x00000000,
        HUFFENC_AC0_12: 0x00000000,
        HUFFENC_AC0_13: 0x00000000,
        HUFFENC_AC0_14: 0x00000000,
        HUFFENC_AC0_15: 0x00000000,
        HUFFENC_AC0_16: 0x00000000,
        HUFFENC_AC0_17: 0x00000000,
        HUFFENC_AC0_18: 0x00000000,
        HUFFENC_AC0_19: 0x00000000,
        HUFFENC_AC0_20: 0x00000000,
        HUFFENC_AC0_21: 0x00000000,
        HUFFENC_AC0_22: 0x00000000,
        HUFFENC_AC0_23: 0x00000000,
        HUFFENC_AC0_24: 0x00000000,
        HUFFENC_AC0_25: 0x00000000,
        HUFFENC_AC0_26: 0x00000000,
        HUFFENC_AC0_27: 0x00000000,
        HUFFENC_AC0_28: 0x00000000,
        HUFFENC_AC0_29: 0x00000000,
        HUFFENC_AC0_30: 0x00000000,
        HUFFENC_AC0_31: 0x00000000,
        HUFFENC_AC0_32: 0x00000000,
        HUFFENC_AC0_33: 0x00000000,
        HUFFENC_AC0_34: 0x00000000,
        HUFFENC_AC0_35: 0x00000000,
        HUFFENC_AC0_36: 0x00000000,
        HUFFENC_AC0_37: 0x00000000,
        HUFFENC_AC0_38: 0x00000000,
        HUFFENC_AC0_39: 0x00000000,
        HUFFENC_AC0_40: 0x00000000,
        HUFFENC_AC0_41: 0x00000000,
        HUFFENC_AC0_42: 0x00000000,
        HUFFENC_AC0_43: 0x00000000,
        HUFFENC_AC0_44: 0x00000000,
        HUFFENC_AC0_45: 0x00000000,
        HUFFENC_AC0_46: 0x00000000,
        HUFFENC_AC0_47: 0x00000000,
        HUFFENC_AC0_48: 0x00000000,
        HUFFENC_AC0_49: 0x00000000,
        HUFFENC_AC0_50: 0x00000000,
        HUFFENC_AC0_51: 0x00000000,
        HUFFENC_AC0_52: 0x00000000,
        HUFFENC_AC0_53: 0x00000000,
        HUFFENC_AC0_54: 0x00000000,
        HUFFENC_AC0_55: 0x00000000,
        HUFFENC_AC0_56: 0x00000000,
        HUFFENC_AC0_57: 0x00000000,
        HUFFENC_AC0_58: 0x00000000,
        HUFFENC_AC0_59: 0x00000000,
        HUFFENC_AC0_60: 0x00000000,
        HUFFENC_AC0_61: 0x00000000,
        HUFFENC_AC0_62: 0x00000000,
        HUFFENC_AC0_63: 0x00000000,
        HUFFENC_AC0_64: 0x00000000,
        HUFFENC_AC0_65: 0x00000000,
        HUFFENC_AC0_66: 0x00000000,
        HUFFENC_AC0_67: 0x00000000,
        HUFFENC_AC0_68: 0x00000000,
        HUFFENC_AC0_69: 0x00000000,
        HUFFENC_AC0_70: 0x00000000,
        HUFFENC_AC0_71: 0x00000000,
        HUFFENC_AC0_72: 0x00000000,
        HUFFENC_AC0_73: 0x00000000,
        HUFFENC_AC0_74: 0x00000000,
        HUFFENC_AC0_75: 0x00000000,
        HUFFENC_AC0_76: 0x00000000,
        HUFFENC_AC0_77: 0x00000000,
        HUFFENC_AC0_78: 0x00000000,
        HUFFENC_AC0_79: 0x00000000,
        HUFFENC_AC0_80: 0x00000000,
        HUFFENC_AC0_81: 0x00000000,
        HUFFENC_AC0_82: 0x00000000,
        HUFFENC_AC0_83: 0x00000000,
        HUFFENC_AC0_84: 0x00000000,
        HUFFENC_AC0_85: 0x00000000,
        HUFFENC_AC0_86: 0x00000000,
        HUFFENC_AC0_87: 0x00000000,
        HUFFENC_AC1_0: 0x00000000,
        HUFFENC_AC1_1: 0x00000000,
        HUFFENC_AC1_2: 0x00000000,
        HUFFENC_AC1_3: 0x00000000,
        HUFFENC_AC1_4: 0x00000000,
        HUFFENC_AC1_5: 0x00000000,
        HUFFENC_AC1_6: 0x00000000,
        HUFFENC_AC1_7: 0x00000000,
        HUFFENC_AC1_8: 0x00000000,
        HUFFENC_AC1_9: 0x00000000,
        HUFFENC_AC1_10: 0x00000000,
        HUFFENC_AC1_11: 0x00000000,
        HUFFENC_AC1_12: 0x00000000,
        HUFFENC_AC1_13: 0x00000000,
        HUFFENC_AC1_14: 0x00000000,
        HUFFENC_AC1_15: 0x00000000,
        HUFFENC_AC1_16: 0x00000000,
        HUFFENC_AC1_17: 0x00000000,
        HUFFENC_AC1_18: 0x00000000,
        HUFFENC_AC1_19: 0x00000000,
        HUFFENC_AC1_20: 0x00000000,
        HUFFENC_AC1_21: 0x00000000,
        HUFFENC_AC1_22: 0x00000000,
        HUFFENC_AC1_23: 0x00000000,
        HUFFENC_AC1_24: 0x00000000,
        HUFFENC_AC1_25: 0x00000000,
        HUFFENC_AC1_26: 0x00000000,
        HUFFENC_AC1_27: 0x00000000,
        HUFFENC_AC1_28: 0x00000000,
        HUFFENC_AC1_29: 0x00000000,
        HUFFENC_AC1_30: 0x00000000,
        HUFFENC_AC1_31: 0x00000000,
        HUFFENC_AC1_32: 0x00000000,
        HUFFENC_AC1_33: 0x00000000,
        HUFFENC_AC1_34: 0x00000000,
        HUFFENC_AC1_35: 0x00000000,
        HUFFENC_AC1_36: 0x00000000,
        HUFFENC_AC1_37: 0x00000000,
        HUFFENC_AC1_38: 0x00000000,
        HUFFENC_AC1_39: 0x00000000,
        HUFFENC_AC1_40: 0x00000000,
        HUFFENC_AC1_41: 0x00000000,
        HUFFENC_AC1_42: 0x00000000,
        HUFFENC_AC1_43: 0x00000000,
        HUFFENC_AC1_44: 0x00000000,
        HUFFENC_AC1_45: 0x00000000,
        HUFFENC_AC1_46: 0x00000000,
        HUFFENC_AC1_47: 0x00000000,
        HUFFENC_AC1_48: 0x00000000,
        HUFFENC_AC1_49: 0x00000000,
        HUFFENC_AC1_50: 0x00000000,
        HUFFENC_AC1_51: 0x00000000,
        HUFFENC_AC1_52: 0x00000000,
        HUFFENC_AC1_53: 0x00000000,
        HUFFENC_AC1_54: 0x00000000,
        HUFFENC_AC1_55: 0x00000000,
        HUFFENC_AC1_56: 0x00000000,
        HUFFENC_AC1_57: 0x00000000,
        HUFFENC_AC1_58: 0x00000000,
        HUFFENC_AC1_59: 0x00000000,
        HUFFENC_AC1_60: 0x00000000,
        HUFFENC_AC1_61: 0x00000000,
        HUFFENC_AC1_62: 0x00000000,
        HUFFENC_AC1_63: 0x00000000,
        HUFFENC_AC1_64: 0x00000000,
        HUFFENC_AC1_65: 0x00000000,
        HUFFENC_AC1_66: 0x00000000,
        HUFFENC_AC1_67: 0x00000000,
        HUFFENC_AC1_68: 0x00000000,
        HUFFENC_AC1_69: 0x00000000,
        HUFFENC_AC1_70: 0x00000000,
        HUFFENC_AC1_71: 0x00000000,
        HUFFENC_AC1_72: 0x00000000,
        HUFFENC_AC1_73: 0x00000000,
        HUFFENC_AC1_74: 0x00000000,
        HUFFENC_AC1_75: 0x00000000,
        HUFFENC_AC1_76: 0x00000000,
        HUFFENC_AC1_77: 0x00000000,
        HUFFENC_AC1_78: 0x00000000,
        HUFFENC_AC1_79: 0x00000000,
        HUFFENC_AC1_80: 0x00000000,
        HUFFENC_AC1_81: 0x00000000,
        HUFFENC_AC1_82: 0x00000000,
        HUFFENC_AC1_83: 0x00000000,
        HUFFENC_AC1_84: 0x00000000,
        HUFFENC_AC1_85: 0x00000000,
        HUFFENC_AC1_86: 0x00000000,
        HUFFENC_AC1_87: 0x00000000,
        HUFFENC_DC0_0: 0x00000000,
        HUFFENC_DC0_1: 0x00000000,
        HUFFENC_DC0_2: 0x00000000,
        HUFFENC_DC0_3: 0x00000000,
        HUFFENC_DC0_4: 0x00000000,
        HUFFENC_DC0_5: 0x00000000,
        HUFFENC_DC0_6: 0x00000000,
        HUFFENC_DC0_7: 0x00000000,
        HUFFENC_DC1_0: 0x00000000,
        HUFFENC_DC1_1: 0x00000000,
        HUFFENC_DC1_2: 0x00000000,
        HUFFENC_DC1_3: 0x00000000,
        HUFFENC_DC1_4: 0x00000000,
        HUFFENC_DC1_5: 0x00000000,
        HUFFENC_DC1_6: 0x00000000,
        HUFFENC_DC1_7: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut JPEG_TAKEN: bool = false;

    /// Safe access to JPEG
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        external_cortex_m::interrupt::free(|_| unsafe {
            if JPEG_TAKEN {
                None
            } else {
                JPEG_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to JPEG
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if JPEG_TAKEN && inst.addr == INSTANCE.addr {
                JPEG_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal JPEG
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        JPEG_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to JPEG
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const JPEG: *const RegisterBlock = 0x50051000 as *const _;
