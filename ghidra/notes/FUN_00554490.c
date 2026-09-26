
uint FUN_00554490(void *param_1,uint *param_2,void *param_3,int *param_4)

{
  uint *puVar1;
  void *this;
  int *this_00;
  uint uVar2;
  int iVar3;
  uint uVar4;
  uint *puVar5;
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  this_00 = param_4;
  puStack_8 = &LAB_00649298;
  local_c = ExceptionList;
  local_30 = 1;
  local_2c = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_4,&local_30);
  local_4 = 0xffffffff;
  FUN_00619730();
  this = param_3;
  puVar1 = param_2;
  uVar2 = FUN_00551270(param_3,param_2);
  FUN_004ece30(&param_4);
  local_4 = 1;
  uVar4 = 0;
  if (uVar2 != 0) {
    iVar3 = FUN_004f6b70(param_1,(uint *)&param_4);
    uVar4 = 0;
    if (iVar3 != 0) {
      uVar4 = FUN_00551270(this,(uint *)&param_4);
    }
  }
  FUN_0054fa40(local_28);
  local_4._0_1_ = 2;
  if (uVar4 != 0) {
    if (this_00[1] == -1) {
      puVar5 = FUN_004025b0(param_1,(uint *)&param_1);
      local_4._0_1_ = 3;
      uVar4 = FUN_00551b50(this,puVar5,puVar1,this_00,local_28);
      local_4._0_1_ = 2;
      FUN_00619730();
    }
    if (uVar4 != 0) goto LAB_005545a3;
  }
  local_30 = 1;
  local_2c = 1;
  local_4._0_1_ = 4;
  FUN_00520580(this_00,&local_30);
  local_4._0_1_ = 2;
  FUN_00619730();
LAB_005545a3:
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_0054fa90(local_28);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar4;
}

