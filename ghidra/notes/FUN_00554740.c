
uint FUN_00554740(int *param_1,uint *param_2,int *param_3,int *param_4)

{
  bool bVar1;
  uint uVar2;
  undefined3 extraout_var;
  uint uVar3;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  int *unaff_EBX;
  void *local_38;
  undefined4 local_34;
  uint local_30;
  void *local_2c;
  uint local_28;
  uint uStack_24;
  uint uStack_20;
  uint uStack_1c;
  uint uStack_18;
  uint local_14;
  void *local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00649310;
  pvStack_c = ExceptionList;
  local_14 = 1;
  local_10 = (void *)0x1;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_4,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_38 = (void *)0x0;
  local_34 = 0;
  local_28 = 0;
  FUN_004ece30(&local_2c);
  local_4 = 1;
  uVar2 = (**(code **)(*param_1 + 0xc))(&local_2c);
  if (uVar2 == 0) goto LAB_00554a79;
  uStack_18 = 0x98;
  local_14 = 0xa0;
  if ((local_30 >> 0x18 < 0x98) || (0x9f < local_30 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) goto LAB_00554a79;
  bVar1 = FUN_004f6b50(param_1,(int *)&stack0xffffffc4);
  uVar2 = CONCAT31(extraout_var,bVar1);
  if (uVar2 == 0) goto LAB_00554a79;
  if (param_4[1] != -1) {
    uStack_18 = 0x14;
    local_14 = 0x1c;
    puStack_8._0_1_ = 2;
    uVar3 = (**(code **)(*unaff_EBX + 4))();
    if ((uVar3 < uStack_18) || (local_14 <= uVar3)) {
      local_28 = 8;
      uStack_24 = 0x10;
      puStack_8._0_1_ = 3;
      uVar3 = (**(code **)(*unaff_EBX + 4))();
      if ((uVar3 < local_28) || (uStack_24 <= uVar3)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      puStack_8._0_1_ = 2;
      FUN_00619730();
      param_4 = param_3;
      if (bVar1) goto LAB_005548f1;
      uStack_20 = 0x90;
      uStack_1c = 0x98;
      puStack_8._0_1_ = 4;
      uVar3 = (**(code **)(*unaff_EBX + 4))();
      if ((uVar3 < uStack_20) || (uStack_1c <= uVar3)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      puStack_8._0_1_ = 2;
      FUN_00619730();
      if (bVar1) goto LAB_005548f1;
      bVar1 = false;
    }
    else {
LAB_005548f1:
      bVar1 = true;
    }
    puStack_8 = (undefined1 *)CONCAT31(puStack_8._1_3_,1);
    FUN_00619730();
    if (bVar1) {
      uVar2 = FUN_005571c0(param_1,unaff_EBX,(int)param_1,param_2,param_4);
    }
  }
  if (uVar2 == 0) goto LAB_00554a79;
  if (param_4[1] != -1) {
    uStack_20 = 0x14;
    uStack_1c = 0x1c;
    puStack_8._0_1_ = 5;
    uVar3 = (**(code **)(*unaff_EBX + 4))();
    if ((uVar3 < uStack_20) || (uStack_1c <= uVar3)) {
      uStack_18 = 8;
      local_14 = 0x10;
      puStack_8._0_1_ = 6;
      uVar3 = (**(code **)(*unaff_EBX + 4))();
      if ((uVar3 < uStack_18) || (local_14 <= uVar3)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      puStack_8._0_1_ = 5;
      FUN_00619730();
      param_4 = param_3;
      if (bVar1) goto LAB_005549ae;
      bVar1 = false;
    }
    else {
LAB_005549ae:
      bVar1 = true;
    }
    puStack_8 = (undefined1 *)CONCAT31(puStack_8._1_3_,1);
    FUN_00619730();
    if (bVar1) {
      bVar1 = FUN_004f6b50(unaff_EBX,(int *)&local_38);
      uVar2 = 0;
      if (CONCAT31(extraout_var_00,bVar1) == 0) goto LAB_00554a79;
      uVar2 = FUN_005571c0(param_1,local_38,(int)param_1,param_2,param_4);
    }
  }
  if ((uVar2 != 0) && (param_4[1] != -1)) {
    uStack_18 = 0x14;
    local_14 = 0x1c;
    puStack_8._0_1_ = 7;
    uVar3 = (**(code **)(*unaff_EBX + 4))();
    if ((uVar3 < uStack_18) || (local_14 <= uVar3)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    puStack_8 = (undefined1 *)CONCAT31(puStack_8._1_3_,1);
    FUN_00619730();
    if (bVar1) {
      bVar1 = FUN_004f6b50(local_38,(int *)&local_2c);
      uVar2 = 0;
      if (CONCAT31(extraout_var_01,bVar1) != 0) {
        uVar2 = FUN_005571c0(param_1,local_2c,(int)param_1,param_2,param_3);
      }
    }
  }
LAB_00554a79:
  puStack_8 = (undefined1 *)0xffffffff;
  FUN_00619730();
  ExceptionList = local_10;
  return uVar2;
}

