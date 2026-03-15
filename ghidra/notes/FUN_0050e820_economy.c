
uint __fastcall FUN_0050e820(int *param_1)

{
  bool bVar1;
  uint uVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  void *this;
  uint *puVar6;
  uint local_108;
  undefined4 local_104;
  undefined4 local_100;
  undefined4 local_fc;
  undefined4 local_f8;
  undefined4 local_f4;
  undefined4 local_f0;
  undefined4 local_ec;
  undefined4 local_e8;
  undefined4 local_e4;
  undefined4 local_e0;
  undefined4 local_dc;
  undefined4 local_d8;
  undefined4 local_d4;
  undefined4 local_d0;
  undefined4 local_cc [7];
  int local_b0;
  undefined4 local_ac [7];
  int local_90;
  undefined4 local_8c [7];
  int local_70;
  undefined4 local_6c [7];
  int local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642004;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  uVar2 = FUN_004f9700(param_1);
  if (uVar2 != 0) {
    if ((*(byte *)(param_1 + 0x14) & 8) == 0) {
      iVar3 = FUN_0055bba0(param_1,(uint)param_1[9] >> 6 & 3,4);
      iVar4 = FUN_0055b820(param_1,(uint)param_1[9] >> 6 & 3,4);
      iVar5 = FUN_0055b480(param_1,(uint)param_1[9] >> 6 & 3,4);
      if (((iVar3 == 1) && (iVar4 == 1)) && (iVar5 == 1)) {
        uVar2 = 1;
      }
      else {
        uVar2 = 0;
      }
    }
    if ((uVar2 != 0) && (uVar2 = FUN_0053fc90(param_1[0x18],0,param_1[0x17]), uVar2 != 0)) {
      uVar2 = FUN_0053fc90(param_1[0x1a],0,param_1[0x19]);
    }
  }
  FUN_004fcd00(local_2c,param_1,4);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while (local_10 != 0) {
    if (uVar2 == 0) goto LAB_0050edcc;
    puVar6 = &local_108;
    this = (void *)FUN_0052bed0((int)local_2c);
    puVar6 = FUN_004025b0(this,puVar6);
    uVar2 = *puVar6 >> 0x18;
    FUN_00619730();
    local_fc = 0x20;
    local_f8 = 0x30;
    local_4 = CONCAT31(local_4._1_3_,1);
    if ((uVar2 < 0x20) || (0x2f < uVar2)) {
      local_e4 = 0x30;
      local_e0 = 0x40;
      if ((uVar2 < 0x30) || (0x3f < uVar2)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) goto LAB_0050ea9d;
      local_d4 = 0x1c;
      local_d0 = 0x20;
      if ((uVar2 < 0x1c) || (0x1f < uVar2)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) goto LAB_0050ea9d;
      local_ec = 8;
      local_e8 = 0x10;
      if ((uVar2 < 8) || (0xf < uVar2)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) goto LAB_0050ea9d;
      local_dc = 0xa0;
      local_d8 = 0xb0;
      if ((uVar2 < 0xa0) || (0xaf < uVar2)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) goto LAB_0050ea9d;
      local_f4 = 0x40;
      local_f0 = 0x80;
      if ((uVar2 < 0x40) || (0x7f < uVar2)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) goto LAB_0050ea9d;
      local_104 = 0x10;
      local_100 = 0x14;
      if ((uVar2 < 0x10) || (0x13 < uVar2)) {
        uVar2 = 0;
      }
      else {
        uVar2 = 1;
      }
      FUN_00619730();
      if (uVar2 != 0) goto LAB_0050ea9d;
    }
    else {
LAB_0050ea9d:
      uVar2 = 1;
    }
    local_4 = local_4 & 0xffffff00;
    FUN_00619730();
    FUN_005130d0((int)local_2c);
  }
  if (uVar2 != 0) {
    local_108 = 0;
    uVar2 = FUN_005093e0(param_1,&local_108);
    if ((uVar2 != 0) && (uVar2 = (uint)((int)local_108 < 2), uVar2 != 0)) {
      uVar2 = 1;
      FUN_00539d70(local_ac,param_1,4);
      local_4 = CONCAT31(local_4._1_3_,2);
      FUN_00513120((int)local_ac);
      while ((local_90 != 0 && (uVar2 != 0))) {
        iVar3 = FUN_0052bed0((int)local_ac);
        local_108 = *(uint *)(iVar3 + 0x50) >> 3 & 1;
        if (local_108 == 0) {
LAB_0050eb80:
          iVar3 = FUN_0052bed0((int)local_ac);
          uVar2 = (uint)((((byte)*(undefined4 *)(iVar3 + 0x24) ^ (byte)param_1[9]) & 0xc0) == 0);
        }
        else {
          iVar3 = FUN_0052bed0((int)local_ac);
          local_108 = *(uint *)(iVar3 + 0x50) >> 0xe & 1;
          if (local_108 == 0) goto LAB_0050eb80;
        }
        FUN_005130d0((int)local_ac);
      }
      FUN_00558480(local_6c,param_1,4);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00513120((int)local_6c);
      while ((local_50 != 0 && (uVar2 != 0))) {
        iVar3 = FUN_0052bed0((int)local_6c);
        local_108 = *(uint *)(iVar3 + 0x50) >> 3 & 1;
        if (local_108 == 0) {
LAB_0050ec16:
          iVar3 = FUN_0052bed0((int)local_6c);
          uVar2 = (uint)((((byte)*(undefined4 *)(iVar3 + 0x24) ^ (byte)param_1[9]) & 0xc0) == 0);
        }
        else {
          iVar3 = FUN_0052bed0((int)local_6c);
          local_108 = *(uint *)(iVar3 + 0x50) >> 0xe & 1;
          if (local_108 == 0) goto LAB_0050ec16;
        }
        FUN_005130d0((int)local_6c);
      }
      FUN_0052c170(local_8c,param_1,4);
      local_4 = CONCAT31(local_4._1_3_,4);
      FUN_00513120((int)local_8c);
      while ((local_70 != 0 && (uVar2 != 0))) {
        iVar3 = FUN_0052bed0((int)local_8c);
        local_108 = *(uint *)(iVar3 + 0x50) >> 3 & 1;
        if (local_108 == 0) {
LAB_0050ecb2:
          iVar3 = FUN_0052bed0((int)local_8c);
          uVar2 = (uint)((((byte)*(undefined4 *)(iVar3 + 0x24) ^ (byte)param_1[9]) & 0xc0) == 0);
        }
        else {
          iVar3 = FUN_0052bed0((int)local_8c);
          local_108 = *(uint *)(iVar3 + 0x50) >> 0xe & 1;
          if (local_108 == 0) goto LAB_0050ecb2;
        }
        FUN_005130d0((int)local_8c);
      }
      FUN_00536da0(local_cc,param_1,4);
      local_4 = CONCAT31(local_4._1_3_,5);
      FUN_00513120((int)local_cc);
      while ((local_b0 != 0 && (uVar2 != 0))) {
        iVar3 = FUN_0052bed0((int)local_cc);
        local_108 = *(uint *)(iVar3 + 0x50) >> 3 & 1;
        if (local_108 == 0) {
LAB_0050ed3f:
          iVar3 = FUN_0052bed0((int)local_cc);
          local_108 = *(uint *)(iVar3 + 0x78) >> 7 & 1;
          if (local_108 == 0) {
            iVar3 = FUN_0052bed0((int)local_cc);
            uVar2 = (uint)((((byte)*(undefined4 *)(iVar3 + 0x24) ^ (byte)param_1[9]) & 0xc0) == 0);
          }
        }
        else {
          iVar3 = FUN_0052bed0((int)local_cc);
          local_108 = *(uint *)(iVar3 + 0x50) >> 0xe & 1;
          if (local_108 == 0) goto LAB_0050ed3f;
        }
        FUN_005130d0((int)local_cc);
      }
      local_4._1_3_ = (uint3)(local_4 >> 8);
      local_4._0_1_ = 4;
      FUN_00536ea0(local_cc);
      local_4._0_1_ = 3;
      FUN_0052c1f0(local_8c);
      local_4._0_1_ = 2;
      FUN_00558500(local_6c);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00539e70(local_ac);
    }
  }
LAB_0050edcc:
  FUN_004ffe70(local_4c,param_1,3);
  local_4 = CONCAT31(local_4._1_3_,6);
  FUN_00513120((int)local_4c);
  while ((uVar2 != 0 && (local_30 != 0))) {
    iVar3 = FUN_0052bed0((int)local_4c);
    local_108 = *(uint *)(iVar3 + 0x50) & 1;
    if (local_108 == 0) {
      uVar2 = 0;
    }
    else {
      uVar2 = (uint)param_1[0x22] >> 5 & 1;
    }
    iVar3 = FUN_0052bed0((int)local_4c);
    uVar2 = (uint)((*(uint *)(iVar3 + 0x58) >> 5 & 1) == uVar2);
    FUN_005130d0((int)local_4c);
  }
  local_4 = local_4 & 0xffffff00;
  FUN_004fff70(local_4c);
  local_4 = 0xffffffff;
  FUN_004fcd80(local_2c);
  ExceptionList = pvStack_c;
  return uVar2;
}

