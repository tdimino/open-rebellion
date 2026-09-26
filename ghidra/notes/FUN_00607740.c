
bool __thiscall
FUN_00607740(void *this,int *param_1,uint param_2,uint param_3,uint param_4,int *param_5,
            int *param_6,int *param_7,uint param_8,int *param_9)

{
  int iVar1;
  int *piVar2;
  void *pvVar3;
  int *piVar4;
  int iVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  int *this_00;
  int iVar9;
  int iVar10;
  undefined4 *puVar11;
  undefined1 uVar12;
  uint uVar13;
  undefined4 *local_14;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar2 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00656693;
  local_c = ExceptionList;
  this_00 = (int *)0x0;
  puVar11 = *(undefined4 **)((int)this + 0xa8);
  ExceptionList = &local_c;
  if (puVar11 != (undefined4 *)0x0) {
    ExceptionList = &local_c;
    FUN_005fbfa0(puVar11);
    FUN_00618b60((undefined *)puVar11);
  }
  pvVar3 = (void *)FUN_00618b70(0x28);
  local_4 = 0;
  if (pvVar3 == (void *)0x0) {
    piVar4 = (int *)0x0;
  }
  else {
    piVar4 = FUN_005fbda0(pvVar3,*(int *)((int)this + 0x38),*(int *)((int)this + 0x3c),
                          *(uint **)((int)this + 0xa4));
  }
  *(int **)((int)this + 0xa8) = piVar4;
  iVar1 = piVar4[2];
  local_4 = 0xffffffff;
  uVar12 = (char)iVar1;
  iVar5 = FUN_005fc0f0(piVar4);
  uVar6 = FUN_005fc0e0(*(int **)((int)this + 0xa8));
  FUN_005fcfe0(*(void **)((int)this + 0xa8),0,0,uVar6,iVar5,uVar12);
  pvVar3 = (void *)FUN_00618b70(0x28);
  local_4 = 1;
  if (pvVar3 == (void *)0x0) {
    local_14 = (undefined4 *)0x0;
  }
  else {
    local_14 = FUN_005fbd20(pvVar3,param_1,(uint)param_6,10);
  }
  local_4 = 0xffffffff;
  pvVar3 = (void *)FUN_00618b70(0x28);
  local_4 = 2;
  if (pvVar3 == (void *)0x0) {
    param_6 = (int *)0x0;
  }
  else {
    param_6 = FUN_005fbd20(pvVar3,param_1,param_2,10);
  }
  local_4 = 0xffffffff;
  pvVar3 = (void *)FUN_00618b70(0x28);
  local_4 = 3;
  if (pvVar3 != (void *)0x0) {
    this_00 = FUN_005fbd20(pvVar3,param_1,(uint)param_7,10);
  }
  local_4 = 0xffffffff;
  pvVar3 = (void *)FUN_00618b70(0x28);
  local_4 = 4;
  if (pvVar3 == (void *)0x0) {
    param_7 = (int *)0x0;
  }
  else {
    param_7 = FUN_005fbd20(pvVar3,param_1,(uint)param_5,10);
  }
  local_4 = 0xffffffff;
  pvVar3 = (void *)FUN_00618b70(0x28);
  local_4 = 5;
  if (pvVar3 == (void *)0x0) {
    piVar4 = (int *)0x0;
  }
  else {
    piVar4 = FUN_005fbd20(pvVar3,param_1,(uint)param_9,10);
  }
  local_4 = 0xffffffff;
  pvVar3 = (void *)FUN_00618b70(0x28);
  local_4 = 6;
  if (pvVar3 == (void *)0x0) {
    param_1 = (int *)0x0;
  }
  else {
    param_1 = FUN_005fbd20(pvVar3,param_1,param_3,10);
  }
  local_4 = 0xffffffff;
  pvVar3 = (void *)FUN_00618b70(0x28);
  local_4 = 7;
  if (pvVar3 == (void *)0x0) {
    param_5 = (int *)0x0;
  }
  else {
    param_5 = FUN_005fbd20(pvVar3,piVar2,param_8,10);
  }
  local_4 = 0xffffffff;
  pvVar3 = (void *)FUN_00618b70(0x28);
  local_4 = 8;
  if (pvVar3 == (void *)0x0) {
    param_9 = (int *)0x0;
  }
  else {
    param_9 = FUN_005fbd20(pvVar3,piVar2,param_4,10);
  }
  local_4 = 0xffffffff;
  if (*(uint **)((int)this + 0xa8) != (uint *)0x0) {
    FUN_005fcc30(local_14,*(uint **)((int)this + 0xa8),0,0,0,0,(undefined4 *)0x0,0);
    uVar13 = 0;
    puVar11 = (undefined4 *)0x0;
    uVar6 = 0;
    iVar10 = 0;
    iVar9 = 0;
    iVar5 = FUN_005fc0e0(*(int **)((int)this + 0xa8));
    iVar7 = FUN_005fc0e0(this_00);
    FUN_005fcc30(this_00,*(uint **)((int)this + 0xa8),iVar5 - iVar7,iVar9,iVar10,uVar6,puVar11,
                 uVar13);
    iVar5 = FUN_005fc0f0(*(int **)((int)this + 0xa8));
    iVar7 = FUN_005fc0f0(param_6);
    iVar9 = FUN_005fc0e0(this_00);
    iVar10 = FUN_005fc0e0(*(int **)((int)this + 0xa8));
    iVar8 = FUN_005fc0e0(this_00);
    if (iVar9 < iVar10 - iVar8) {
      do {
        uVar6 = FUN_005fc0f0(param_6);
        puVar11 = (undefined4 *)FUN_005fc0e0(param_6);
        FUN_005fcc30(param_6,*(uint **)((int)this + 0xa8),iVar9,0,0,0,puVar11,uVar6);
        uVar6 = FUN_005fc0f0(param_1);
        puVar11 = (undefined4 *)FUN_005fc0e0(param_1);
        FUN_005fcc30(param_1,*(uint **)((int)this + 0xa8),iVar9,iVar5 - iVar7,0,0,puVar11,uVar6);
        iVar10 = FUN_005fc0e0(param_6);
        iVar9 = iVar9 + iVar10;
        iVar10 = FUN_005fc0e0(*(int **)((int)this + 0xa8));
        iVar8 = FUN_005fc0e0(this_00);
      } while (iVar9 < iVar10 - iVar8);
    }
    iVar5 = *(int *)((int)this + 0x34);
    uVar6 = FUN_005fc0f0(param_5);
    puVar11 = (undefined4 *)FUN_005fc0e0(param_5);
    uVar13 = 0;
    iVar9 = 0;
    iVar7 = FUN_005fc0f0(param_5);
    FUN_005fcc30(param_5,*(uint **)((int)this + 0xa8),0,iVar5 - iVar7,iVar9,uVar13,puVar11,uVar6);
    iVar5 = *(int *)((int)this + 0x3c);
    iVar7 = *(int *)((int)this + 0x38);
    uVar6 = FUN_005fc0f0(piVar4);
    puVar11 = (undefined4 *)FUN_005fc0e0(piVar4);
    uVar13 = 0;
    iVar10 = 0;
    iVar9 = FUN_005fc0f0(piVar4);
    iVar5 = iVar5 - iVar9;
    iVar9 = FUN_005fc0e0(piVar4);
    FUN_005fcc30(piVar4,*(uint **)((int)this + 0xa8),iVar7 - iVar9,iVar5,iVar10,uVar13,puVar11,uVar6
                );
    iVar5 = FUN_005fc0e0(*(int **)((int)this + 0xa8));
    iVar7 = FUN_005fc0e0(piVar4);
    iVar9 = FUN_005fc0f0(piVar4);
    iVar10 = FUN_005fc0f0(*(int **)((int)this + 0xa8));
    iVar8 = FUN_005fc0f0(piVar4);
    if (iVar9 < iVar10 - iVar8) {
      do {
        uVar6 = FUN_005fc0f0(param_9);
        puVar11 = (undefined4 *)FUN_005fc0e0(param_9);
        FUN_005fcc30(param_9,*(uint **)((int)this + 0xa8),0,iVar9,0,0,puVar11,uVar6);
        uVar6 = FUN_005fc0f0(param_7);
        puVar11 = (undefined4 *)FUN_005fc0e0(param_7);
        FUN_005fcc30(param_7,*(uint **)((int)this + 0xa8),iVar5 - iVar7,iVar9,0,0,puVar11,uVar6);
        iVar10 = FUN_005fc0f0(piVar4);
        iVar9 = iVar9 + iVar10;
        iVar10 = FUN_005fc0f0(*(int **)((int)this + 0xa8));
        iVar8 = FUN_005fc0f0(piVar4);
      } while (iVar9 < iVar10 - iVar8);
    }
    *(char *)(*(int *)((int)this + 0xa8) + 8) = (char)iVar1;
    if (local_14 != (undefined4 *)0x0) {
      FUN_005fbfa0(local_14);
      FUN_00618b60((undefined *)local_14);
    }
    if (param_6 != (int *)0x0) {
      FUN_005fbfa0(param_6);
      FUN_00618b60((undefined *)param_6);
    }
    if (this_00 != (int *)0x0) {
      FUN_005fbfa0(this_00);
      FUN_00618b60((undefined *)this_00);
    }
    if (param_7 != (int *)0x0) {
      FUN_005fbfa0(param_7);
      FUN_00618b60((undefined *)param_7);
    }
    if (piVar4 != (int *)0x0) {
      FUN_005fbfa0(piVar4);
      FUN_00618b60((undefined *)piVar4);
    }
    if (param_1 != (int *)0x0) {
      FUN_005fbfa0(param_1);
      FUN_00618b60((undefined *)param_1);
    }
    if (param_5 != (int *)0x0) {
      FUN_005fbfa0(param_5);
      FUN_00618b60((undefined *)param_5);
    }
    if (param_9 != (int *)0x0) {
      FUN_005fbfa0(param_9);
      FUN_00618b60((undefined *)param_9);
    }
  }
  ExceptionList = local_c;
  return *(int *)((int)this + 0xa8) != 0;
}

