
void __thiscall FUN_004a0e10(void *this,int *param_1,void *param_2,uint param_3)

{
  int *piVar1;
  uint *puVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  uint uVar7;
  void *pvVar8;
  uint *puVar9;
  void *this_00;
  undefined *puVar10;
  char *pcVar11;
  uint *this_01;
  undefined1 uVar12;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637688;
  local_c = ExceptionList;
  this_01 = (uint *)0x0;
  if (param_1 == (int *)0x0) {
    ExceptionList = &local_c;
    FUN_004ece30(&param_1);
    local_4 = 4;
    param_2 = (void *)FUN_00618b70(0x74);
    local_4._0_1_ = 5;
    if (param_2 == (void *)0x0) {
      pvVar8 = (void *)0x0;
    }
    else {
      pvVar8 = FUN_004421d0(param_2,(uint *)&param_1);
    }
    local_4 = CONCAT31(local_4._1_3_,4);
    if (param_3 != 0) {
      pcVar11 = (char *)FUN_00583c40(param_3);
      FUN_005f35e0((void *)((int)pvVar8 + 0x14),pcVar11);
      *(undefined4 *)((int)pvVar8 + 0x50) = 0x25;
      *(undefined4 *)((int)pvVar8 + 0x38) = 10;
      *(undefined4 *)((int)pvVar8 + 0x34) = 10;
    }
    FUN_005f59f0(*(void **)((int)this + 0x1bc),(int)pvVar8);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  else {
    ExceptionList = &local_c;
    puVar2 = FUN_004025b0(param_1,&param_3);
    local_4 = 0;
    puVar2 = FUN_0042c3b0(*(undefined4 *)((int)this + 0x1b0),puVar2,piVar1,
                          (uint)(param_2 == (void *)0x0),1);
    local_4 = 0xffffffff;
    FUN_00619730();
    param_2 = (void *)FUN_00618b70(0x28);
    local_4 = 1;
    if (param_2 != (void *)0x0) {
      this_01 = FUN_005fbda0(param_2,0x7a,0x2b,puVar2);
    }
    local_4 = 0xffffffff;
    iVar3 = FUN_005fc0e0((int *)puVar2);
    iVar4 = FUN_005fc0e0((int *)this_01);
    iVar5 = FUN_005fc0f0((int *)puVar2);
    iVar6 = FUN_005fc0f0((int *)this_01);
    uVar12 = (undefined1)puVar2[2];
    iVar6 = (iVar6 - iVar5) / 2;
    iVar5 = FUN_005fc0f0((int *)this_01);
    uVar7 = FUN_005fc0e0((int *)this_01);
    FUN_005fcfe0(this_01,0,0,uVar7,iVar5,uVar12);
    FUN_005fcc30(puVar2,this_01,(iVar4 - iVar3) / 2,iVar6,0,0,(undefined4 *)0x0,0);
    pvVar8 = (void *)FUN_00618b70(0x74);
    local_4 = 2;
    if (pvVar8 == (void *)0x0) {
      this_00 = (void *)0x0;
    }
    else {
      puVar9 = FUN_004025b0(param_1,(uint *)&param_2);
      local_4 = CONCAT31(local_4._1_3_,3);
      this_00 = FUN_004421d0(pvVar8,puVar9);
    }
    local_4 = 2;
    if (pvVar8 != (void *)0x0) {
      FUN_00619730();
    }
    local_4 = 0xffffffff;
    FUN_0060be60(this_00,(int)this_01,0);
    iVar3 = FUN_005fc0f0((int *)puVar2);
    *(int *)((int)this_00 + 0x34) = iVar3 + iVar6;
    *(uint *)((int)this_00 + 0x3c) = *(uint *)((int)this_00 + 0x3c) | 4;
    puVar10 = FUN_004f62d0((int)param_1);
    pcVar11 = (char *)FUN_00583c40((int)puVar10);
    FUN_005f35e0((void *)((int)this_00 + 0x14),pcVar11);
    *(undefined4 *)((int)this_00 + 0x38) = 10;
    *(undefined4 *)((int)this_00 + 0x50) = 0x21;
    *(char *)(*(int *)((int)this + 0x1c0) + 0xd4) = (char)puVar2[2];
    FUN_005f59f0(*(void **)((int)this + 0x1bc),(int)this_00);
    if (puVar2 != (uint *)0x0) {
      FUN_005fbfa0(puVar2);
      FUN_00618b60((undefined *)puVar2);
    }
  }
  ExceptionList = local_c;
  return;
}

