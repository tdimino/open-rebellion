
void __fastcall FUN_004a2200(int *param_1)

{
  undefined4 *puVar1;
  int *piVar2;
  undefined4 uVar3;
  uint uVar4;
  void *pvVar5;
  int iVar6;
  int iVar7;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637882;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  piVar2 = FUN_004a25c0(param_1);
  if (piVar2 == (int *)0x0) {
    ExceptionList = local_c;
    return;
  }
  uVar3 = FUN_006037f0(7);
  puVar1 = (undefined4 *)param_1[0x5a];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar1 = (undefined4 *)param_1[0x5b];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  param_1[0x5a] = 0;
  param_1[0x5b] = 0;
  uVar4 = FUN_004a1f60((int)piVar2,*(int *)(param_1[0x53] + 0x9c));
  if (uVar4 == 1) {
    pvVar5 = (void *)FUN_00618b70(0x28);
    local_4 = 0;
    if (pvVar5 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_005fbd20(pvVar5,uVar3,0x283b,10);
    }
    local_4 = 0xffffffff;
    param_1[0x5b] = (int)pvVar5;
    pvVar5 = (void *)FUN_00618b70(0x28);
    local_4 = 1;
    if (pvVar5 != (void *)0x0) {
      pvVar5 = FUN_005fbd20(pvVar5,uVar3,0x27d8,10);
      goto LAB_004a23cf;
    }
  }
  else if (uVar4 == 2) {
    pvVar5 = (void *)FUN_00618b70(0x28);
    local_4 = 2;
    if (pvVar5 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_005fbd20(pvVar5,uVar3,0x27d9,10);
    }
    local_4 = 0xffffffff;
    param_1[0x5b] = (int)pvVar5;
    pvVar5 = (void *)FUN_00618b70(0x28);
    local_4 = 3;
    if (pvVar5 != (void *)0x0) {
      pvVar5 = FUN_005fbd20(pvVar5,uVar3,0x283e,10);
      goto LAB_004a23cf;
    }
  }
  else {
    pvVar5 = (void *)FUN_00618b70(0x28);
    local_4 = 4;
    if (pvVar5 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_005fbd20(pvVar5,uVar3,0x283f,10);
    }
    local_4 = 0xffffffff;
    param_1[0x5b] = (int)pvVar5;
    pvVar5 = (void *)FUN_00618b70(0x28);
    local_4 = 5;
    if (pvVar5 != (void *)0x0) {
      pvVar5 = FUN_005fbd20(pvVar5,uVar3,0x2840,10);
      goto LAB_004a23cf;
    }
  }
  pvVar5 = (void *)0x0;
LAB_004a23cf:
  local_4 = 0xffffffff;
  param_1[0x5a] = (int)pvVar5;
  iVar6 = FUN_005fc0f0((int *)param_1[0x5b]);
  iVar6 = iVar6 + 2;
  iVar7 = FUN_005fc0e0((int *)param_1[0x5a]);
  SetRect((LPRECT)(param_1 + 0x4a),2,2,iVar7 + 2,iVar6);
  InvalidateRect((HWND)param_1[6],(LPRECT)(param_1 + 0x4a),0);
  ExceptionList = local_c;
  return;
}

