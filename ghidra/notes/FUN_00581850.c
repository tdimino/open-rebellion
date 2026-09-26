
uint __thiscall FUN_00581850(void *this,void *param_1)

{
  uint uVar1;
  int iVar2;
  int *piVar3;
  void *pvVar4;
  int local_84 [6];
  undefined4 local_6c [4];
  int local_5c [20];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064ecd9;
  pvStack_c = ExceptionList;
  local_84[0] = 1;
  local_84[1] = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_1,local_84);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_00541e70(local_6c);
  local_4 = 1;
  FUN_0051fcb0(this,local_6c);
  uVar1 = FUN_005201c0(this,param_1);
  if (uVar1 != 0) {
    if (*(int *)((int)param_1 + 4) == -1) {
      FUN_00563090(local_5c);
      local_4._0_1_ = 2;
      FUN_00541e70(local_84 + 2);
      local_4._0_1_ = 3;
      FUN_0051fcb0(this,local_84 + 2);
      FUN_0054f3e0(local_5c,local_84 + 2);
      FUN_004f4340(local_84);
      piVar3 = local_84;
      local_4 = CONCAT31(local_4._1_3_,4);
      pvVar4 = param_1;
      iVar2 = (**(code **)(*(int *)this + 0x28))();
      uVar1 = FUN_005187f0((uint)(local_84 + 2),iVar2,piVar3,pvVar4);
      if ((uVar1 != 0) && (*(int *)((int)param_1 + 4) == -1)) {
        uVar1 = FUN_005535b0(local_5c,(int)local_84,*(int **)((int)this + 0x44),param_1);
      }
      local_4._0_1_ = 3;
      FUN_004f4380(local_84);
      local_4._0_1_ = 2;
      FUN_00541ea0();
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_005630d0(local_5c);
    }
    if (uVar1 != 0) goto LAB_005819d3;
  }
  local_84[0] = 1;
  local_84[1] = 1;
  local_4._0_1_ = 5;
  FUN_00520580(param_1,local_84);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00619730();
LAB_005819d3:
  local_4 = 0xffffffff;
  FUN_00541ea0();
  ExceptionList = pvStack_c;
  return uVar1;
}

