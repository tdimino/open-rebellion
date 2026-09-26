
undefined4 __thiscall FUN_0040d510(void *this,int *param_1,undefined4 param_2)

{
  undefined4 uVar1;
  void *pvVar2;
  int iVar3;
  int iVar4;
  int *piVar5;
  uint *puVar6;
  int *piVar7;
  undefined4 local_2c [7];
  int iStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar5 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a2a0;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  pvVar2 = FUN_005039d0(local_2c,param_1,3);
  local_4 = 0;
  iVar3 = FUN_00513180((int)pvVar2);
  local_4 = 0xffffffff;
  FUN_00503ad0(local_2c);
  pvVar2 = FUN_005039d0(local_2c,piVar5,1);
  local_4 = 1;
  param_1 = (int *)FUN_00513180((int)pvVar2);
  local_4 = 0xffffffff;
  FUN_00503ad0(local_2c);
  iVar4 = (**(code **)(*piVar5 + 0x26c))();
  piVar7 = (int *)(iVar3 - iVar4);
  if (0 < (int)piVar7) {
    if (((int)param_1 < (int)piVar7) && (piVar7 = (int *)0x0, param_1 != (int *)0x0)) {
      piVar7 = param_1;
    }
    FUN_005039d0(local_2c,piVar5,1);
    local_4 = 2;
    FUN_00513120((int)local_2c);
    uVar1 = param_2;
    for (; (iStack_10 != 0 && (piVar7 != (int *)0x0)); piVar7 = (int *)((int)piVar7 + -1)) {
      piVar5 = (int *)FUN_0052bed0((int)local_2c);
      FUN_005130d0((int)local_2c);
      puVar6 = FUN_004025b0(*(void **)((int)this + 8),(uint *)&param_1);
      local_4._0_1_ = 3;
      (**(code **)(*piVar5 + 0xa8))(puVar6,uVar1);
      local_4 = CONCAT31(local_4._1_3_,2);
      FUN_00619730();
    }
    local_4 = 0xffffffff;
    FUN_00503ad0(local_2c);
  }
  ExceptionList = pvStack_c;
  return 1;
}

