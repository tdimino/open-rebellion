
uint __thiscall
FUN_005545e0(void *this,int *param_1,uint *param_2,uint *param_3,uint *param_4,int param_5,
            uint *param_6,int *param_7)

{
  uint uVar1;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006492b8;
  local_c = ExceptionList;
  uVar1 = 1;
  local_14 = 1;
  local_10 = 1;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_7,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  if ((param_7[1] != -1) && (param_5 != 0)) {
    uVar1 = FUN_00554740(param_1,param_4,(int *)param_6,param_7);
  }
  if (uVar1 != 0) {
    if (param_7[1] != -1) {
      uVar1 = FUN_00554ab0(param_1,param_3,param_6,param_7);
    }
    if (uVar1 != 0) {
      if (param_7[1] != -1) {
        uVar1 = FUN_00554e80(this,param_1,param_2,param_6,param_7);
      }
      if (uVar1 != 0) {
        if (param_7[1] != -1) {
          uVar1 = FUN_00554f80(this,param_1,param_2,param_6,param_7);
        }
        if (uVar1 != 0) {
          if (param_7[1] != -1) {
            uVar1 = FUN_005550e0(this,param_1,param_2,param_6,param_7);
          }
          if (uVar1 != 0) {
            if (param_7[1] != -1) {
              uVar1 = FUN_005551e0(this,param_1,param_2,param_6,param_7);
            }
            if ((uVar1 != 0) && (param_7[1] != -1)) {
              uVar1 = FUN_005552e0(this,param_1,param_2,param_6,param_7);
            }
          }
        }
      }
    }
  }
  ExceptionList = local_c;
  return uVar1;
}

