
bool __cdecl FUN_00515480(uint *param_1,uint param_2,void *param_3)

{
  uint uVar1;
  bool bVar2;
  undefined1 uVar3;
  bool bVar4;
  int *this;
  uint uVar5;
  void *pvVar6;
  undefined3 extraout_var;
  int iVar7;
  undefined4 *puVar8;
  int *piVar9;
  uint *puVar10;
  uint *puVar11;
  uint uStack_24;
  uint uStack_20;
  void *pvStack_1c;
  uint uStack_18;
  uint local_14;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puVar11 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00642710;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this = (int *)FUN_00504dc0(param_1);
  if (this == (int *)0x0) {
    ExceptionList = local_c;
    return true;
  }
  if ((*(byte *)(this + 0x14) & 8) != 0) {
    ExceptionList = local_c;
    return true;
  }
  local_14 = 0x30;
  local_10 = 0x40;
  local_4 = 0;
  uVar5 = (**(code **)(*this + 4))();
  if (((uVar5 < local_14) || (local_10 <= uVar5)) || ((*(byte *)(this + 0x14) & 4) == 0)) {
    bVar4 = false;
  }
  else {
    bVar4 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar4) {
    pvVar6 = (void *)thunk_FUN_00506e40();
    if (pvVar6 == (void *)0x0) {
      ExceptionList = local_c;
      return pvVar6 != (void *)0x0;
    }
    bVar4 = FUN_00545df0(pvVar6,puVar11,param_2,param_3);
    if ((CONCAT31(extraout_var,bVar4) != 0) && (pvVar6 != (void *)0x0)) {
      ExceptionList = local_c;
      return true;
    }
    ExceptionList = local_c;
    return false;
  }
  if (param_2 == 0) {
    ExceptionList = local_c;
    return true;
  }
  pvVar6 = (void *)thunk_FUN_00506e60();
  bVar4 = pvVar6 != (void *)0x0;
  FUN_004ece30(&param_2);
  local_4 = 1;
  FUN_004ece30(&uStack_20);
  local_4._0_1_ = 2;
  FUN_004ece30(&uStack_24);
  local_4._0_1_ = 3;
  uVar3 = (undefined1)local_4;
  local_4._0_1_ = 3;
  if ((this[0x14] & 4U) == 0) {
    pvStack_1c = (void *)0x0;
    param_1 = (uint *)0x0;
    local_4._0_1_ = uVar3;
    uVar5 = FUN_00555540(this,(int *)&pvStack_1c);
    if ((uVar5 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    piVar9 = (int *)FUN_00504dc0((uint *)(this + 0xe));
    if ((piVar9 == (int *)0x0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    if ((pvVar6 != (void *)0x0) && (piVar9 != (int *)0x0)) {
      uVar5 = FUN_00555540(piVar9,(int *)&param_1);
      if ((uVar5 == 0) || (!bVar4)) {
        bVar4 = false;
      }
      else {
        bVar4 = true;
      }
    }
    uVar3 = (undefined1)local_4;
    if ((pvStack_1c == (void *)0x0) || (param_1 == (uint *)0x0)) goto LAB_0051572b;
    puVar11 = FUN_004025b0(pvStack_1c,&local_14);
    local_4._0_1_ = 5;
    puVar10 = FUN_004025b0(param_1,&uStack_18);
    uVar5 = *puVar10;
    uVar1 = *puVar11;
    FUN_00619730();
    local_4._0_1_ = 3;
    FUN_00619730();
    uVar3 = (undefined1)local_4;
    if (uVar5 == uVar1) goto LAB_0051572b;
    puVar11 = FUN_004025b0(param_1,&local_14);
    local_4._0_1_ = 6;
    FUN_004f26d0(&param_2,puVar11);
  }
  else {
    if ((this[0x14] & 0x20U) == 0) goto LAB_0051572b;
    iVar7 = FUN_004f6e30(this,&param_2);
    if ((iVar7 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    iVar7 = FUN_00540450(this,&uStack_20);
    if ((iVar7 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    uVar3 = (undefined1)local_4;
    if (param_2 != uStack_20) goto LAB_0051572b;
    FUN_004ece80(&param_2);
    puVar8 = (undefined4 *)FUN_004ece30(&param_1);
    local_4._0_1_ = 4;
    iVar7 = FUN_004f7fa0(this,puVar8);
    if ((iVar7 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
  }
  local_4._0_1_ = 3;
  FUN_00619730();
  uVar3 = (undefined1)local_4;
LAB_0051572b:
  local_4._0_1_ = uVar3;
  puVar11 = (uint *)FUN_0050c640(this);
  local_14 = 0x90;
  local_10 = 0x98;
  local_4._0_1_ = 7;
  if ((*puVar11 >> 0x18 < 0x90) || (0x97 < *puVar11 >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  local_4._0_1_ = 3;
  FUN_00619730();
  if ((bVar2) && (bVar4)) {
    bVar4 = true;
  }
  else {
    bVar4 = false;
  }
  if (pvVar6 != (void *)0x0) {
    if (bVar2) {
      FUN_00520570(&local_14);
      local_4._0_1_ = 8;
      puVar11 = (uint *)FUN_0050c640(this);
      local_4._0_1_ = 9;
      uVar5 = FUN_005545e0(pvVar6,this,puVar11,&param_2,(uint *)0x0,1,&uStack_24,(int *)&local_14);
      if ((uVar5 == 0) || (!bVar4)) {
        bVar4 = false;
      }
      else {
        bVar4 = true;
      }
      local_4 = CONCAT31(local_4._1_3_,8);
      FUN_00619730();
      if (local_10 != -1) {
        iVar7 = (**(code **)(*this + 0xac))(2,param_3);
        if ((iVar7 == 0) || (!bVar4)) {
          bVar4 = false;
        }
        else {
          bVar4 = true;
        }
      }
      local_4._0_1_ = 3;
      FUN_00619730();
    }
    if (pvVar6 != (void *)0x0) {
      if (((this[0x14] & 8U) == 0) && ((this[0x14] & 4U) != 0)) {
        param_1 = (uint *)0x1;
        puVar11 = FUN_004025b0(this,&local_14);
        local_4._0_1_ = 10;
        uVar5 = FUN_00555410(puVar11,&uStack_24,(uint *)&param_1);
        if ((uVar5 == 0) || (!bVar4)) {
          bVar4 = false;
        }
        else {
          bVar4 = true;
        }
        local_4._0_1_ = 3;
        FUN_00619730();
        if ((param_1 == (uint *)0x0) &&
           (iVar7 = (**(code **)(*this + 0x34))((*(byte *)(this + 0x14) & 0x20) != 0), iVar7 == 0))
        {
          iVar7 = (**(code **)(*this + 0xac))(5,param_3);
          if ((iVar7 == 0) || (!bVar4)) {
            bVar4 = false;
          }
          else {
            bVar4 = true;
          }
        }
      }
      if ((pvVar6 != (void *)0x0) && ((*(byte *)(this + 0x14) & 8) == 0)) {
        puVar11 = FUN_004025b0(this,(uint *)&param_1);
        local_4._0_1_ = 0xb;
        iVar7 = FUN_00556390(puVar11,&uStack_24,param_3);
        if ((iVar7 == 0) || (!bVar4)) {
          bVar4 = false;
        }
        else {
          bVar4 = true;
        }
        local_4._0_1_ = 3;
        FUN_00619730();
      }
    }
  }
  local_4._0_1_ = 2;
  FUN_00619730();
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return bVar4;
}

