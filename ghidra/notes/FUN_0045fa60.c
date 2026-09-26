
void __fastcall FUN_0045fa60(void *param_1)

{
  undefined4 *puVar1;
  bool bVar2;
  ushort uVar3;
  short sVar4;
  int *piVar5;
  undefined4 uVar6;
  void *pvVar7;
  HRSRC hResInfo;
  HGLOBAL hResData;
  char *pcVar8;
  int iVar9;
  uint local_28;
  void *local_24;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00630f8b;
  pvStack_c = ExceptionList;
  puVar1 = *(undefined4 **)((int)param_1 + 0x14c);
  ExceptionList = &pvStack_c;
  if (puVar1 != (undefined4 *)0x0) {
    ExceptionList = &pvStack_c;
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
    *(undefined4 *)((int)param_1 + 0x14c) = 0;
  }
  FUN_0042d170(*(void **)((int)param_1 + 0x148),&local_28);
  local_4 = 0;
  local_24 = (void *)0x90;
  local_20 = 0x98;
  if ((local_28 >> 0x18 < 0x90) || (0x97 < local_28 >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    piVar5 = FUN_004f3220(*(int *)(*(int *)((int)param_1 + 0x160) + 0x9c),&local_28);
    uVar6 = FUN_00509610((int)piVar5);
    uVar6 = FUN_0045f660(uVar6);
    sVar4 = (short)uVar6;
  }
  else {
    local_14 = 0x40;
    local_10 = 0x80;
    local_28 = local_28 >> 0x18;
    local_4 = CONCAT31(local_4._1_3_,1);
    if ((local_28 < 0x40) || (0x7f < local_28)) {
      local_1c = 8;
      local_18 = 0x10;
      if ((local_28 < 8) || (0xf < local_28)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (bVar2) goto LAB_0045fb79;
      bVar2 = false;
    }
    else {
LAB_0045fb79:
      bVar2 = true;
    }
    local_4 = local_4 & 0xffffff00;
    FUN_00619730();
    if (bVar2) {
      iVar9 = *(int *)(*(int *)((int)param_1 + 0x160) + 0x9c);
      uVar3 = (ushort)*(undefined4 *)(*(int *)((int)param_1 + 0x148) + 0xc) & 0xfff;
      if (iVar9 == 1) {
        iVar9 = 1;
      }
      else if (iVar9 == 2) {
        iVar9 = 2;
      }
      else {
        iVar9 = 0;
      }
      if (iVar9 == 2) {
        uVar6 = *(undefined4 *)((int)param_1 + 0x15c);
        sVar4 = uVar3 + 0x2000;
      }
      else {
        uVar6 = *(undefined4 *)((int)param_1 + 0x15c);
        sVar4 = uVar3 + 0x1000;
      }
      goto LAB_0045fbee;
    }
    sVar4 = (short)*(undefined4 *)(*(int *)((int)param_1 + 0x148) + 0xc);
  }
  uVar6 = *(undefined4 *)((int)param_1 + 0x15c);
LAB_0045fbee:
  pvVar7 = FUN_0045f970(sVar4,uVar6);
  *(void **)((int)param_1 + 0x14c) = pvVar7;
  if (*(HMODULE *)((int)param_1 + 0x158) != (HMODULE)0x0) {
    hResInfo = FindResourceA(*(HMODULE *)((int)param_1 + 0x158),
                             (LPCSTR)(*(uint *)(*(int *)((int)param_1 + 0x148) + 0xc) & 0xffff),
                             (LPCSTR)0xa);
    if (hResInfo != (HRSRC)0x0) {
      hResData = LoadResource(*(HMODULE *)((int)param_1 + 0x158),hResInfo);
      pcVar8 = LockResource(hResData);
      if (*(undefined4 **)((int)param_1 + 0x150) != (undefined4 *)0x0) {
        (**(code **)**(undefined4 **)((int)param_1 + 0x150))(1);
      }
      local_24 = (void *)FUN_00618b70(0xc);
      local_4._0_1_ = 2;
      if (local_24 == (void *)0x0) {
        pvVar7 = (void *)0x0;
      }
      else {
        pvVar7 = FUN_005f35b0(local_24,pcVar8);
      }
      local_4 = (uint)local_4._1_3_ << 8;
      *(void **)((int)param_1 + 0x150) = pvVar7;
      FreeResource(hResData);
      pvVar7 = *(void **)((int)param_1 + 0x13c);
      FUN_005f3090((void *)((int)pvVar7 + 0xa0),*(int *)((int)param_1 + 0x150));
      *(undefined4 *)((int)pvVar7 + 0x13c) = 0;
      FUN_0041fc30(pvVar7);
      (**(code **)(**(int **)((int)param_1 + 0x13c) + 4))(5);
      if (*(undefined4 **)((int)param_1 + 0x150) != (undefined4 *)0x0) {
        (**(code **)**(undefined4 **)((int)param_1 + 0x150))(1);
      }
      *(undefined4 *)((int)param_1 + 0x150) = 0;
    }
  }
  FUN_0045fd20(param_1,*(int **)((int)param_1 + 0x148));
  pcVar8 = (char *)FUN_00583c40(*(int *)((int)param_1 + 0x148) + 0x14);
  FUN_00601aa0(*(void **)((int)param_1 + 0x120),pcVar8);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

