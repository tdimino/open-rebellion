
int __fastcall FUN_004cf7c0(int param_1)

{
  bool bVar1;
  void *this;
  uint uVar2;
  void *pvVar3;
  undefined4 *puVar4;
  void *pvVar5;
  uint local_1c;
  int local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063bbc0;
  local_c = ExceptionList;
  bVar1 = false;
  local_18 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_1c);
  local_4 = 0;
  this = (void *)FUN_005f5500((void *)(*(int *)(param_1 + 0x2c) + 0xd8),*(uint *)(param_1 + 0x40));
  if (this == (void *)0x0) goto LAB_004cfa98;
  uVar2 = *(uint *)((int)this + 0x34) >> 0x18;
  local_14 = 0x80;
  local_10 = 0x90;
  if ((0x7f < uVar2) && (uVar2 < 0x90)) {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    pvVar3 = FUN_004f5940((void *)(*(int *)(param_1 + 0x2c) + 0x44),(uint *)((int)this + 0x34));
    if (((pvVar3 != (void *)0x0) && (*(int *)((int)pvVar3 + 0x18c) == 0)) &&
       (*(int *)((int)pvVar3 + 0x38) == *(int *)((int)this + 0x18))) {
      local_18 = 1;
    }
    if (local_18 == 0) {
      puVar4 = FUN_00419b80(*(void **)(param_1 + 0x2c),(uint *)((int)this + 0x34),0,0,0x10000,0,0,0,
                            2);
      pvVar3 = (void *)(param_1 + 0x48);
      FUN_00435790(pvVar3,(int)puVar4);
      puVar4 = FUN_004357b0(pvVar3,&local_14);
      local_4._0_1_ = 1;
      FUN_004f26d0(&local_1c,puVar4);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      FUN_005f58b0(pvVar3);
      FUN_00477190(this);
      local_14 = 0x80;
      local_10 = 0x90;
      if ((local_1c >> 0x18 < 0x80) || (0x8f < local_1c >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        FUN_004768b0(this,&local_1c);
        local_18 = 1;
      }
      goto LAB_004cf935;
    }
  }
  else {
LAB_004cf935:
    if (local_18 == 0) {
      pvVar5 = FUN_0041a1d0(*(void **)(param_1 + 0x2c),0,0,0x8010000,0,0,0,&DAT_00000002);
      pvVar3 = (void *)(param_1 + 0x48);
      FUN_00435790(pvVar3,(int)pvVar5);
      puVar4 = FUN_004357b0(pvVar3,&local_14);
      local_4._0_1_ = 2;
      FUN_004f26d0(&local_1c,puVar4);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_00619730();
      FUN_005f58b0(pvVar3);
      local_14 = 0x80;
      local_10 = 0x90;
      if ((local_1c >> 0x18 < 0x80) || (0x8f < local_1c >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        FUN_004768b0(this,&local_1c);
        local_18 = 1;
      }
      if (local_18 == 0) {
        pvVar3 = FUN_0041a1d0(*(void **)(param_1 + 0x2c),0,0,0x10000,0,0,0,&DAT_00000002);
        pvVar5 = (void *)(param_1 + 0x48);
        FUN_00435790(pvVar5,(int)pvVar3);
        puVar4 = FUN_004357b0(pvVar5,&local_14);
        local_4._0_1_ = 3;
        FUN_004f26d0(&local_1c,puVar4);
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00619730();
        FUN_005f58b0(pvVar5);
        local_14 = 0x80;
        local_10 = 0x90;
        if ((local_1c >> 0x18 < 0x80) || (0x8f < local_1c >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        if (bVar1) {
          FUN_004768b0(this,&local_1c);
          local_18 = 1;
        }
        if (local_18 == 0) goto LAB_004cfa98;
      }
    }
  }
  local_18 = FUN_004773f0(this);
LAB_004cfa98:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return local_18;
}

