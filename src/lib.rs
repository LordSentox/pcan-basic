use pcan_basic_sys as pcan;


#[derive(Debug, PartialEq)]
pub enum MessageType {
    Standard,
    Extended
}

#[derive(Debug, PartialEq)]
pub enum FrameConstructionError {
    TooMuchData,
    CanIdMessageTypeMismatch
}

pub const STANDARD_MASK: u32 = 0x07_FF;
pub const EXTENDED_MASK: u32 = 0x1F_FF_FF_FF;

#[derive(Debug, Copy, Clone)]
pub struct CanFrame {
    frame: pcan::TPCANMsg
}

impl CanFrame {
    const MAX_DLC: usize = 8;

    pub fn new(can_id: u32, msg_type: MessageType, data: &[u8]) -> Result<CanFrame, FrameConstructionError> {
        if data.len() > Self::MAX_DLC {
            Err(FrameConstructionError::TooMuchData)
        } else {
            let mut frame_data: [u8; 8] = [0; 8];
            for (i, v) in data.into_iter().enumerate() {
                frame_data[i] = *v;
            }

            match msg_type {
                MessageType::Standard => {
                    Ok(CanFrame {
                        frame: pcan::TPCANMsg {
                            ID: can_id & STANDARD_MASK,
                            MSGTYPE: pcan::PCAN_MESSAGE_STANDARD as u8,
                            LEN: data.len() as u8,
                            DATA: frame_data
                        }
                    })
                },
                MessageType::Extended => {
                    Ok(CanFrame {
                        frame: pcan::TPCANMsg {
                            ID: can_id & STANDARD_MASK,
                            MSGTYPE: pcan::PCAN_MESSAGE_STANDARD as u8,
                            LEN: data.len() as u8,
                            DATA: frame_data
                        }
                    })
                }
            }
        }
    }

    pub fn is_standard_frame(&self) -> bool {
        if self.frame.MSGTYPE & pcan::PCAN_MESSAGE_STANDARD as u8 != 0 {
            true
        } else {
            false
        }
    }

    pub fn is_extended_frame(&self) -> bool {
        if self.frame.MSGTYPE & pcan::PCAN_MESSAGE_EXTENDED as u8 != 0 {
            true
        } else {
            false
        }
    }

    pub fn can_id(&self) -> u32 {
        if self.is_standard_frame() {
            self.frame.ID & STANDARD_MASK
        } else {
            self.frame.ID & EXTENDED_MASK
        }
    }

    pub fn dlc(&self) -> u8 {
        self.frame.LEN
    }

    pub fn data(&self) -> &[u8] {
        &self.frame.DATA[0..self.dlc() as usize]
    }

    pub fn mut_data(&mut self) -> &mut [u8] {
        let dlc = self.dlc();
        &mut self.frame.DATA[0..dlc as usize]
    }
}

impl Default for CanFrame {
    fn default() -> Self {
        CanFrame::new(0, MessageType::Standard, &[]).unwrap()
    }
}

