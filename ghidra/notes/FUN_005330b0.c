
bool __fastcall FUN_005330b0(void *param_1)

{
  void *pvVar1;
  uint *puVar2;
  int iVar3;
  int iVar4;
  bool bVar5;
  uint local_30;
  uint local_2c;
  undefined4 local_28;
  undefined4 local_24 [4];
  undefined1 local_14 [4];
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00645780;
  local_c = ExceptionList;
  bVar5 = true;
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_00506960();
  while ((pvVar1 != (void *)0x0 && (bVar5))) {
    FUN_00583c70(local_24);
    local_4 = 1;
    puVar2 = FUN_004025b0(pvVar1,&local_30);
    local_4._0_1_ = 2;
    FUN_004f26d0(local_14,puVar2);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
    local_10 = FUN_0053a6e0((int)pvVar1);
    iVar3 = FUN_0052e4f0(param_1,local_24);
    iVar4 = 0;
    if ((*(int *)((int)pvVar1 + 0x58) == 1) &&
       (((*(uint *)((int)param_1 + 0x24) ^ *(uint *)((int)pvVar1 + 0x24)) & 0xc0) == 0)) {
      iVar4 = 1;
    }
    bVar5 = iVar4 == iVar3;
    local_4 = 0xffffffff;
    FUN_00583cf0(local_24);
    local_2c = 0x2d;
    local_28 = 0x2e;
    local_4 = 0;
    pvVar1 = (void *)FUN_004f6010(pvVar1,&local_2c,1);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  pvVar1 = (void *)FUN_005056e0();
  while ((pvVar1 != (void *)0x0 && (bVar5))) {
    FUN_00583c70(local_24);
    local_4 = 4;
    puVar2 = FUN_004025b0(pvVar1,&local_30);
    local_4._0_1_ = 5;
    FUN_004f26d0(local_14,puVar2);
    local_4 = CONCAT31(local_4._1_3_,4);
    FUN_00619730();
    local_10 = FUN_0053a6e0((int)pvVar1);
    iVar3 = FUN_0052e510(param_1,local_24);
    iVar4 = 0;
    if ((*(int *)((int)pvVar1 + 0x58) == 1) &&
       (((*(uint *)((int)param_1 + 0x24) ^ *(uint *)((int)pvVar1 + 0x24)) & 0xc0) == 0)) {
      iVar4 = 1;
    }
    bVar5 = iVar4 == iVar3;
    local_4 = 0xffffffff;
    FUN_00583cf0(local_24);
    local_2c = 0x28;
    local_28 = 0x2c;
    local_4 = 3;
    pvVar1 = (void *)FUN_004f6010(pvVar1,&local_2c,1);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return bVar5;
}

