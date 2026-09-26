
void * __thiscall FUN_004b4a20(void *this,void *param_1)

{
  int iVar1;
  bool bVar2;
  void *pvVar3;
  void *pvVar4;
  undefined4 *puVar5;
  uint local_20;
  undefined1 *local_1c;
  undefined4 local_18;
  undefined1 *local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639337;
  local_c = ExceptionList;
  local_10 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_20);
  iVar1 = *(int *)((int)this + 0xc);
  local_14 = &stack0xffffffcc;
  local_4 = 1;
  FUN_004f26d0(&stack0xffffffcc,(undefined4 *)((int)this + 0x28));
  local_14 = FUN_00419e40((void *)(iVar1 + 0x78));
  if (local_14 != (undefined1 *)0x0) {
    local_1c = (undefined1 *)0x90;
    local_18 = 0x98;
    if ((local_20 >> 0x18 < 0x90) || (0x97 < local_20 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      iVar1 = *(int *)((int)this + 0xc);
      local_1c = &stack0xffffffcc;
      FUN_004f26d0(&stack0xffffffcc,(undefined4 *)(*(int *)((int)this + 0x10) + 0x40));
      pvVar3 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if ((pvVar3 != (void *)0x0) && ((*(byte *)((int)pvVar3 + 0x28) & 2) == 0)) {
        FUN_004f26d0(&local_20,(undefined4 *)(*(int *)((int)this + 0x10) + 0x40));
      }
    }
    local_1c = (undefined1 *)0x90;
    local_18 = 0x98;
    if ((local_20 >> 0x18 < 0x90) || (0x97 < local_20 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      pvVar4 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0x80000000,0,1,2,0,0,7
                            ,&DAT_00000002);
      pvVar3 = (void *)((int)this + 0x18);
      FUN_00435790(pvVar3,(int)pvVar4);
      puVar5 = FUN_004357b0(pvVar3,&local_1c);
      local_4._0_1_ = 2;
      FUN_004f26d0(&local_20,puVar5);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
      FUN_005f58b0(pvVar3);
    }
    local_1c = (undefined1 *)0x90;
    local_18 = 0x98;
    if ((local_20 >> 0x18 < 0x90) || (0x97 < local_20 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      pvVar4 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0x3e00000,0,1,2,0,0,
                            0x21,(void *)0x1);
      pvVar3 = (void *)((int)this + 0x18);
      FUN_00435790(pvVar3,(int)pvVar4);
      puVar5 = FUN_004357b0(pvVar3,&local_1c);
      local_4._0_1_ = 3;
      FUN_004f26d0(&local_20,puVar5);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
      FUN_005f58b0(pvVar3);
    }
    local_1c = (undefined1 *)0x90;
    local_18 = 0x98;
    if ((local_20 >> 0x18 < 0x90) || (0x97 < local_20 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      iVar1 = *(int *)((int)this + 0xc);
      puVar5 = (undefined4 *)(local_14 + 0x28);
      local_14 = &stack0xffffffcc;
      FUN_004f26d0(&stack0xffffffcc,puVar5);
      pvVar3 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if ((pvVar3 != (void *)0x0) && ((*(byte *)((int)pvVar3 + 0x30) & 4) != 0)) {
        puVar5 = FUN_0041a400(*(void **)((int)this + 0xc),
                              (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),
                              (undefined4 *)((int)this + 0x28),0,0,0,0,0,0x204,
                              (undefined4 *)&DAT_00000002);
        pvVar3 = (void *)((int)this + 0x18);
        FUN_00435790(pvVar3,(int)puVar5);
        puVar5 = FUN_004357b0(pvVar3,&local_14);
        local_4._0_1_ = 4;
        FUN_004f26d0(&local_20,puVar5);
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_00619730();
        FUN_005f58b0(pvVar3);
      }
    }
  }
  FUN_004f26d0(param_1,&local_20);
  local_10 = 1;
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  ExceptionList = local_c;
  return param_1;
}

