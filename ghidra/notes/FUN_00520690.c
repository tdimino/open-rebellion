
undefined4 * __thiscall FUN_00520690(void *this,undefined4 param_1,undefined4 param_2)

{
  bool bVar1;
  undefined3 extraout_var;
  undefined4 *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064356b;
  local_c = ExceptionList;
  puVar2 = (undefined4 *)0x0;
  ExceptionList = &local_c;
  bVar1 = FUN_00520620(this,param_1);
  if (CONCAT31(extraout_var,bVar1) == 0) {
    puVar2 = (undefined4 *)FUN_00618b70(0x24);
    local_4 = 0;
    if (puVar2 == (undefined4 *)0x0) {
      puVar2 = (undefined4 *)0x0;
    }
    else {
      puVar2 = FUN_00582a00(puVar2);
    }
    local_4 = 0xffffffff;
    if (puVar2 != (undefined4 *)0x0) {
      puVar2[7] = param_2;
      puVar2[6] = param_1;
      FUN_005f6470(this,(int)puVar2);
    }
  }
  ExceptionList = local_c;
  return puVar2;
}

