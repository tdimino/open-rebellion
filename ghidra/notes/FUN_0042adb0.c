
void __fastcall FUN_0042adb0(int param_1)

{
  uint uVar1;
  void *pvVar2;
  void *pvVar3;
  uint *puVar4;
  int iVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  undefined4 uVar9;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c628;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00610c70(1);
  uVar1 = FUN_004fcee0();
  if (uVar1 < 2 != 0) {
    *(undefined4 *)(param_1 + 0xc0) = 8;
  }
  FUN_0042d650(*(int *)(param_1 + 0x9c),(uint)(uVar1 < 2));
  if (*(int *)(param_1 + 0x9c) == 1) {
    pvVar2 = (void *)FUN_00618b70(0x6c);
    pvVar3 = (void *)0x0;
    local_4 = 0;
    if (pvVar2 != (void *)0x0) {
      uVar9 = 0x43;
      iVar8 = 0x74;
      iVar7 = 0x43;
      iVar6 = 0x151;
      iVar5 = 0x21d;
      puVar4 = (uint *)FUN_005ff440(param_1);
      pvVar3 = FUN_00441a60(pvVar2,param_1,puVar4,iVar5,iVar6,iVar7,iVar8,uVar9);
    }
    local_4 = 0xffffffff;
    *(void **)(param_1 + 0x128) = pvVar3;
    pvVar2 = (void *)FUN_00618b70(0x58);
    local_4 = 1;
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_004727e0(pvVar2,1);
    }
    local_4 = 0xffffffff;
    FUN_005f5600(pvVar2,0xdead);
    (**(code **)(**(int **)(param_1 + 0x128) + 0x24))();
    (**(code **)(**(int **)(param_1 + 0x128) + 0x38))();
    FUN_00472b20(pvVar2,0x12,(undefined4 *)&stack0xffffffd8,*(undefined4 *)(param_1 + 0x128),0xf,
                 FUN_0041cea0);
    pvVar2 = (void *)FUN_00618b70(0x6c);
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      uVar9 = 0x43;
      iVar8 = 0x45;
      iVar7 = 0x2f;
      iVar6 = 0x19b;
      iVar5 = 0x13c;
      puVar4 = (uint *)FUN_005ff440(param_1);
      pvVar2 = FUN_00441a60(pvVar2,param_1,puVar4,iVar5,iVar6,iVar7,iVar8,uVar9);
    }
    *(void **)(param_1 + 300) = pvVar2;
    pvVar2 = (void *)FUN_00618b70(0x58);
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_004727e0(pvVar2,1);
    }
    FUN_005f5600(pvVar2,0xbeef);
    pvVar3 = pvVar2;
    (**(code **)(**(int **)(param_1 + 300) + 0x24))();
    (**(code **)(**(int **)(param_1 + 300) + 0x38))(0xbeef,&stack0xffffffd4,5,5);
    uVar9 = *(undefined4 *)(param_1 + 300);
  }
  else {
    pvVar2 = (void *)FUN_00618b70(0x6c);
    local_4 = 4;
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      uVar9 = 0x43;
      iVar8 = 0x85;
      iVar7 = 0x6b;
      iVar6 = 0x15b;
      iVar5 = 0;
      puVar4 = (uint *)FUN_005ff440(param_1);
      pvVar2 = FUN_00441a60(pvVar2,param_1,puVar4,iVar5,iVar6,iVar7,iVar8,uVar9);
    }
    local_4 = 0xffffffff;
    *(void **)(param_1 + 0x128) = pvVar2;
    pvVar2 = (void *)FUN_00618b70(0x6c);
    local_4 = 5;
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      uVar9 = 0x43;
      iVar8 = 0x4f;
      iVar7 = 0x65;
      iVar6 = 0x191;
      iVar5 = 0x12e;
      puVar4 = (uint *)FUN_005ff440(param_1);
      pvVar2 = FUN_00441a60(pvVar2,param_1,puVar4,iVar5,iVar6,iVar7,iVar8,uVar9);
    }
    local_4 = 0xffffffff;
    *(void **)(param_1 + 300) = pvVar2;
    pvVar2 = (void *)FUN_00618b70(0x58);
    local_4 = 6;
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_004727e0(pvVar2,1);
    }
    local_4 = 0xffffffff;
    FUN_005f5600(pvVar2,0xdead);
    (**(code **)(**(int **)(param_1 + 0x128) + 0x24))();
    (**(code **)(**(int **)(param_1 + 0x128) + 0x38))();
    FUN_00472b20(pvVar2,0x12,(undefined4 *)&stack0xffffffd8,*(undefined4 *)(param_1 + 0x128),0xf,
                 FUN_0041cea0);
    pvVar2 = (void *)FUN_00618b70(0x58);
    if (pvVar2 == (void *)0x0) {
      pvVar2 = (void *)0x0;
    }
    else {
      pvVar2 = FUN_004727e0(pvVar2,1);
    }
    FUN_005f5600(pvVar2,0xbeef);
    pvVar3 = pvVar2;
    (**(code **)(**(int **)(param_1 + 300) + 0x24))();
    (**(code **)(**(int **)(param_1 + 300) + 0x38))(0xbeef,&stack0xffffffd4,5,5);
    uVar9 = *(undefined4 *)(param_1 + 300);
  }
  FUN_00472b20(pvVar2,5,(undefined4 *)&stack0xffffffc0,uVar9,0xf,&LAB_0041cec0);
  (**(code **)(**(int **)(param_1 + 0x128) + 0x1c))();
  (**(code **)(**(int **)(param_1 + 300) + 0x1c))();
  (**(code **)(**(int **)(param_1 + 0x128) + 8))();
  (**(code **)(**(int **)(param_1 + 300) + 8))();
  ExceptionList = pvVar3;
  return;
}

