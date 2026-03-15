
undefined4
FUN_005f88d0(undefined4 param_1,char *param_2,undefined4 param_3,undefined4 param_4,
            undefined4 param_5)

{
  void *pvVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00655b5b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_00618b70(0x38);
  local_4 = 0;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_005f79b0(pvVar1,param_1,param_2,param_3,param_4,param_5);
  }
  local_4 = 0xffffffff;
  FUN_005f5440(&DAT_006be340,pvVar1);
  ExceptionList = local_c;
  return 1;
}

