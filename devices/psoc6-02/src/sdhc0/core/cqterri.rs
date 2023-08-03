#[doc = "Reader of register CQTERRI"]
pub type R = crate::R<u32, super::CQTERRI>;
#[doc = "Reader of field `RESP_ERR_CMD_INDX`"]
pub type RESP_ERR_CMD_INDX_R = crate::R<u8, u8>;
#[doc = "Reader of field `RESP_ERR_TASKID`"]
pub type RESP_ERR_TASKID_R = crate::R<u8, u8>;
#[doc = "Reader of field `RESP_ERR_FIELDS_VALID`"]
pub type RESP_ERR_FIELDS_VALID_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRANS_ERR_CMD_INDX`"]
pub type TRANS_ERR_CMD_INDX_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRANS_ERR_TASKID`"]
pub type TRANS_ERR_TASKID_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRANS_ERR_FIELDS_VALID`"]
pub type TRANS_ERR_FIELDS_VALID_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:5 - This field captures the index of the command that was executed on the command line when the error occurred."]
    #[inline(always)]
    pub fn resp_err_cmd_indx(&self) -> RESP_ERR_CMD_INDX_R {
        RESP_ERR_CMD_INDX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - This field captures the ID of the task which was executed on the command line when the error occurred."]
    #[inline(always)]
    pub fn resp_err_taskid(&self) -> RESP_ERR_TASKID_R {
        RESP_ERR_TASKID_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - This bit is updated when an error is detected while a command transaction was in progress. Values: - 0x1 (SET): Response-related error is detected. Check contents of RESP_ERR_TASKID and RESP_ERR_CMD_INDX fields - 0x0 (NOT_SET): Ignore contents of RESP_ERR_TASKID and RESP_ERR_CMD_INDX"]
    #[inline(always)]
    pub fn resp_err_fields_valid(&self) -> RESP_ERR_FIELDS_VALID_R {
        RESP_ERR_FIELDS_VALID_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - This field captures the index of the command that was executed and whose data transfer has errors."]
    #[inline(always)]
    pub fn trans_err_cmd_indx(&self) -> TRANS_ERR_CMD_INDX_R {
        TRANS_ERR_CMD_INDX_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:28 - This field captures the ID of the task that was executed and whose data transfer has errors."]
    #[inline(always)]
    pub fn trans_err_taskid(&self) -> TRANS_ERR_TASKID_R {
        TRANS_ERR_TASKID_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - This bit is updated when an error is detected while a data transfer transaction was in progress. Values: - 0x1 (SET): data transfer related error detected. Check contents of TRANS_ERR_TASKID and TRANS_ERR_CMD_INDX fields - 0x0 (NOT_SET): Ignore contents of TRANS_ERR_TASKID and TRANS_ERR_CMD_INDX"]
    #[inline(always)]
    pub fn trans_err_fields_valid(&self) -> TRANS_ERR_FIELDS_VALID_R {
        TRANS_ERR_FIELDS_VALID_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
