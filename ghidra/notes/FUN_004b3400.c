
void * __thiscall FUN_004b3400(void *this,void *param_1)

{
  int iVar1;
  bool bVar2;
  short sVar3;
  undefined1 *puVar4;
  uint uVar5;
  void *pvVar6;
  undefined4 *puVar7;
  void *pvVar8;
  uint local_20;
  undefined1 *local_1c;
  undefined4 local_18;
  undefined1 *local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006391cf;
  local_c = ExceptionList;
  local_10 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_20);
  iVar1 = *(int *)((int)this + 0xc);
  local_14 = &stack0xffffffcc;
  local_4 = 1;
  FUN_004f26d0(&stack0xffffffcc,(undefined4 *)((int)this + 0x28));
  puVar4 = (undefined1 *)FUN_00419e40((void *)(iVar1 + 0x78));
  local_14 = puVar4;
  if (puVar4 != (undefined1 *)0x0) {
    sVar3 = FUN_005f50e0(*(int *)((int)this + 0x10) + 0x2c);
    if (sVar3 == 1) {
      local_1c = (undefined1 *)0x90;
      local_18 = 0x98;
      uVar5 = *(uint *)(*(int *)((int)this + 0x10) + 0x40) >> 0x18;
      if ((uVar5 < 0x90) || (0x97 < uVar5)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if ((!bVar2) && ((*(uint *)(puVar4 + 0x38) & 0xa00) == 0)) {
        pvVar6 = FUN_0041a460(*(void **)((int)this + 0xc),
                              (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0,0,0,0,0x8000000,
                              0x700201,(void *)0x1);
        pvVar8 = (void *)((int)this + 0x18);
        FUN_00435790(pvVar8,(int)pvVar6);
        puVar7 = (undefined4 *)FUN_004357b0(pvVar8,&local_1c);
        local_4._0_1_ = 2;
        FUN_004f26d0(&local_20,puVar7);
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_00619730();
        FUN_005f58b0(pvVar8);
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
      iVar1 = *(int *)((int)this + 0xc);
      local_1c = &stack0xffffffcc;
      FUN_004f26d0(&stack0xffffffcc,(undefined4 *)(*(int *)((int)this + 0x10) + 0x40));
      pvVar8 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if ((pvVar8 != (void *)0x0) && ((*(byte *)((int)pvVar8 + 0x28) & 2) == 0)) {
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
      pvVar6 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0x80000000,0,1,2,0,0,7
                            ,&DAT_00000002);
      pvVar8 = (void *)((int)this + 0x18);
      FUN_00435790(pvVar8,(int)pvVar6);
      puVar7 = (undefined4 *)FUN_004357b0(pvVar8,&local_1c);
      local_4._0_1_ = 3;
      FUN_004f26d0(&local_20,puVar7);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
      FUN_005f58b0(pvVar8);
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
      pvVar6 = FUN_0041a340(*(void **)((int)this + 0xc),
                            (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),0x3e00000,0,1,2,0,0,
                            0x21,(void *)0x1);
      pvVar8 = (void *)((int)this + 0x18);
      FUN_00435790(pvVar8,(int)pvVar6);
      puVar7 = (undefined4 *)FUN_004357b0(pvVar8,&local_1c);
      local_4._0_1_ = 4;
      FUN_004f26d0(&local_20,puVar7);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
      FUN_005f58b0(pvVar8);
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
      puVar7 = (undefined4 *)(local_14 + 0x28);
      local_14 = &stack0xffffffcc;
      FUN_004f26d0(&stack0xffffffcc,puVar7);
      pvVar8 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if ((pvVar8 != (void *)0x0) && ((*(byte *)((int)pvVar8 + 0x30) & 4) != 0)) {
        puVar7 = FUN_0041a400(*(void **)((int)this + 0xc),
                              (undefined4 *)(*(int *)((int)this + 0x10) + 0x34),
                              (undefined4 *)((int)this + 0x28),0,0,0,0,0,0x204,
                              (undefined4 *)&DAT_00000002);
        pvVar8 = (void *)((int)this + 0x18);
        FUN_00435790(pvVar8,(int)puVar7);
        puVar7 = (undefined4 *)FUN_004357b0(pvVar8,&local_14);
        local_4._0_1_ = 5;
        FUN_004f26d0(&local_20,puVar7);
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_00619730();
        FUN_005f58b0(pvVar8);
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

