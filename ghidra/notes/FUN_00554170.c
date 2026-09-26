
uint FUN_00554170(int *param_1,void *param_2)

{
  int *this;
  void *this_00;
  int iVar1;
  uint uVar2;
  uint *puVar3;
  undefined4 local_58;
  undefined4 local_54;
  undefined4 local_50;
  undefined4 local_4c [7];
  undefined4 local_30 [9];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_2;
  puStack_8 = &LAB_00649220;
  local_c = ExceptionList;
  local_54 = 1;
  local_50 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,&local_54);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_00551060(local_30);
  this = param_1;
  local_4 = 1;
  local_58 = 3;
  iVar1 = FUN_005511d0(local_30,param_1,(uint)param_1[9] >> 6 & 3,&local_58);
  FUN_004ece30(&param_2);
  local_4._0_1_ = 2;
  uVar2 = 0;
  if (iVar1 != 0) {
    iVar1 = FUN_004f6b70(this,(uint *)&param_2);
    uVar2 = 0;
    if (iVar1 != 0) {
      uVar2 = FUN_00551270(local_30,(uint *)&param_2);
    }
  }
  FUN_0054fa40(local_4c);
  local_4._0_1_ = 3;
  if (uVar2 != 0) {
    if (*(int *)((int)this_00 + 4) == -1) {
      puVar3 = FUN_004025b0(this,(uint *)&param_1);
      local_4._0_1_ = 4;
      uVar2 = FUN_00551c80(local_30,puVar3,(uint *)&param_2,this_00,local_4c);
      local_4._0_1_ = 3;
      FUN_00619730();
    }
    if (uVar2 != 0) goto LAB_005542a4;
  }
  local_54 = 1;
  local_50 = 1;
  local_4._0_1_ = 5;
  FUN_00520580(this_00,&local_54);
  local_4._0_1_ = 3;
  FUN_00619730();
LAB_005542a4:
  local_4._0_1_ = 2;
  FUN_0054fa90(local_4c);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_005510b0(local_30);
  ExceptionList = local_c;
  return uVar2;
}

