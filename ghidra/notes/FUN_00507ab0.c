
undefined4 __thiscall
FUN_00507ab0(void *this,uint *param_1,undefined4 param_2,undefined4 *param_3,undefined4 *param_4,
            int *param_5,void *param_6)

{
  bool bVar1;
  undefined4 uVar2;
  void *pvVar3;
  int iVar4;
  int iVar5;
  undefined4 uVar6;
  undefined4 local_54;
  undefined4 local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006416a0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_3 = 1;
  *param_4 = 1;
  *param_5 = 0;
  local_54 = 1;
  local_50 = 0xffffffff;
  local_4 = 0;
  FUN_00520580(param_6,&local_54);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_54 = 0x18;
  local_50 = 0x1c;
  if ((*param_1 < 0x18) || (0x1c < param_1[1])) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    iVar5 = 0;
    FUN_004ffe70(local_4c,this,param_2);
    local_4 = 1;
    FUN_00513120((int)local_4c);
    while (local_30 != 0) {
      uVar6 = param_2;
      uVar2 = FUN_0052bed0((int)local_4c);
      pvVar3 = FUN_005003d0(local_2c,uVar2,uVar6);
      local_4._0_1_ = 2;
      iVar4 = FUN_00513180((int)pvVar3);
      iVar5 = iVar5 + iVar4;
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_005004d0(local_2c);
      FUN_005130d0((int)local_4c);
    }
    *param_4 = 0;
    iVar4 = FUN_0040f340();
    local_54 = 0x90;
    *param_5 = iVar4 - iVar5;
    local_50 = 3;
    local_4._0_1_ = 3;
    FUN_00520580(param_6,&local_54);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_004fff70(local_4c);
  }
  ExceptionList = local_c;
  return 1;
}

