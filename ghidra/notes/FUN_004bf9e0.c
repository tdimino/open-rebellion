
undefined4 __fastcall FUN_004bf9e0(int param_1)

{
  void *this;
  uint *this_00;
  bool bVar1;
  int *piVar2;
  void *pvVar3;
  undefined4 *puVar4;
  int iVar5;
  undefined4 uVar6;
  uint local_28;
  uint local_24;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063a168;
  local_c = ExceptionList;
  local_20 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_28);
  local_24 = 0x20000005;
  local_4._0_1_ = 1;
  local_4._1_3_ = 0;
  piVar2 = FUN_004f3330(*(int *)(param_1 + 0x30),&local_24);
  local_4._0_1_ = 0;
  FUN_00619730();
  uVar6 = 0;
  if ((piVar2 == (int *)0x0) || ((*(byte *)(piVar2 + 0x14) & 0x10) != 0)) goto LAB_004bfdfc;
  pvVar3 = FUN_00419a00(*(void **)(param_1 + 0x2c),0,0,0x80000000,0,0,0,3,(void *)0x1);
  this = (void *)(param_1 + 0x40);
  FUN_00435790(this,(int)pvVar3);
  puVar4 = FUN_004357b0(this,&local_24);
  local_4._0_1_ = 2;
  FUN_004f26d0(&local_28,puVar4);
  local_4._0_1_ = 0;
  FUN_00619730();
  FUN_005f58b0(this);
  pvVar3 = FUN_0041a340(*(void **)(param_1 + 0x2c),&local_28,0,0,0x80000000,2,0,0,0,(void *)0x1);
  FUN_00435790(this,(int)pvVar3);
  puVar4 = FUN_004357b0(this,&local_24);
  this_00 = (uint *)(param_1 + 0x50);
  local_4._0_1_ = 3;
  FUN_004f26d0(this_00,puVar4);
  local_4._0_1_ = 0;
  FUN_00619730();
  FUN_005f58b0(this);
  local_14 = 0x80;
  local_10 = 0x90;
  local_4 = CONCAT31(local_4._1_3_,4);
  if ((local_28 >> 0x18 < 0x80) || (0x8f < local_28 >> 0x18)) {
LAB_004bfb5c:
    bVar1 = false;
  }
  else {
    local_1c = 0x90;
    local_18 = 0x98;
    if ((*this_00 >> 0x18 < 0x90) || (local_24 = 1, 0x97 < *this_00 >> 0x18)) {
      local_24 = 0;
    }
    FUN_00619730();
    bVar1 = true;
    if (local_24 == 0) goto LAB_004bfb5c;
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  if (bVar1) {
    FUN_004ece80(this_00);
    pvVar3 = FUN_004f5940((void *)(*(int *)(param_1 + 0x2c) + 0x44),&local_28);
    if (pvVar3 != (void *)0x0) {
      if ((*(uint *)((int)pvVar3 + 0x2c) & 0x40000) == 0) {
        iVar5 = thunk_FUN_004fcf00();
        if (iVar5 < *(int *)(param_1 + 0x54)) goto LAB_004bfdf8;
      }
      pvVar3 = FUN_00419a00(*(void **)(param_1 + 0x2c),0,0,4,0,0,0x40000,4,&DAT_00000002);
      FUN_00435790(this,(int)pvVar3);
      puVar4 = FUN_004357b0(this,&local_24);
      local_4._0_1_ = 5;
      FUN_004f26d0(&local_28,puVar4);
      local_4._0_1_ = 0;
      FUN_00619730();
      FUN_005f58b0(this);
      local_14 = 0x80;
      local_10 = 0x90;
      if ((local_28 >> 0x18 < 0x80) || (0x8f < local_28 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (!bVar1) {
        pvVar3 = FUN_00419a00(*(void **)(param_1 + 0x2c),0,0,4,0,0,0,4,&DAT_00000002);
        FUN_00435790(this,(int)pvVar3);
        puVar4 = FUN_004357b0(this,&local_24);
        local_4._0_1_ = 6;
        FUN_004f26d0(&local_28,puVar4);
        local_4._0_1_ = 0;
        FUN_00619730();
        FUN_005f58b0(this);
      }
      local_14 = 0x80;
      local_10 = 0x90;
      if ((local_28 >> 0x18 < 0x80) || (local_24 = 1, 0x8f < local_28 >> 0x18)) {
        local_24 = 0;
      }
      FUN_00619730();
      if (local_24 != 0) {
        pvVar3 = FUN_0041a460(*(void **)(param_1 + 0x2c),&local_28,0x20,0,1,0,0,0x200,(void *)0x1);
        FUN_00435790(this,(int)pvVar3);
        puVar4 = FUN_004357b0(this,&local_24);
        local_4._0_1_ = 7;
        FUN_004f26d0(this_00,puVar4);
        local_4._0_1_ = 0;
        FUN_00619730();
        FUN_005f58b0(this);
        local_14 = 0x90;
        local_10 = 0x98;
        if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (!bVar1) {
          pvVar3 = FUN_0041a460(*(void **)(param_1 + 0x2c),&local_28,0,0,1,0,0,0x200,(void *)0x1);
          FUN_00435790(this,(int)pvVar3);
          puVar4 = FUN_004357b0(this,&local_24);
          local_4._0_1_ = 8;
          FUN_004f26d0(this_00,puVar4);
          local_4._0_1_ = 0;
          FUN_00619730();
          FUN_005f58b0(this);
        }
        local_14 = 0x90;
        local_10 = 0x98;
        if ((*this_00 >> 0x18 < 0x90) || (0x97 < *this_00 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        uVar6 = 1;
        if (bVar1) goto LAB_004bfdfc;
      }
    }
  }
LAB_004bfdf8:
  uVar6 = local_20;
LAB_004bfdfc:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar6;
}

