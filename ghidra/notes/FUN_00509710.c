
undefined4 __thiscall FUN_00509710(void *this,undefined4 param_1)

{
  bool bVar1;
  bool bVar2;
  void *pvVar3;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined4 local_2c [8];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006419e0;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  pvVar3 = FUN_004ffef0(local_2c,this,1,param_1);
  local_4 = 0;
  bVar1 = FUN_005131b0((int)pvVar3);
  local_4 = 0xffffffff;
  FUN_004fff70(local_2c);
  pvVar3 = FUN_00503a50(local_2c,this,1,param_1);
  local_4 = 1;
  bVar2 = FUN_005131b0((int)pvVar3);
  local_4 = 0xffffffff;
  FUN_00503ad0(local_2c);
  if ((CONCAT31(extraout_var,bVar1) == 0) && (CONCAT31(extraout_var_00,bVar2) == 0)) {
    ExceptionList = pvStack_c;
    return 0;
  }
  ExceptionList = pvStack_c;
  return 1;
}

