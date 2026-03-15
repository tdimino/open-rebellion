
uint __thiscall
FUN_00551900(void *this,uint *param_1,uint *param_2,int *param_3,void *param_4,uint *param_5)

{
  uint uVar1;
  undefined4 local_f8;
  undefined4 local_f4;
  undefined4 local_f0 [57];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00648c76;
  local_c = ExceptionList;
  uVar1 = 1;
  local_f8 = 1;
  local_f4 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_f8);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_0054fb20((int)param_4);
  FUN_004ece80(param_5);
  FUN_00550100(local_f0);
  local_4 = 1;
  if (param_3[1] == -1) {
    uVar1 = FUN_005529a0(this,param_1,param_3,local_f0);
  }
  if (uVar1 != 0) {
    if (param_3[1] == -1) {
      uVar1 = FUN_00552300(this,(int)local_f0,1,param_2,param_3,param_4,param_5);
    }
    if ((uVar1 != 0) && (param_3[1] == -1)) {
      uVar1 = FUN_00552dd0(this,(int)local_f0,1,param_5);
    }
  }
  local_4 = 0xffffffff;
  FUN_00550170(local_f0);
  ExceptionList = local_c;
  return uVar1;
}