impl PartialEq for CanFrame {
    fn eq(&self, other: &Self) -> bool {
        if self.frame.ID != other.frame.ID {
            return false;
        }

        if self.frame.LEN != other.frame.LEN {
            return false;
        }

        if self.frame.MSGTYPE != other.frame.MSGTYPE {
            return false;
        }

        if self.data() != other.data() {
            return false;
        }

        true
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CanFdFrame {
    frame: pcan::TPCANMsgFD
}

impl CanFdFrame {
    const MAX_DLC: usize = 64;

    pub fn new(can_id: u32, msg_type: MessageType, data: &[u8]) -> Result<CanFdFrame, FrameConstructionError> {
        if data.len() > Self::MAX_DLC {
            Err(FrameConstructionError::TooMuchData)
        } else {
            let mut frame_data: [u8; 64] = [0; 64];
            for (i, v) in data.into_iter().enumerate() {
                frame_data[i] = *v;
            }

            match msg_type {
                MessageType::Standard => {
                    Ok(CanFdFrame {
                        frame: pcan::TPCANMsgFD {
                            ID: can_id & STANDARD_MASK,
                            MSGTYPE: pcan::PCAN_MESSAGE_STANDARD as u8,
                            DLC: data.len() as u8,
                            DATA: frame_data
                        }
                    })
                },
                MessageType::Extended => {
                    Ok(CanFdFrame {
                        frame: pcan::TPCANMsgFD {
                            ID: can_id & STANDARD_MASK,
                            MSGTYPE: pcan::PCAN_MESSAGE_STANDARD as u8,
                            DLC: data.len() as u8,
                            DATA: frame_data
                        }
                    })
                }
            }
        }
    }

    pub fn is_standard_frame(&self) -> bool {
        if self.frame.MSGTYPE & pcan::PCAN_MESSAGE_STANDARD as u8 != 0 {
            true
        } else {
            false
        }
    }

    pub fn is_extended_frame(&self) -> bool {
        if self.frame.MSGTYPE & pcan::PCAN_MESSAGE_EXTENDED as u8 != 0 {
            true
        } else {
            false
        }
    }

    pub fn can_id(&self) -> u32 {
        if self.is_standard_frame() {
            self.frame.ID & STANDARD_MASK
        } else {
            self.frame.ID & EXTENDED_MASK
        }
    }

    pub fn dlc(&self) -> u8 {
        self.frame.DLC
    }

    pub fn data(&self) -> &[u8] {
        &self.frame.DATA[0..self.dlc() as usize]
    }

    pub fn mut_data(&mut self) -> &mut [u8] {
        let dlc = self.dlc();
        &mut self.frame.DATA[0..dlc as usize]
    }
}

impl Default for CanFdFrame {
    fn default() -> Self {
        CanFdFrame::new(0, MessageType::Standard, &[]).unwrap()
    }
}

impl PartialEq for CanFdFrame {
    fn eq(&self, other: &Self) -> bool {
        if self.frame.ID != other.frame.ID {
            return false;
        }

        if self.frame.DLC != other.frame.DLC {
            return false;
        }

        if self.frame.MSGTYPE != other.frame.MSGTYPE {
            return false;
        }

        if self.data() != other.data() {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_frame_new_001() {
        let can_frame_1 = CanFrame::new(
            0x20,
            MessageType::Standard,
            &[0, 1, 2, 3, 4, 5, 6, 7]
        ).unwrap();

        let can_frame_2 = CanFrame::new(
            0x20,
            MessageType::Standard,
            &[0, 1, 2, 3, 4, 5, 6, 7]
        ).unwrap();

        assert_eq!(can_frame_1, can_frame_2);
    }

    #[test]
    fn can_frame_new_002() {
        let can_frame_1 = CanFrame::new(
            0x20,
            MessageType::Extended,
            &[0, 1, 2, 3, 4, 5, 6, 7]
        ).unwrap();

        let can_frame_2 = CanFrame::new(
            0x20,
            MessageType::Extended,
            &[0, 1, 2, 3, 4, 5, 6, 7]
        ).unwrap();

        assert_eq!(can_frame_1, can_frame_2);
    }

    #[test]
    #[should_panic]
    fn can_frame_new_003() {
        let _can_frame_1 = CanFrame::new(
            0x20,
            MessageType::Standard,
            &[0, 1, 2, 3, 4, 5, 6, 7, 8]
        ).unwrap();
    }

    #[test]
    #[should_panic]
    fn can_frame_new_004() {
        let _can_frame_1 = CanFrame::new(
            0x20,
            MessageType::Extended,
            &[0, 1, 2, 3, 4, 5, 6, 7, 8]
        ).unwrap();
    }

    /* CAN FD FRAME */

    #[test]
    fn can_fd_frame_new_001() {
        let can_frame_1 = CanFdFrame::new(
            0x20,
            MessageType::Standard,
            &(0..64u8).collect::<Vec<_>>()
        ).unwrap();

        let can_frame_2 = CanFdFrame::new(
            0x20,
            MessageType::Standard,
            &(0..64u8).collect::<Vec<_>>()
        ).unwrap();

        assert_eq!(can_frame_1, can_frame_2);
    }

    #[test]
    fn can_fd_frame_new_002() {
        let can_frame_1 = CanFdFrame::new(
            0x20,
            MessageType::Extended,
            &(0..64u8).collect::<Vec<_>>()
        ).unwrap();

        let can_frame_2 = CanFdFrame::new(
            0x20,
            MessageType::Extended,
            &(0..64u8).collect::<Vec<_>>()
        ).unwrap();

        assert_eq!(can_frame_1, can_frame_2);
    }

    #[test]
    #[should_panic]
    fn can_fd_frame_new_003() {
        let _can_frame_1 = CanFdFrame::new(
            0x20,
            MessageType::Standard,
            &(0..65u8).collect::<Vec<_>>()
        ).unwrap();
    }

    #[test]
    #[should_panic]
    fn can_fd_frame_new_004() {
        let _can_frame_1 = CanFrame::new(
            0x20,
            MessageType::Extended,
            &&(0..65u8).collect::<Vec<_>>()
        ).unwrap();
    }
}
