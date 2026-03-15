
bool __thiscall FUN_00508660(void *this,int *param_1,int param_2,void *param_3)

{
  bool bVar1;
  int iVar2;
  uint uVar3;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  bool bVar4;
  int iVar5;
  void *pvVar6;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641830;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar2 = FUN_00524b10();
  bVar4 = iVar2 != 0;
  local_4 = 0;
  uVar3 = (**(code **)(*param_1 + 4))();
  if ((uVar3 < 0x20) || (0x2f < uVar3)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    iVar2 = FUN_0050b760(this,param_3);
    if ((iVar2 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
  }
  local_4 = 1;
  uVar3 = (**(code **)(*param_1 + 4))();
  if ((uVar3 < 0x20) || (0x21 < uVar3)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    iVar5 = 1;
    pvVar6 = param_3;
    iVar2 = FUN_0055a0f0(param_2);
    bVar1 = FUN_0050c9f0(this,iVar2,iVar5,pvVar6);
    if ((CONCAT31(extraout_var,bVar1) == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
  }
  local_4 = 2;
  uVar3 = (**(code **)(*param_1 + 4))();
  if ((uVar3 < 0x22) || (0x22 < uVar3)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    iVar2 = FUN_0050b310(this,param_3);
    if ((iVar2 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
  }
  local_4 = 3;
  uVar3 = (**(code **)(*param_1 + 4))();
  if ((uVar3 < 0x2c) || (0x2f < uVar3)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  if ((bVar1) && (param_2 != 0)) {
    iVar2 = FUN_0050c560(this,param_1,param_3);
    if ((iVar2 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
  }
  local_4 = 4;
  uVar3 = (**(code **)(*param_1 + 4))();
  if ((uVar3 < 0x28) || (0x28 < uVar3)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    iVar2 = FUN_0050bb70(this,param_3);
    if ((iVar2 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    iVar2 = FUN_0050c180(this,param_3);
    if ((iVar2 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
  }
  local_4 = 5;
  uVar3 = (**(code **)(*param_1 + 4))();
  if ((uVar3 < 0x29) || (0x29 < uVar3)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    iVar2 = FUN_0050c1b0(this,param_3);
    if ((iVar2 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
  }
  if (bVar4) {
    local_4 = 6;
    uVar3 = (**(code **)(*param_1 + 4))();
    if ((uVar3 < 0x2a) || (0x2a < uVar3)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (bVar1) {
      iVar2 = FUN_0050c1e0(this,param_3);
      bVar4 = iVar2 != 0;
    }
    if (bVar4) {
      local_4 = 7;
      uVar3 = (**(code **)(*param_1 + 4))();
      if ((uVar3 < 8) || (0xf < uVar3)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      local_4 = 0xffffffff;
      FUN_00619730();
      if (bVar1) {
        iVar2 = FUN_0050ac80(this,param_3);
        iVar5 = FUN_0050bc60(this,param_3);
        if ((iVar5 == 0) || (iVar2 == 0)) {
          bVar4 = false;
        }
        else {
          bVar4 = true;
        }
        iVar2 = FUN_0050be00(this,param_3);
        if ((iVar2 == 0) || (!bVar4)) {
          bVar4 = false;
        }
        else {
          bVar4 = true;
        }
        if (param_2 != 0) {
          iVar2 = FUN_0050a1b0(this,(uint)param_1[9] >> 6 & 3,param_3);
          if ((iVar2 == 0) || (!bVar4)) {
            bVar4 = false;
          }
          else {
            bVar4 = true;
          }
        }
        iVar2 = FUN_0050c0b0(this,param_3);
        if ((iVar2 == 0) || (!bVar4)) {
          bVar4 = false;
        }
        else {
          bVar4 = true;
        }
        bVar1 = FUN_0050c6c0(this,param_1,param_2,param_3);
        if ((CONCAT31(extraout_var_00,bVar1) == 0) || (!bVar4)) {
          bVar4 = false;
        }
        else {
          bVar4 = true;
        }
        iVar2 = FUN_0050ac10(this,1,param_3);
        if ((iVar2 == 0) || (!bVar4)) {
          bVar4 = false;
        }
        else {
          bVar4 = true;
        }
      }
    }
  }
  local_4 = 8;
  uVar3 = (**(code **)(*param_1 + 4))();
  if ((uVar3 < 0x1c) || (0x1f < uVar3)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    if (param_2 != 0) {
      iVar2 = FUN_0050a1b0(this,(uint)param_1[9] >> 6 & 3,param_3);
      if ((iVar2 == 0) || (!bVar4)) {
        bVar4 = false;
      }
      else {
        bVar4 = true;
      }
    }
    bVar1 = FUN_0050c6c0(this,param_1,param_2,param_3);
    if ((CONCAT31(extraout_var_01,bVar1) == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    iVar2 = FUN_0050ac10(this,1,param_3);
    if ((iVar2 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
  }
  local_4 = 9;
  uVar3 = (**(code **)(*param_1 + 4))();
  if ((uVar3 < 0x10) || (0x13 < uVar3)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    bVar1 = FUN_0050b610(this,param_3);
    if ((CONCAT31(extraout_var_02,bVar1) == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    iVar2 = FUN_0050ba90(this,param_3);
    if ((iVar2 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    iVar2 = FUN_0050b500(this,param_3);
    if ((iVar2 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    if (param_2 != 0) {
      iVar2 = FUN_0050c540(this,param_1,param_3);
      if ((iVar2 == 0) || (!bVar4)) {
        bVar4 = false;
      }
      else {
        bVar4 = true;
      }
    }
  }
  ExceptionList = pvStack_c;
  return bVar4;
}

