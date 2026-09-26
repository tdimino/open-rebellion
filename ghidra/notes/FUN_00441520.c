
void __fastcall FUN_00441520(int param_1)

{
  bool bVar1;
  int *this;
  int iVar2;
  void *pvVar3;
  uint *puVar4;
  char *pcVar5;
  int iVar6;
  void *this_00;
  int iVar7;
  longlong lVar8;
  longlong lVar9;
  undefined4 *puVar10;
  int local_d4;
  int local_c8;
  undefined4 local_c0 [3];
  undefined2 local_b4;
  undefined2 local_b2;
  undefined2 local_b0;
  undefined2 local_ae;
  undefined4 local_ac;
  uint uStack_a8;
  undefined4 uStack_a4;
  uint local_a0;
  undefined4 local_9c [4];
  int local_8c;
  tagRECT local_88;
  undefined4 local_78 [3];
  undefined4 local_6c [3];
  undefined4 local_60;
  undefined4 local_5c [3];
  undefined4 local_50 [8];
  undefined4 local_30 [3];
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ea64;
  pvStack_c = ExceptionList;
  bVar1 = false;
  ExceptionList = &pvStack_c;
  FUN_005f2f50(local_c0);
  local_b2 = DAT_0065d424;
  local_4 = 0;
  local_b4 = 0x173b;
  FUN_005f2fc0(local_9c,&local_b4);
  pvVar3 = (void *)(param_1 + 0x260);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00601c60(pvVar3,5);
  FUN_00403e90(pvVar3,0);
  FUN_00601c90(pvVar3,0x200f0f0,(HWND)0x0);
  this = FUN_004f3900(*(int *)(param_1 + 0x9c));
  while (this != (int *)0x0) {
    FUN_00526560(this,&local_60);
    iVar2 = FUN_004067d0();
    if (iVar2 == 0) {
      local_b0 = 0x4da0;
      local_ae = DAT_0065d424;
      FUN_005f2fc0(local_6c,&local_b0);
      local_4._0_1_ = 4;
      iVar2 = FUN_005f34c0(local_6c);
      if (iVar2 == 0) {
        pvVar3 = FUN_004f6270(this,local_18);
        local_4._0_1_ = 7;
        FUN_005f3090(local_c0,(int)pvVar3);
        local_4._0_1_ = 4;
        FUN_005f2ff0(local_18);
        FUN_005f35b0(local_78,&DAT_006a8170);
        local_4._0_1_ = 8;
        FUN_005f30d0(local_c0,(int)local_78);
        local_4 = CONCAT31(local_4._1_3_,4);
        FUN_005f2ff0(local_78);
        FUN_005f30d0(local_c0,(int)local_9c);
      }
      else {
        FUN_005f3090(local_c0,(int)local_9c);
        FUN_005f35b0(local_5c,&DAT_006a8170);
        local_4._0_1_ = 5;
        FUN_005f30d0(local_c0,(int)local_5c);
        local_4._0_1_ = 4;
        FUN_005f2ff0(local_5c);
        pvVar3 = FUN_004f6270(this,local_30);
        local_4._0_1_ = 6;
        FUN_005f30d0(local_c0,(int)pvVar3);
        local_4 = CONCAT31(local_4._1_3_,4);
        FUN_005f2ff0(local_30);
      }
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_005f2ff0(local_6c);
    }
    else {
      pvVar3 = FUN_004f6270(this,local_24);
      local_4._0_1_ = 3;
      FUN_005f3090(local_c0,(int)pvVar3);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_005f2ff0(local_24);
      FUN_005f30d0(local_c0,(int)local_9c);
    }
    lVar8 = __ftol();
    lVar9 = __ftol();
    if (*(int *)(param_1 + 0x9c) == 1) {
      iVar2 = (int)lVar8 + 0x15;
      local_c8 = (int)lVar9 + 0x19;
    }
    else {
      iVar2 = (int)lVar8 + 0x54;
      local_c8 = (int)lVar9 + 0x1b;
    }
    local_8c = local_c8;
    pvVar3 = (void *)FUN_00618b70(0x74);
    local_4._0_1_ = 9;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      puVar4 = FUN_004025b0(this,&local_a0);
      bVar1 = true;
      local_4 = CONCAT31(local_4._1_3_,10);
      pvVar3 = FUN_004421d0(pvVar3,puVar4);
    }
    local_4 = 9;
    if (bVar1) {
      bVar1 = false;
      FUN_00619730();
    }
    local_4._0_1_ = 1;
    *(int *)((int)pvVar3 + 0x28) = iVar2;
    *(int *)((int)pvVar3 + 0x2c) = local_c8;
    pcVar5 = (char *)FUN_00583c40((int)local_c0);
    FUN_005f35e0((void *)((int)pvVar3 + 0x14),pcVar5);
    FUN_00512f90(local_50,this,4);
    local_4 = CONCAT31(local_4._1_3_,0xb);
    FUN_00513120((int)local_50);
    iVar6 = FUN_0052bed0((int)local_50);
    local_d4 = iVar2;
    iVar7 = local_c8;
    while (iVar6 != 0) {
      puVar10 = &local_ac;
      this_00 = (void *)FUN_0052bed0((int)local_50);
      FUN_00509620(this_00,puVar10);
      lVar8 = __ftol();
      lVar9 = __ftol();
      if (*(int *)(param_1 + 0x9c) == 1) {
        iVar6 = (int)lVar8 + 0x15;
        iVar7 = (int)lVar9 + 0x19;
      }
      else {
        iVar6 = (int)lVar8 + 0x54;
        iVar7 = (int)lVar9 + 0x1b;
      }
      if (local_d4 < iVar6) {
        local_d4 = iVar6;
      }
      if (local_c8 < iVar7) {
        local_c8 = iVar7;
      }
      FUN_005130d0((int)local_50);
      iVar6 = FUN_0052bed0((int)local_50);
      iVar7 = local_8c;
    }
    SetRect(&local_88,iVar2,iVar7,local_d4 + 8,local_c8 + 8);
    *(LONG *)((int)pvVar3 + 0x40) = local_88.left;
    *(LONG *)((int)pvVar3 + 0x44) = local_88.top;
    *(LONG *)((int)pvVar3 + 0x48) = local_88.right;
    *(LONG *)((int)pvVar3 + 0x4c) = local_88.bottom;
    FUN_005f59f0((void *)(param_1 + 0x24c),(int)pvVar3);
    local_4._0_1_ = 1;
    FUN_00513010(local_50);
    uStack_a8 = 0x80;
    uStack_a4 = 0x90;
    local_4._0_1_ = 2;
    this = (int *)FUN_004f6010(this,&uStack_a8,1);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
  }
  local_4 = local_4 & 0xffffff00;
  FUN_005f2ff0(local_9c);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_c0);
  ExceptionList = pvStack_c;
  return;
}

