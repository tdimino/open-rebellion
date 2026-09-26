
/* WARNING: Type propagation algorithm not settling */

void * __thiscall FUN_004b8ca0(void *this,void *param_1)

{
  int iVar1;
  bool bVar2;
  void *pvVar3;
  undefined4 *puVar4;
  void *pvVar5;
  uint local_20 [3];
  undefined1 *local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006397a7;
  local_c = ExceptionList;
  local_10 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(local_20);
  iVar1 = *(int *)((int)this + 0xc);
  local_14 = &stack0xffffffcc;
  local_4 = 1;
  FUN_004f26d0(&stack0xffffffcc,(undefined4 *)((int)this + 0x28));
  local_14 = FUN_00419e40((void *)(iVar1 + 0x78));
  if (local_14 != (undefined1 *)0x0) {
    local_20[1] = 0x90;
    local_20[2] = 0x98;
    if ((local_20[0] >> 0x18 < 0x90) || (0x97 < local_20[0] >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      FUN_004f26d0(local_20,(undefined4 *)(*(int *)((int)this + 0x10) + 0x40));
    }
    local_20[1] = 0x90;
    local_20[2] = 0x98;
    if ((local_20[0] >> 0x18 < 0x90) || (0x97 < local_20[0] >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      pvVar3 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0x80000000,0,1,0,0,0,7
                            ,&DAT_00000002);
      pvVar5 = (void *)((int)this + 0x18);
      FUN_00435790(pvVar5,(int)pvVar3);
      puVar4 = FUN_004357b0(pvVar5,local_20 + 1);
      local_4._0_1_ = 2;
      FUN_004f26d0(local_20,puVar4);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
      FUN_005f58b0(pvVar5);
    }
    local_20[1] = 0x90;
    local_20[2] = 0x98;
    if ((local_20[0] >> 0x18 < 0x90) || (0x97 < local_20[0] >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      pvVar3 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0x3e00000,0,1,0,0,0,
                            0x21,(void *)0x1);
      pvVar5 = (void *)((int)this + 0x18);
      FUN_00435790(pvVar5,(int)pvVar3);
      puVar4 = FUN_004357b0(pvVar5,local_20 + 1);
      local_4._0_1_ = 3;
      FUN_004f26d0(local_20,puVar4);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
      FUN_005f58b0(pvVar5);
    }
    local_20[1] = 0x90;
    local_20[2] = 0x98;
    if ((local_20[0] >> 0x18 < 0x90) || (0x97 < local_20[0] >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      iVar1 = *(int *)((int)this + 0xc);
      puVar4 = (undefined4 *)(local_14 + 0x28);
      local_14 = &stack0xffffffcc;
      FUN_004f26d0(&stack0xffffffcc,puVar4);
      pvVar5 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if ((pvVar5 != (void *)0x0) && ((*(byte *)((int)pvVar5 + 0x30) & 4) != 0)) {
        puVar4 = FUN_0041a400(*(void **)((int)this + 0xc),
                              (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),
                              (undefined4 *)((int)this + 0x28),0,0,0,0,0,0x204,
                              (undefined4 *)&DAT_00000002);
        pvVar5 = (void *)((int)this + 0x18);
        FUN_00435790(pvVar5,(int)puVar4);
        puVar4 = FUN_004357b0(pvVar5,&local_14);
        local_4._0_1_ = 4;
        FUN_004f26d0(local_20,puVar4);
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_00619730();
        FUN_005f58b0(pvVar5);
      }
    }
  }
  FUN_004f26d0(param_1,local_20);
  local_10 = 1;
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  ExceptionList = local_c;
  return param_1;
}

