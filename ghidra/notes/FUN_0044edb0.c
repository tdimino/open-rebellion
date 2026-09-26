
void __thiscall FUN_0044edb0(void *this,int *param_1,uint *param_2,uint param_3)

{
  uint uVar1;
  int *piVar2;
  uint *puVar3;
  int iVar4;
  int iVar5;
  void *pvVar6;
  uint *puVar7;
  void *this_00;
  undefined *puVar8;
  char *pcVar9;
  int iVar10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar2 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062f948;
  local_c = ExceptionList;
  iVar10 = *(int *)(*(int *)((int)this + 0x120) + 0xd0);
  uVar1 = *(uint *)(*(int *)((int)this + 0x120) + 0xcc);
  if (param_1 == (int *)0x0) {
    ExceptionList = &local_c;
    FUN_004ece30(&param_1);
    local_4 = 4;
    param_2 = (uint *)FUN_00618b70(0x74);
    local_4._0_1_ = 5;
    if (param_2 == (uint *)0x0) {
      pvVar6 = (void *)0x0;
    }
    else {
      iVar10 = *(int *)((int)this + 0x184);
      *(int *)((int)this + 0x184) = iVar10 + 1;
      pvVar6 = FUN_00442130(param_2,iVar10,&param_1);
    }
    local_4 = CONCAT31(local_4._1_3_,4);
    if (param_3 != 0) {
      pcVar9 = (char *)FUN_00583c40(param_3);
      FUN_005f35e0((void *)((int)pvVar6 + 0x14),pcVar9);
      *(undefined4 *)((int)pvVar6 + 0x38) = 6;
      *(undefined4 *)((int)pvVar6 + 0x34) = 10;
      *(undefined4 *)((int)pvVar6 + 0x50) = 5;
    }
    FUN_005f59f0(*(void **)((int)this + 0x124),(int)pvVar6);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  else {
    ExceptionList = &local_c;
    puVar3 = FUN_004025b0(param_1,&param_3);
    local_4 = 0;
    puVar3 = FUN_0042c3b0(*(undefined4 *)((int)this + 0x178),puVar3,piVar2,
                          (uint)(param_2 == (uint *)0x0),1);
    local_4 = 0xffffffff;
    FUN_00619730();
    param_2 = (uint *)FUN_00618b70(0x28);
    local_4 = 1;
    if (param_2 == (uint *)0x0) {
      param_2 = (uint *)0x0;
    }
    else {
      param_2 = FUN_005fbda0(param_2,uVar1,iVar10,puVar3);
    }
    local_4 = 0xffffffff;
    FUN_005fcfe0(param_2,0,0,uVar1,iVar10,(char)puVar3[2]);
    iVar4 = FUN_005fc0e0((int *)puVar3);
    iVar5 = FUN_005fc0f0((int *)puVar3);
    iVar10 = (iVar10 - iVar5) / 2;
    FUN_005fd0f0(puVar3,param_2,(int)(uVar1 - iVar4) / 2,iVar10);
    pvVar6 = (void *)FUN_00618b70(0x74);
    local_4 = 2;
    if (pvVar6 == (void *)0x0) {
      this_00 = (void *)0x0;
    }
    else {
      puVar7 = FUN_004025b0(param_1,&param_3);
      iVar4 = *(int *)((int)this + 0x184);
      local_4 = CONCAT31(local_4._1_3_,3);
      *(int *)((int)this + 0x184) = iVar4 + 1;
      this_00 = FUN_00442130(pvVar6,iVar4,puVar7);
    }
    local_4 = 2;
    if (pvVar6 != (void *)0x0) {
      FUN_00619730();
    }
    local_4 = 0xffffffff;
    FUN_0060be60(this_00,(int)param_2,0);
    *(uint *)((int)this_00 + 0x3c) = *(uint *)((int)this_00 + 0x3c) | 4;
    puVar8 = FUN_004f62d0((int)param_1);
    pcVar9 = (char *)FUN_00583c40((int)puVar8);
    FUN_005f35e0((void *)((int)this_00 + 0x14),pcVar9);
    iVar4 = FUN_005fc0f0((int *)puVar3);
    *(undefined4 *)((int)this_00 + 0x38) = 10;
    *(int *)((int)this_00 + 0x34) = iVar4 + iVar10;
    *(undefined4 *)((int)this_00 + 0x50) = 1;
    FUN_005f59f0(*(void **)((int)this + 0x124),(int)this_00);
    if (puVar3 != (uint *)0x0) {
      FUN_005fbfa0(puVar3);
      FUN_00618b60((undefined *)puVar3);
    }
  }
  ExceptionList = local_c;
  return;
}

