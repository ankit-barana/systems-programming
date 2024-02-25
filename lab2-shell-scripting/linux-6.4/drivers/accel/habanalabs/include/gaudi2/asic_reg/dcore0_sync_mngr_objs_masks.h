/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2016-2020 HabanaLabs, Ltd.
 * All Rights Reserved.
 *
 */

/************************************
 ** This is an auto-generated file **
 **       DO NOT EDIT BELOW        **
 ************************************/

#ifndef ASIC_REG_DCORE0_SYNC_MNGR_OBJS_MASKS_H_
#define ASIC_REG_DCORE0_SYNC_MNGR_OBJS_MASKS_H_

/*
 *****************************************
 *   DCORE0_SYNC_MNGR_OBJS
 *   (Prototype: SOB_OBJS)
 *****************************************
 */

/* DCORE0_SYNC_MNGR_OBJS_SOB_OBJ */
#define DCORE0_SYNC_MNGR_OBJS_SOB_OBJ_VAL_SHIFT 0
#define DCORE0_SYNC_MNGR_OBJS_SOB_OBJ_VAL_MASK 0x7FFF
#define DCORE0_SYNC_MNGR_OBJS_SOB_OBJ_LONG_SOB_SHIFT 24
#define DCORE0_SYNC_MNGR_OBJS_SOB_OBJ_LONG_SOB_MASK 0x1000000
#define DCORE0_SYNC_MNGR_OBJS_SOB_OBJ_TRACE_EVICT_SHIFT 30
#define DCORE0_SYNC_MNGR_OBJS_SOB_OBJ_TRACE_EVICT_MASK 0x40000000
#define DCORE0_SYNC_MNGR_OBJS_SOB_OBJ_INC_SHIFT 31
#define DCORE0_SYNC_MNGR_OBJS_SOB_OBJ_INC_MASK 0x80000000

/* DCORE0_SYNC_MNGR_OBJS_MON_PAY_ADDRL */
#define DCORE0_SYNC_MNGR_OBJS_MON_PAY_ADDRL_ADDRL_SHIFT 0
#define DCORE0_SYNC_MNGR_OBJS_MON_PAY_ADDRL_ADDRL_MASK 0xFFFFFFFF

/* DCORE0_SYNC_MNGR_OBJS_MON_PAY_ADDRH */
#define DCORE0_SYNC_MNGR_OBJS_MON_PAY_ADDRH_ADDRH_SHIFT 0
#define DCORE0_SYNC_MNGR_OBJS_MON_PAY_ADDRH_ADDRH_MASK 0xFFFFFFFF

/* DCORE0_SYNC_MNGR_OBJS_MON_PAY_DATA */
#define DCORE0_SYNC_MNGR_OBJS_MON_PAY_DATA_DATA_SHIFT 0
#define DCORE0_SYNC_MNGR_OBJS_MON_PAY_DATA_DATA_MASK 0xFFFFFFFF

/* DCORE0_SYNC_MNGR_OBJS_MON_ARM */
#define DCORE0_SYNC_MNGR_OBJS_MON_ARM_SID_SHIFT 0
#define DCORE0_SYNC_MNGR_OBJS_MON_ARM_SID_MASK 0xFF
#define DCORE0_SYNC_MNGR_OBJS_MON_ARM_MASK_SHIFT 8
#define DCORE0_SYNC_MNGR_OBJS_MON_ARM_MASK_MASK 0xFF00
#define DCORE0_SYNC_MNGR_OBJS_MON_ARM_SOP_SHIFT 16
#define DCORE0_SYNC_MNGR_OBJS_MON_ARM_SOP_MASK 0x10000
#define DCORE0_SYNC_MNGR_OBJS_MON_ARM_SOD_SHIFT 17
#define DCORE0_SYNC_MNGR_OBJS_MON_ARM_SOD_MASK 0xFFFE0000

/* DCORE0_SYNC_MNGR_OBJS_MON_CONFIG */
#define DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_LONG_SOB_SHIFT 0
#define DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_LONG_SOB_MASK 0x1
#define DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_CQ_EN_SHIFT 4
#define DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_CQ_EN_MASK 0x10
#define DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_WR_NUM_SHIFT 5
#define DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_WR_NUM_MASK 0x60
#define DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_LBW_EN_SHIFT 8
#define DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_LBW_EN_MASK 0x100
#define DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_MSB_SID_SHIFT 16
#define DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_MSB_SID_MASK 0xF0000
#define DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_LONG_HIGH_GROUP_SHIFT 31
#define DCORE0_SYNC_MNGR_OBJS_MON_CONFIG_LONG_HIGH_GROUP_MASK 0x80000000

/* DCORE0_SYNC_MNGR_OBJS_MON_STATUS */
#define DCORE0_SYNC_MNGR_OBJS_MON_STATUS_VALID_SHIFT 0
#define DCORE0_SYNC_MNGR_OBJS_MON_STATUS_VALID_MASK 0x1
#define DCORE0_SYNC_MNGR_OBJS_MON_STATUS_PENDING_SHIFT 1
#define DCORE0_SYNC_MNGR_OBJS_MON_STATUS_PENDING_MASK 0x1FE
#define DCORE0_SYNC_MNGR_OBJS_MON_STATUS_PROT_SHIFT 9
#define DCORE0_SYNC_MNGR_OBJS_MON_STATUS_PROT_MASK 0x200
#define DCORE0_SYNC_MNGR_OBJS_MON_STATUS_PRIV_SHIFT 10
#define DCORE0_SYNC_MNGR_OBJS_MON_STATUS_PRIV_MASK 0x400

/* DCORE0_SYNC_MNGR_OBJS_SM_SEC */
#define DCORE0_SYNC_MNGR_OBJS_SM_SEC_SEC_VEC_SHIFT 0
#define DCORE0_SYNC_MNGR_OBJS_SM_SEC_SEC_VEC_MASK 0xFFFFFFFF

/* DCORE0_SYNC_MNGR_OBJS_SM_PRIV */
#define DCORE0_SYNC_MNGR_OBJS_SM_PRIV_PRIV_SHIFT 0
#define DCORE0_SYNC_MNGR_OBJS_SM_PRIV_PRIV_MASK 0xFFFFFFFF

#endif /* ASIC_REG_DCORE0_SYNC_MNGR_OBJS_MASKS_H_ */
