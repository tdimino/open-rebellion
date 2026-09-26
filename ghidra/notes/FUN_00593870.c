
bool __thiscall FUN_00593870(void *this,int param_1,int param_2,undefined4 param_3)

{
  bool bVar1;
  int iVar2;
  void *pvVar3;
  uint *puVar4;
  int *piVar5;
  void *pvVar6;
  undefined4 uVar7;
  bool bVar8;
  uint local_24;
  undefined4 local_20;
  undefined1 local_1c [4];
  undefined1 local_18 [4];
  uint local_14;
  undefined1 local_10 [4];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006509d8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_24);
  local_4 = 0;
  FUN_0042dbe0(&local_20);
  local_4 = CONCAT31(local_4._1_3_,1);
  iVar2 = FUN_00521300(param_1,&local_24,&local_20);
  if (iVar2 == 0) {
    bVar8 = false;
  }
  else {
    bVar8 = true;
    if ((*(byte *)((int)this + 0xa4) & 2) == 0) {
      iVar2 = FUN_004ece60(&local_24);
      bVar8 = iVar2 != 0;
    }
  }
  pvVar3 = (void *)thunk_FUN_005f5060(param_1);
  for (; pvVar3 != (void *)0x0; pvVar3 = *(void **)((int)pvVar3 + 0x10)) {
    uVar7 = 0;
    puVar4 = FUN_00403040(pvVar3,local_1c);
    local_4._0_1_ = 2;
    piVar5 = (int *)FUN_00505d40(puVar4);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
    iVar2 = FUN_004ece60(&local_24);
    if (iVar2 != 0) {
      puVar4 = FUN_00403040(pvVar3,local_18);
      local_14 = (uint)(*puVar4 != local_24);
      FUN_00619730();
      if (((local_14 != 0) && (piVar5 != (int *)0x0)) &&
         (iVar2 = (**(code **)(*piVar5 + 0x1d4))(), iVar2 == 0)) {
        uVar7 = 1;
      }
    }
    pvVar6 = FUN_00403040(pvVar3,local_10);
    local_4._0_1_ = 3;
    iVar2 = (**(code **)(*(int *)this + 0x1cc))(pvVar6,uVar7,param_3);
    if ((iVar2 == 0) || (bVar1 = true, !bVar8)) {
      bVar1 = false;
    }
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
    bVar8 = bVar1;
  }
  for (pvVar3 = (void *)thunk_FUN_005f5060(param_2); pvVar3 != (void *)0x0;
      pvVar3 = *(void **)((int)pvVar3 + 0x10)) {
    pvVar6 = FUN_00403040(pvVar3,&param_2);
    local_4._0_1_ = 4;
    iVar2 = (**(code **)(*(int *)this + 0x1cc))(pvVar6,1,param_3);
    if ((iVar2 == 0) || (bVar1 = !bVar8, bVar8 = true, bVar1)) {
      bVar8 = false;
    }
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return bVar8;
}

