
uint __thiscall
FUN_00551770(void *this,int param_1,int param_2,uint *param_3,int *param_4,void *param_5,
            uint *param_6)

{
  uint uVar1;
  undefined4 local_f8;
  undefined4 local_f4;
  undefined4 local_f0 [57];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00648c56;
  local_c = ExceptionList;
  uVar1 = 1;
  local_f8 = 1;
  local_f4 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_4,&local_f8);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_0054fb20((int)param_5);
  FUN_004ece80(param_6);
  FUN_00550100(local_f0);
  local_4 = 1;
  if (param_4[1] == -1) {
    uVar1 = FUN_00552a50(this,param_1,param_4,local_f0);
  }
  if (uVar1 != 0) {
    if (param_4[1] == -1) {
      uVar1 = FUN_00552a50(this,param_2,param_4,local_f0);
    }
    if (uVar1 != 0) {
      if (param_4[1] == -1) {
        uVar1 = FUN_00552150(this,param_1,param_4);
      }
      if (uVar1 != 0) {
        if (param_4[1] == -1) {
          uVar1 = FUN_00552150(this,param_2,param_4);
        }
        if (uVar1 != 0) {
          if (param_4[1] == -1) {
            uVar1 = FUN_00552300(this,(int)local_f0,1,param_3,param_4,param_5,param_6);
          }
          if ((uVar1 != 0) && (param_4[1] == -1)) {
            uVar1 = FUN_00552dd0(this,(int)local_f0,1,param_6);
          }
        }
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_00550170(local_f0);
  ExceptionList = local_c;
  return uVar1;
}

