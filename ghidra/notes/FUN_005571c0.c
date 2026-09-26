
uint FUN_005571c0(int *param_1,void *param_2,int param_3,uint *param_4,int *param_5)

{
  void *this;
  int *this_00;
  int iVar1;
  uint *puVar2;
  uint *puVar3;
  uint uVar4;
  undefined4 local_54;
  undefined4 local_50;
  undefined4 local_4c [7];
  undefined4 local_30 [9];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006497d0;
  local_c = ExceptionList;
  uVar4 = 1;
  ExceptionList = &local_c;
  FUN_004ece80(param_4);
  this_00 = param_5;
  local_54 = 1;
  local_50 = 0xffffffff;
  local_4 = 0;
  FUN_00520580(param_5,&local_54);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_00551060(local_30);
  this = param_2;
  local_4._0_1_ = 1;
  local_4._1_3_ = 0;
  if (this_00[1] == -1) {
    param_2 = (void *)0x3;
    iVar1 = FUN_005511d0(local_30,param_1,(uint)param_1[9] >> 6 & 3,&param_2);
    uVar4 = 0;
    if (iVar1 != 0) {
      puVar2 = FUN_004025b0(this,(uint *)&param_5);
      local_4._0_1_ = 2;
      uVar4 = FUN_005513a0(local_30,puVar2);
      local_4._0_1_ = 1;
      FUN_00619730();
      if (uVar4 != 0) {
        if (param_3 == 0) {
          FUN_004ece30(&param_5);
          local_4 = CONCAT31(local_4._1_3_,3);
          iVar1 = FUN_004f6b70(param_1,(uint *)&param_5);
          uVar4 = 0;
          if (iVar1 != 0) {
            uVar4 = FUN_005515d0(local_30,(uint *)&param_5);
          }
          local_4._0_1_ = 1;
          FUN_00619730();
        }
        goto LAB_005572de;
      }
    }
  }
  else {
LAB_005572de:
    if (uVar4 != 0) {
      if (this_00[1] == -1) {
        FUN_0054fa40(local_4c);
        local_4._0_1_ = 4;
        puVar2 = FUN_004025b0(this,(uint *)&param_2);
        local_4._0_1_ = 5;
        puVar3 = FUN_004025b0(param_1,(uint *)&param_5);
        local_4._0_1_ = 6;
        uVar4 = FUN_00551630(local_30,puVar3,puVar2,this_00,local_4c,param_4);
        local_4._0_1_ = 5;
        FUN_00619730();
        local_4._0_1_ = 4;
        FUN_00619730();
        local_4._0_1_ = 1;
        FUN_0054fa90(local_4c);
      }
      if (uVar4 != 0) goto LAB_0055738c;
    }
  }
  local_54 = 1;
  local_50 = 1;
  local_4._0_1_ = 7;
  FUN_00520580(this_00,&local_54);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00619730();
LAB_0055738c:
  local_4 = 0xffffffff;
  FUN_005510b0(local_30);
  ExceptionList = local_c;
  return uVar4;
}

