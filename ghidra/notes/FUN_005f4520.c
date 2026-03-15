
undefined4 __thiscall FUN_005f4520(void *this,int param_1,undefined4 param_2)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006556a3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)FUN_00618b70(0x1c);
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    FUN_005f5270(puVar1);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_005f2f90(puVar1 + 3,param_1);
    *puVar1 = &PTR_FUN_0066dcb0;
    puVar1[6] = param_2;
  }
  local_4 = 0xffffffff;
  FUN_005f59f0(this,(int)puVar1);
  ExceptionList = local_c;
  return 1;
}

