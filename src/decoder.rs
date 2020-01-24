#[allow(dead_code)]
pub struct Context {
    x: usize,
    y: usize,
    n: usize,
    kk: u8,
    nnn: u16,
    nibbles: (u8, u8, u8, u8),
}

impl Context {
    pub fn new(instruction: u16) -> Self {
        let nibbles: (u8, u8, u8, u8) = (
            ((instruction & 0xF000) >> 12) as u8,
            ((instruction & 0x0F00) >> 8) as u8,
            ((instruction & 0x00F0) >> 4) as u8,
            (instruction & 0x000F) as u8,
        );

        Self {
            nibbles: nibbles,
            x: nibbles.1 as usize,
            y: nibbles.2 as usize,
            n: nibbles.3 as usize,
            kk: (instruction & 0x00FF) as u8,
            nnn: instruction & 0x0FFF,
        }
    }
}

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Opcode {
    CLS,
    RET,
    JP_ADDR,
    CALL_ADDR,
    SE_REG_CONST,
    SNE_REG_CONST,
    SE_REG_REG,
    LD_REG_CONST,
    ADD_REG_CONST,
    LD_REG_REG,
    OR_REG_REG,
    AND_REG_REG,
    XOR_REG_REG,
    ADD_REG_REG,
    SUB_REG_REG,
    SHR_REG,
    SUBN_REG_REG,
    SHL_REG,
    SNE_REG_REG,
    LD_I_ADDR,
    JP_V0_ADDR,
    RND_REG_CONST,
    DRW_REG_REG_CONST,
    SKP_REG,
    SKNP_REG,
    LD_REG_DELAY,
    LD_REG_KEY,
    LD_DELAY_REG,
    LD_SOUND_REG,
    ADD_I_REG,
    LD_FONT_CHAR,
    LD_BCD,
    LD_IND_REG,
    LD_REG_IND,
    UNKNOWN,
}

pub fn decode_instruction(instruction: u16) -> (Opcode, Context) {
    let ctx = Context::new(instruction);

    use Opcode::*;

    let opcode = match ctx.nibbles {
        (0x0, 0x0, 0xE, 0x0) => CLS,
        (0x0, 0x0, 0xE, 0xE) => RET,
        (0x1, _, _, _) => JP_ADDR,
        (0x2, _, _, _) => CALL_ADDR,
        (0x3, _, _, _) => SE_REG_CONST,
        (0x4, _, _, _) => SNE_REG_CONST,
        (0x5, _, _, 0x0) => SE_REG_REG,
        (0x6, _, _, _) => LD_REG_CONST,
        (0x7, _, _, _) => ADD_REG_CONST,
        (0x8, _, _, 0x0) => LD_REG_REG,
        (0x8, _, _, 0x1) => OR_REG_REG,
        (0x8, _, _, 0x2) => AND_REG_REG,
        (0x8, _, _, 0x3) => XOR_REG_REG,
        (0x8, _, _, 0x4) => ADD_REG_REG,
        (0x8, _, _, 0x5) => SUB_REG_REG,
        (0x8, _, _, 0x6) => SHR_REG,
        (0x8, _, _, 0x7) => SUBN_REG_REG,
        (0x8, _, _, 0xE) => SHL_REG,
        (0x9, _, _, 0x00) => SNE_REG_REG,
        (0xA, _, _, _) => LD_I_ADDR,
        (0xB, _, _, _) => JP_V0_ADDR,
        (0xC, _, _, _) => RND_REG_CONST,
        (0xD, _, _, _) => DRW_REG_REG_CONST,
        (0xE, _, 0x9, 0xE) => SKP_REG,
        (0xE, _, 0xA, 0x1) => SKNP_REG,
        (0xF, _, 0x0, 0x7) => LD_REG_DELAY,
        (0xF, _, 0x0, 0xA) => LD_REG_KEY,
        (0xF, _, 0x1, 0x5) => LD_DELAY_REG,
        (0xF, _, 0x1, 0x8) => LD_SOUND_REG,
        (0xF, _, 0x1, 0xE) => ADD_I_REG,
        (0xF, _, 0x2, 0x9) => LD_FONT_CHAR,
        (0xF, _, 0x3, 0x3) => LD_BCD,
        (0xF, _, 0x5, 0x5) => LD_IND_REG,
        (0xF, _, 0x6, 0x5) => LD_REG_IND,
        (_, _, _, _) => UNKNOWN,
    };

    (opcode, ctx)
}

#[cfg(test)]
mod decoder_test {
    use super::*;

    #[test]
    fn test_new_context() {
        let ctx = Context::new(0x1234);

        assert_eq!(ctx.nibbles, (0x1, 0x2, 0x3, 0x4));
        assert_eq!(ctx.x, 0x2);
        assert_eq!(ctx.y, 0x3);
        assert_eq!(ctx.n, 0x4);
        assert_eq!(ctx.kk, 0x34);
        assert_eq!(ctx.nnn, 0x234);
    }

    #[test]
    fn test_clear_screen() {
        let (opcode, _) = decode_instruction(0x00E0);

        assert_eq!(opcode, Opcode::CLS);
    }

    #[test]
    fn test_ret() {
        let (opcode, _) = decode_instruction(0x00EE);

        assert_eq!(opcode, Opcode::RET);
    }

    #[test]
    fn test_jp_addr() {
        let (opcode, _) = decode_instruction(0x1234);

        assert_eq!(opcode, Opcode::JP_ADDR);
    }

    #[test]
    fn test_call_addr() {
        let (opcode, _) = decode_instruction(0x2345);

        assert_eq!(opcode, Opcode::CALL_ADDR);
    }

