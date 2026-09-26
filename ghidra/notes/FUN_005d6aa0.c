
undefined4
FUN_005d6aa0(undefined4 *param_1,char *param_2,char *param_3,undefined4 *param_4,undefined4 *param_5
            ,void *param_6)

{
  void *this;
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654a2b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this = (void *)FUN_00618b70(0x14c);
  local_4 = 0;
  if (this == (void *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_005eeed0(this,param_1,param_2,param_3,param_4,param_5);
  }
  local_4 = 0xffffffff;
  if ((DAT_006bcbb0 & puVar1[0x45]) == 0) {
    if (puVar1 != (undefined4 *)0x0) {
      (**(code **)*puVar1)(1,this);
    }
  }
  else {
    FUN_005f4f10(param_6,(int)puVar1);
  }
  ExceptionList = local_c;
  return 1;
}

