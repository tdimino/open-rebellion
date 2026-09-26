
bool FUN_005429e0(int *param_1,undefined4 param_2,undefined4 param_3,int param_4,int param_5,
                 int *param_6)

{
  int *piVar1;
  bool bVar2;
  undefined3 extraout_var;
  int iVar3;
  int local_34 [2];
  undefined4 local_2c;
  undefined4 local_28;
  int local_24 [2];
  int local_1c [2];
  int local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar1 = param_6;
  puStack_8 = &LAB_00647130;
  pvStack_c = ExceptionList;
  local_2c = 1;
  local_28 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_6,&local_2c);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_004f4340(local_14);
  local_4 = 1;
  FUN_004f4340(local_1c);
  local_4._0_1_ = 2;
  FUN_004f4340(local_24);
  local_4._0_1_ = 3;
  FUN_004f4340(local_34);
  local_4._0_1_ = 4;
  bVar2 = FUN_0054bb90(param_1,param_4,param_5,local_14,local_1c,local_24,local_34,piVar1);
  bVar2 = CONCAT31(extraout_var,bVar2) != 0;
  FUN_004ece30(&param_6);
  local_4._0_1_ = 5;
  if (piVar1[1] == -1) {
    iVar3 = FUN_0054bf00((int)local_34,&param_6,piVar1);
    if ((iVar3 == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    if (piVar1[1] == -1) {
      iVar3 = FUN_0054c110(param_1,(int)local_34,piVar1);
      if ((iVar3 == 0) || (!bVar2)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
    }
  }
  local_4._0_1_ = 4;
  FUN_00619730();
  local_4._0_1_ = 3;
  FUN_004f4380(local_34);
  local_4._0_1_ = 2;
  FUN_004f4380(local_24);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_004f4380(local_1c);
  local_4 = 0xffffffff;
  FUN_004f4380(local_14);
  ExceptionList = pvStack_c;
  return bVar2;
}

