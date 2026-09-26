
uint __thiscall FUN_00552210(void *this,void *param_1,void *param_2)

{
  bool bVar1;
  undefined3 extraout_var;
  uint uVar2;
  undefined4 local_100 [2];
  undefined4 local_f8;
  undefined4 local_f4;
  undefined4 local_f0 [57];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00648d91;
  pvStack_c = ExceptionList;
  local_f8 = 1;
  local_f4 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_2,&local_f8);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_00550100(local_f0);
  local_4 = 1;
  FUN_004f4340(local_100);
  local_4 = CONCAT31(local_4._1_3_,2);
  bVar1 = FUN_005287f0(param_1,local_100);
  uVar2 = 0;
  if (CONCAT31(extraout_var,bVar1) != 0) {
    uVar2 = FUN_00552150(this,(int)local_100,param_2);
  }
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_004f4380(local_100);
  local_4 = 0xffffffff;
  FUN_00550170(local_f0);
  ExceptionList = pvStack_c;
  return uVar2;
}

