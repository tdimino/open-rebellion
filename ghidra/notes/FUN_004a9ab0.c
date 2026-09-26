
void __thiscall FUN_004a9ab0(void *this,int *param_1,void *param_2)

{
  int *this_00;
  int *piVar1;
  void *pvVar2;
  uint *puVar3;
  int iVar4;
  int iVar5;
  uint *puVar6;
  undefined *puVar7;
  char *pcVar8;
  undefined4 *puVar9;
  uint local_14;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006381ff;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar2 = (void *)FUN_00618b70(0x74);
  this_00 = param_1;
  local_4 = 0;
  local_10 = pvVar2;
  if (pvVar2 == (void *)0x0) {
    param_1 = (int *)0x0;
  }
  else {
    puVar3 = FUN_004025b0(param_1,&local_14);
    local_4 = CONCAT31(local_4._1_3_,1);
    param_1 = FUN_004421d0(pvVar2,puVar3);
  }
  local_4 = 0;
  if (pvVar2 != (void *)0x0) {
    FUN_00619730();
  }
  local_4 = 0xffffffff;
  puVar3 = FUN_004025b0(this_00,&local_14);
  local_4 = 2;
  puVar3 = FUN_0042c3b0(*(undefined4 *)((int)this + 0x15c),puVar3,this_00,0,1);
  local_4 = 0xffffffff;
  FUN_00619730();
  pvVar2 = (void *)FUN_00618b70(0x28);
  local_4 = 3;
  local_10 = pvVar2;
  if (pvVar2 == (void *)0x0) {
    puVar6 = (uint *)0x0;
  }
  else {
    puVar6 = puVar3;
    iVar4 = FUN_005fc0f0((int *)puVar3);
    iVar5 = FUN_005fc0e0((int *)puVar3);
    puVar6 = FUN_005fbda0(pvVar2,iVar5,iVar4,puVar6);
  }
  local_4 = 0xffffffff;
  FUN_005fcc30(puVar3,puVar6,0,0,0,0,(undefined4 *)0x0,0);
  FUN_005fd0f0(param_2,puVar6,0,0);
  piVar1 = param_1;
  FUN_0060be60(param_1,(int)puVar3,(int)puVar6);
  puVar7 = FUN_004f62d0((int)this_00);
  pcVar8 = (char *)FUN_00583c40((int)puVar7);
  FUN_005f35e0(piVar1 + 5,pcVar8);
  *(char *)(*(int *)((int)this + 0x164) + 0xd4) = (char)puVar3[2];
  FUN_005f59f0(*(void **)((int)this + 0x168),(int)piVar1);
  puVar3 = FUN_004025b0(this_00,(uint *)&param_1);
  local_4 = 4;
  pvVar2 = FUN_004f5940((void *)((int)this + 0x114),puVar3);
  local_4 = 0xffffffff;
  FUN_00619730();
  if (pvVar2 == (void *)0x0) {
    pvVar2 = (void *)FUN_00618b70(0x28);
    local_4 = 5;
    param_2 = pvVar2;
    if (pvVar2 == (void *)0x0) {
      puVar9 = (undefined4 *)0x0;
    }
    else {
      puVar3 = FUN_004025b0(this_00,(uint *)&param_1);
      local_4 = CONCAT31(local_4._1_3_,6);
      puVar9 = FUN_004acd80(pvVar2,puVar3,0);
    }
    local_4 = 5;
    if (pvVar2 != (void *)0x0) {
      FUN_00619730();
    }
    local_4 = 0xffffffff;
    FUN_004acba0((void *)((int)this + 0x114),puVar9,0);
  }
  ExceptionList = local_c;
  return;
}