    #[test]
    fn test_se_reg_const() {
        let (opcode, _) = decode_instruction(0x3456);

        assert_eq!(opcode, Opcode::SE_REG_CONST);
    }

    #[test]
    fn test_sne_reg_const() {
        let (opcode, _) = decode_instruction(0x4567);

        assert_eq!(opcode, Opcode::SNE_REG_CONST);
    }

    #[test]
    fn test_se_reg_reg() {
        let (opcode, _) = decode_instruction(0x5120);

        assert_eq!(opcode, Opcode::SE_REG_REG);
    }

    #[test]
    fn test_ld_reg_const() {
        let (opcode, _) = decode_instruction(0x6123);

        assert_eq!(opcode, Opcode::LD_REG_CONST);
    }

    #[test]
    fn test_add_reg_const() {
        let (opcode, _) = decode_instruction(0x7234);

        assert_eq!(opcode, Opcode::ADD_REG_CONST);
    }

    #[test]
    fn test_ld_reg_reg() {
        let (opcode, _) = decode_instruction(0x8230);

        assert_eq!(opcode, Opcode::LD_REG_REG);
    }

    #[test]
    fn test_or_reg_reg() {
        let (opcode, _) = decode_instruction(0x8341);

        assert_eq!(opcode, Opcode::OR_REG_REG);
    }

    #[test]
    fn test_and_reg_reg() {
        let (opcode, _) = decode_instruction(0x8562);

        assert_eq!(opcode, Opcode::AND_REG_REG);
    }

    #[test]
    fn test_xor_reg_reg() {
        let (opcode, _) = decode_instruction(0x8563);

        assert_eq!(opcode, Opcode::XOR_REG_REG);
    }

    #[test]
    fn test_add_reg_reg() {
        let (opcode, _) = decode_instruction(0x8674);

        assert_eq!(opcode, Opcode::ADD_REG_REG);
    }

    #[test]
    fn test_sub_reg_reg() {
        let (opcode, _) = decode_instruction(0x8785);

        assert_eq!(opcode, Opcode::SUB_REG_REG);
    }

    #[test]
    fn test_shr_reg() {
        let (opcode, _) = decode_instruction(0x8786);

        assert_eq!(opcode, Opcode::SHR_REG);
    }

    #[test]
    fn test_subn_reg_reg() {
        let (opcode, _) = decode_instruction(0x8787);

        assert_eq!(opcode, Opcode::SUBN_REG_REG);
    }

    #[test]
    fn test_shl_reg() {
        let (opcode, _) = decode_instruction(0x878E);

        assert_eq!(opcode, Opcode::SHL_REG);
    }

    #[test]
    fn test_sne_reg_reg() {
        let (opcode, _) = decode_instruction(0x9120);

        assert_eq!(opcode, Opcode::SNE_REG_REG);
    }

    #[test]
    fn test_ld_i_addr() {
        let (opcode, _) = decode_instruction(0xA123);

        assert_eq!(opcode, Opcode::LD_I_ADDR);
    }

    #[test]
    fn test_jp_v0_addr() {
        let (opcode, _) = decode_instruction(0xB123);

        assert_eq!(opcode, Opcode::JP_V0_ADDR);
    }

    #[test]
    fn test_rnd_reg_const() {
        let (opcode, _) = decode_instruction(0xC123);

        assert_eq!(opcode, Opcode::RND_REG_CONST);
    }

    #[test]
    fn test_drw_reg_reg_const() {
        let (opcode, _) = decode_instruction(0xD123);

        assert_eq!(opcode, Opcode::DRW_REG_REG_CONST);
    }

    #[test]
    fn test_skp_reg() {
        let (opcode, _) = decode_instruction(0xE19E);

        assert_eq!(opcode, Opcode::SKP_REG);
    }

    #[test]
    fn test_sknp_reg() {
        let (opcode, _) = decode_instruction(0xE1A1);

        assert_eq!(opcode, Opcode::SKNP_REG);
    }

    #[test]
    fn test_ld_reg_delay() {
        let (opcode, _) = decode_instruction(0xF107);

        assert_eq!(opcode, Opcode::LD_REG_DELAY);
    }

    #[test]
    fn test_ld_reg_key() {
        let (opcode, _) = decode_instruction(0xF10A);

        assert_eq!(opcode, Opcode::LD_REG_KEY);
    }

    #[test]
    fn test_ld_delay_reg() {
        let (opcode, _) = decode_instruction(0xF115);

        assert_eq!(opcode, Opcode::LD_DELAY_REG);
    }

    #[test]
    fn test_ld_sound_reg() {
        let (opcode, _) = decode_instruction(0xF118);

        assert_eq!(opcode, Opcode::LD_SOUND_REG);
    }

    #[test]
    fn test_add_i_reg() {
        let (opcode, _) = decode_instruction(0xF11E);

        assert_eq!(opcode, Opcode::ADD_I_REG);
    }

    #[test]
    fn test_ld_font_char() {
        let (opcode, _) = decode_instruction(0xF129);

        assert_eq!(opcode, Opcode::LD_FONT_CHAR);
    }

    #[test]
    fn test_ld_bcd() {
        let (opcode, _) = decode_instruction(0xF133);

        assert_eq!(opcode, Opcode::LD_BCD);
    }

    #[test]
    fn test_ld_ind_reg() {
        let (opcode, _) = decode_instruction(0xF155);

        assert_eq!(opcode, Opcode::LD_IND_REG);
    }

    #[test]
    fn test_reg_ind() {
        let (opcode, _) = decode_instruction(0xF165);

        assert_eq!(opcode, Opcode::LD_REG_IND);
    }

    #[test]
    fn test_unknown() {
        let (opcode, _) = decode_instruction(0x0000);

        assert_eq!(opcode, Opcode::UNKNOWN);
    }
}
