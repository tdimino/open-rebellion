
undefined4 __fastcall FUN_00480370(int param_1)

{
  uint *this;
  int iVar1;
  bool bVar2;
  uint uVar3;
  void *pvVar4;
  undefined4 *puVar5;
  void *pvVar6;
  undefined4 local_1c;
  undefined4 local_18;
  undefined1 *local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00633fb8;
  local_c = ExceptionList;
  bVar2 = false;
  local_18 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_1c);
  local_4 = 0;
  FUN_004ece80((undefined4 *)(param_1 + 0x30));
  *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) & 0xffffffef;
  if ((*(byte *)(param_1 + 0x20) & 1) == 0) {
    FUN_004ece80((undefined4 *)(param_1 + 0x34));
LAB_00480697:
    if ((*(byte *)(param_1 + 0x20) & 0x10) == 0) goto LAB_004806a5;
  }
  else {
    this = (uint *)(param_1 + 0x34);
    uVar3 = *(uint *)(param_1 + 0x34) >> 0x18;
    local_14 = (undefined1 *)0x90;
    local_10 = 0x98;
    if ((0x8f < uVar3) && (uVar3 < 0x98)) {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      iVar1 = *(int *)(param_1 + 0x68);
      local_14 = &stack0xffffffd0;
      FUN_004f26d0(&stack0xffffffd0,this);
      pvVar4 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if (((pvVar4 != (void *)0x0) && ((*(uint *)((int)pvVar4 + 0x2c) & 0x100) != 0)) &&
         ((*(uint *)((int)pvVar4 + 0x30) & 0x200) == 0)) {
        FUN_004f26d0((undefined4 *)(param_1 + 0x30),this);
        *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) | 0x10;
      }
    }
    if ((*(byte *)(param_1 + 0x20) & 0x10) == 0) {
      FUN_004ece80(this);
      uVar3 = *(uint *)(param_1 + 0x38) >> 0x18;
      local_14 = (undefined1 *)0x90;
      local_10 = 0x98;
      if ((uVar3 < 0x90) || (0x97 < uVar3)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (bVar2) {
        puVar5 = FUN_00419b80(*(void **)(param_1 + 0x68),(undefined4 *)(param_1 + 0x38),0,0x200,8,0,
                              0,0,2);
        pvVar4 = (void *)(param_1 + 0x54);
        FUN_00435790(pvVar4,(int)puVar5);
        puVar5 = (undefined4 *)FUN_004357b0(pvVar4,&local_14);
        local_4._0_1_ = 1;
        FUN_004f26d0(&local_1c,puVar5);
        local_4._0_1_ = 0;
        FUN_00619730();
        FUN_005f58b0(pvVar4);
        puVar5 = FUN_0041a400(*(void **)(param_1 + 0x68),&local_1c,(undefined4 *)(param_1 + 0x38),0,
                              0x100,4,0,0,0,(undefined4 *)&DAT_00000002);
        FUN_00435790(pvVar4,(int)puVar5);
        puVar5 = (undefined4 *)FUN_004357b0(pvVar4,&local_14);
        local_4._0_1_ = 2;
        FUN_004f26d0(this,puVar5);
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00619730();
        FUN_005f58b0(pvVar4);
      }
      local_14 = (undefined1 *)0x90;
      local_10 = 0x98;
      if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (!bVar2) {
        pvVar6 = FUN_00419a00(*(void **)(param_1 + 0x68),0,0x200,8,0,0,0,4,(void *)0x1);
        pvVar4 = (void *)(param_1 + 0x54);
        FUN_00435790(pvVar4,(int)pvVar6);
        pvVar6 = FUN_0041a1d0(*(void **)(param_1 + 0x68),0,0x200,8,0,0,0,(void *)0x1);
        FUN_00435790(pvVar4,(int)pvVar6);
        puVar5 = (undefined4 *)FUN_004357b0(pvVar4,&local_14);
        local_4._0_1_ = 3;
        FUN_004f26d0(&local_1c,puVar5);
        local_4._0_1_ = 0;
        FUN_00619730();
        FUN_005f58b0(pvVar4);
        pvVar6 = FUN_0041a340(*(void **)(param_1 + 0x68),&local_1c,0,0x100,4,0,0,0,0,(void *)0x1);
        FUN_00435790(pvVar4,(int)pvVar6);
        puVar5 = (undefined4 *)FUN_004357b0(pvVar4,&local_14);
        local_4._0_1_ = 4;
        FUN_004f26d0(this,puVar5);
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00619730();
        FUN_005f58b0(pvVar4);
      }
      local_14 = (undefined1 *)0x90;
      local_10 = 0x98;
      if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (bVar2) {
        FUN_004f26d0((void *)(param_1 + 0x30),this);
        *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) | 0x10;
      }
      goto LAB_00480697;
    }
  }
  local_18 = 1;
LAB_004806a5:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return local_18;
}

